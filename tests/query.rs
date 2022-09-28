use indexmap::{indexmap, IndexMap};
use json_api::{
    query::{self, Direction, Query},
    Error,
};

type Mapping = IndexMap<&'static str, Query>;

fn from_mapping() -> Result<Mapping, Error> {
    Ok(indexmap! {
        "" => Default::default(),
        "fields[articles]=title" => Query::builder()
            .fields("articles", vec!["title"])
            .build()?,
        concat!(
            "fields[articles]=body%2Ctitle%2Cpublished-at&",
            "fields[comments]=body&",
            "fields[users]=name",
        ) => Query::builder()
            .fields("articles", vec!["body", "title", "published-at"])
            .fields("comments", vec!["body"])
            .fields("users", vec!["name"])
            .build()?,
        "filter[users.name]=Alfred+Pennyworth" => Query::builder()
            .filter("users.name", "Alfred Pennyworth")
            .build()?,
        "include=author" => Query::builder()
            .include("author")
            .build()?,
        "include=author%2Ccomments%2Ccomments.author" => Query::builder()
            .include("author")
            .include("comments")
            .include("comments.author")
            .build()?,
        "page[number]=0" => Query::builder()
            .page(1, None)
            .build()?,
        "page[number]=1" => Query::builder()
            .page(1, None)
            .build()?,
        "page[size]=10" => Query::builder()
            .page(1, Some(10))
            .build()?,
        "page[number]=2&page[size]=15" => Query::builder()
            .page(2, Some(15))
            .build()?,
        "sort=-published-at" => Query::builder()
            .sort("published-at", Direction::Desc)
            .build()?,
        "sort=published-at%2C-title" => Query::builder()
            .sort("published-at", Direction::Asc)
            .sort("title", Direction::Desc)
            .build()?,
        "sort=published-at%2C-title%2C-author.name" => Query::builder()
            .sort("published-at", Direction::Asc)
            .sort("title", Direction::Desc)
            .sort("author.name", Direction::Desc)
            .build()?,
        concat!(
            "fields[articles]=body%2Ctitle%2Cpublished-at&",
            "fields[comments]=body&",
            "fields[users]=name&",
            "filter[users.name]=Alfred+Pennyworth&",
            "include=author%2Ccomments%2Ccomments.author&",
            "page[number]=2&page[size]=15&",
            "sort=published-at%2C-title%2C-author.name",
        ) => Query::builder()
            .fields("articles", vec!["body", "title", "published-at"])
            .fields("comments", vec!["body"])
            .fields("users", vec!["name"])
            .filter("users.name", "Alfred Pennyworth")
            .include("author")
            .include("comments")
            .include("comments.author")
            .page(2, Some(15))
            .sort("published-at", Direction::Asc)
            .sort("title", Direction::Desc)
            .sort("author.name", Direction::Desc)
            .build()?,
    })
}

fn to_mapping() -> Result<Mapping, Error> {
    let mapping = from_mapping()?
        .into_iter()
        .map(|(key, value)| match key {
            "page[number]=0" | "page[number]=1" => ("", value),
            _ => (key, value),
        })
        .collect();

    Ok(mapping)
}

#[test]
fn query_from_slice() {
    for (source, expected) in from_mapping().unwrap() {
        let actual = query::from_slice(source.as_bytes()).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn query_from_str() {
    for (source, expected) in from_mapping().unwrap() {
        let actual = query::from_str(source).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn query_to_string() {
    for (expected, source) in to_mapping().unwrap() {
        let actual = query::to_string(&source).unwrap();
        assert_eq!(actual, expected);
    }
}

#[test]
fn query_to_vec() {
    for (expected, source) in to_mapping().unwrap() {
        let actual = query::to_vec(&source).unwrap();
        assert_eq!(actual, expected.to_owned().into_bytes());
    }
}
