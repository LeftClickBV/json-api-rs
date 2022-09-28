use std::ops::{Deref, DerefMut};

use serde::de::DeserializeOwned;

use json_api::{
    self,
    doc::{NewObject, Object},
    query::{Page, Query as JsonApiQuery, Sort},
    value::{
        collections::{map, set, Set},
        Key, Path, Value,
    },
    Error,
};
use rocket::{
    data::{self, ByteUnit, Data, FromData},
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest, Request},
};

// FIXME: Is this a good  limit?
const DATA_LIMIT: ByteUnit = ByteUnit::Mebibyte(10);

#[derive(Debug)]
pub struct Create<T: DeserializeOwned>(pub T);

impl<T: DeserializeOwned> Create<T> {
    /// Consumes the `Create` wrapper and returns the wrapped value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: DeserializeOwned> Deref for Create<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DeserializeOwned> DerefMut for Create<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[rocket::async_trait]
impl<'r, T: DeserializeOwned> FromData<'r> for Create<T> {
    type Error = Error;

    async fn from_data(_: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        let string = match data.open(DATA_LIMIT).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return fail(Error::payload_too_large(DATA_LIMIT.as_u64())),
            Err(e) => return fail(e.into()),
        };

        match json_api::from_str::<NewObject, _>(&string) {
            Ok(value) => Outcome::Success(Create(value)),
            Err(e) => fail(e),
        }
    }
}

#[derive(Debug)]
pub struct Update<T: DeserializeOwned>(pub T);

impl<T: DeserializeOwned> Update<T> {
    /// Consumes the `Update` wrapper and returns the wrapped value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: DeserializeOwned> Deref for Update<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DeserializeOwned> DerefMut for Update<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[rocket::async_trait]
impl<'r, T: DeserializeOwned> FromData<'r> for Update<T> {
    type Error = Error;

    async fn from_data(_: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        let string = match data.open(DATA_LIMIT).into_string().await {
            Ok(string) if string.is_complete() => string.into_inner(),
            Ok(_) => return fail(Error::payload_too_large(DATA_LIMIT.as_u64())),
            Err(e) => return fail(e.into()),
        };

        match json_api::from_str::<Object, _>(&string) {
            Ok(value) => Outcome::Success(Update(value)),
            Err(e) => fail(e),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Query {
    inner: JsonApiQuery,
}

impl Query {
    /// Consumes the [`Query`] wrapper and returns the wrapped value.
    ///
    /// [`Query`]: ./struct.Query.html
    pub fn into_inner(self) -> JsonApiQuery {
        self.inner
    }

    pub fn fields(&self) -> map::Iter<Key, Set> {
        self.inner.fields.iter()
    }

    pub fn filter(&self) -> map::Iter<Path, Value> {
        self.inner.filter.iter()
    }

    pub fn include(&self) -> set::Iter<Path> {
        self.inner.include.iter()
    }

    pub fn page(&self) -> Option<Page> {
        self.inner.page
    }

    pub fn sort(&self) -> set::Iter<Sort> {
        self.inner.sort.iter()
    }
}

impl Deref for Query {
    type Target = JsonApiQuery;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Query {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Query {
    type Error = Error;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // FIXME: It may not be necessary to go from `rocket::http::ury::Query` to `JsonApiQuery`
        // via `&str`
        let request_query = req.uri().query();
        match request_query.map(|query| json_api::query::from_str(query.as_str())) {
            Some(Ok(inner)) => Outcome::Success(Query { inner }),
            Some(Err(e)) => fail(e),
            None => Outcome::Success(Default::default()),
        }
    }
}

fn fail<T, F>(e: Error) -> Outcome<T, (Status, Error), F> {
    // use config::ROCKET_ENV;
    //
    // if !ROCKET_ENV.is_prod() {
    //     eprintln!("{:?}", e);
    // }

    Outcome::Failure((Status::BadRequest, e))
}
