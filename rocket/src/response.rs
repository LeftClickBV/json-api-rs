use std::io::Cursor;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use json_api::{self, doc::Object, Error, Resource};
use rocket::{
    http::Status,
    request::Request,
    response::{Responder, Response},
};

#[derive(Debug)]
pub struct Collection<T: Resource>(pub Vec<T>);

impl<T: Resource> Collection<T> {
    /// Consumes the [`Collection`] wrapper and returns the wrapped value.
    ///
    /// [`Collection`]: ./struct.Collection.html
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<T: Resource> Deref for Collection<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Resource> DerefMut for Collection<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Resource> FromIterator<T> for Collection<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Collection(Vec::from_iter(iter))
    }
}

impl<'r, T: Resource> Responder<'r, 'static> for Collection<T> {
    fn respond_to(self, request: &'r Request<'_>) -> Result<Response<'static>, Status> {
        let request_query = request.uri().query();
        let query = match request_query.map(|query| json_api::query::from_str(query.as_str())) {
            Some(Ok(query)) => Some(query),
            Some(Err(_)) => None,
            None => Some(Default::default()),
        };

        json_api::to_vec::<_, Object>(&*self, query.as_ref())
            .map(with_body)
            .or_else(fail)
    }
}

#[derive(Debug)]
pub struct Created<T: Resource>(pub T);

impl<T: Resource> Created<T> {
    /// Consumes the [`Created`] wrapper and returns the wrapped value.
    ///
    /// [`Created`]: ./struct.Created.html
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Resource> Deref for Created<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Resource> DerefMut for Created<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'r, T: Resource> Responder<'r, 'static> for Created<T> {
    fn respond_to(self, request: &Request) -> Result<Response<'static>, Status> {
        let request_query = request.uri().query();
        let query = match request_query.map(|query| json_api::query::from_str(query.as_str())) {
            Some(Ok(query)) => Some(query),
            Some(Err(_)) => None,
            None => Some(Default::default()),
        };

        json_api::to_vec::<_, Object>(&*self, query.as_ref())
            .map(with_body)
            .or_else(fail)
            .map(|mut resp| {
                resp.set_status(Status::Created);
                resp
            })
    }
}

#[derive(Debug)]
pub struct Member<T>(pub T);

impl<T: Resource> Member<T> {
    /// Consumes the [`Member`] wrapper and returns the wrapped value.
    ///
    /// [`Member`]: ./struct.Member.html
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Resource> Deref for Member<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Resource> DerefMut for Member<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'r, T: Resource> Responder<'r, 'static> for Member<T> {
    fn respond_to(self, request: &Request) -> Result<Response<'static>, Status> {
        let request_query = request.uri().query();
        let query = match request_query.map(|query| json_api::query::from_str(query.as_str())) {
            Some(Ok(query)) => Some(query),
            Some(Err(_)) => None,
            None => Some(Default::default()),
        };

        json_api::to_vec::<_, Object>(&*self, query.as_ref())
            .map(with_body)
            .or_else(fail)
    }
}

pub(crate) fn with_body(body: Vec<u8>) -> Response<'static> {
    Response::build()
        .raw_header("Content-Type", "application/vnd.api+json")
        .sized_body(None, Cursor::new(body))
        .finalize()
}

pub(crate) fn fail(_e: Error) -> Result<Response<'static>, Status> {
    // use config::ROCKET_ENV;
    //
    // if !ROCKET_ENV.is_prod() {
    //     eprintln!("{:?}", e);
    // }

    Err(Status::InternalServerError)
}
