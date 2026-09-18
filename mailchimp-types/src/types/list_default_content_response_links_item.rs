pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// This object represents a link from the resource where it is found to another resource or action that may be performed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDefaultContentResponseLinksItem {
    /// This property contains a fully-qualified URL that can be called to retrieve the linked resource or perform the linked action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The HTTP method that should be used when accessing the URL defined in 'href'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<ListDefaultContentResponseLinksItemMethod>,
    /// As with an HTML 'rel' attribute, this describes the type of link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    /// For HTTP methods that can receive bodies (POST and PUT), this is a URL representing the schema that the body should conform to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// For GETs, this is a URL representing the schema that the response should conform to.
    #[serde(rename = "targetSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_schema: Option<String>,
}

impl ListDefaultContentResponseLinksItem {
    pub fn builder() -> ListDefaultContentResponseLinksItemBuilder {
        <ListDefaultContentResponseLinksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDefaultContentResponseLinksItemBuilder {
    href: Option<String>,
    method: Option<ListDefaultContentResponseLinksItemMethod>,
    rel: Option<String>,
    schema: Option<String>,
    target_schema: Option<String>,
}

impl ListDefaultContentResponseLinksItemBuilder {
    pub fn href(mut self, value: impl Into<String>) -> Self {
        self.href = Some(value.into());
        self
    }

    pub fn method(mut self, value: ListDefaultContentResponseLinksItemMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn rel(mut self, value: impl Into<String>) -> Self {
        self.rel = Some(value.into());
        self
    }

    pub fn schema(mut self, value: impl Into<String>) -> Self {
        self.schema = Some(value.into());
        self
    }

    pub fn target_schema(mut self, value: impl Into<String>) -> Self {
        self.target_schema = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListDefaultContentResponseLinksItem`].
    pub fn build(self) -> Result<ListDefaultContentResponseLinksItem, BuildError> {
        Ok(ListDefaultContentResponseLinksItem {
            href: self.href,
            method: self.method,
            rel: self.rel,
            schema: self.schema,
            target_schema: self.target_schema,
        })
    }
}
