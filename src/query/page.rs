use std::fmt::{self, Formatter};

use serde::{
    de::Deserializer,
    ser::{Serialize, SerializeStruct, Serializer},
    Deserialize,
};

/// Limit and offset based pagination parameters.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Page {
    /// The page number. This value is checked to be non-zero when a page is created via
    /// the constructor method or decoded from a query string. If zero is passed to the
    /// constructor or decoded from a query string, `1` will be used instead.
    pub number: u64,

    /// Optionally specifies the maximum number of items to include per page.
    pub size: Option<u64>,
}

impl Page {
    /// Returns a new `Page`. If zero is used for `number` it will be treated as `1`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate json_api;
    /// #
    /// # fn main() {
    /// use json_api::query::Page;
    /// assert_eq!(Page::new(1, None), Page::default());
    /// # }
    /// ```
    pub fn new(number: u64, size: Option<u64>) -> Self {
        let number = if number > 0 { number } else { 1 };

        Page { number, size }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::new(1, None)
    }
}

impl<'de> Deserialize<'de> for Page {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};

        const FIELDS: &[&str] = &["number", "size"];

        #[derive(Debug, Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Number,
            Size,
        }

        struct PageVisitor;

        impl<'de> Visitor<'de> for PageVisitor {
            type Value = Page;

            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "an object containing json api pagination parameters")
            }

            fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut number = None;
                let mut size = None;

                while let Some(key) = access.next_key()? {
                    match key {
                        Field::Number => {
                            number = access.next_value()?;
                        }
                        Field::Size => {
                            size = access.next_value()?;
                        }
                    }
                }

                Ok(Page::new(number.unwrap_or(1), size))
            }
        }

        deserializer.deserialize_struct("Page", FIELDS, PageVisitor)
    }
}

impl Serialize for Page {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Page", 2)?;
        let number = &self.number;
        let size = &self.size;

        if *number != 1 {
            state.serialize_field("number", number)?;
        }

        if let Some(ref value) = *size {
            state.serialize_field("size", value)?;
        }

        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::Page;

    #[test]
    fn page_new() {
        let mut page = Page::new(0, None);

        // Page number should always be a positive unsigned integer.
        // If 0 is passed to the constructor, it should be treated as 1.
        assert_eq!(page.number, 1);
        assert_eq!(page.size, None);

        for number in 1..5 {
            page = Page::new(number, None);

            assert_eq!(page.number, number);
            assert_eq!(page.size, None);
        }

        for size in (0..10).map(Some) {
            page = Page::new(1, size);

            assert_eq!(page.number, 1);
            assert_eq!(page.size, size);
        }
    }
}
