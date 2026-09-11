pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific segment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct List {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListLinksItem>>,
    /// The date and time the segment was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The unique id for the segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The number of active subscribers currently included in the segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i64>,
    /// The name of the segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The conditions of the segment. Static segments (tags) and fuzzy segments don't have conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ListOptions>,
    /// The type of segment. Static segments are now known as tags. Learn more about [tags](https://mailchimp.com/help/getting-started-tags?utm_source=mc-api&utm_medium=docs&utm_campaign=apidocs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListType>,
    /// The date and time the segment was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl List {
    pub fn builder() -> ListBuilder {
        <ListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBuilder {
    links: Option<Vec<ListLinksItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<i64>,
    list_id: Option<String>,
    member_count: Option<i64>,
    name: Option<String>,
    options: Option<ListOptions>,
    r#type: Option<ListType>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ListBuilder {
    pub fn links(mut self, value: Vec<ListLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn member_count(mut self, value: i64) -> Self {
        self.member_count = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn options(mut self, value: ListOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn r#type(mut self, value: ListType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`List`].
    pub fn build(self) -> Result<List, BuildError> {
        Ok(List {
            links: self.links,
            created_at: self.created_at,
            id: self.id,
            list_id: self.list_id,
            member_count: self.member_count,
            name: self.name,
            options: self.options,
            r#type: self.r#type,
            updated_at: self.updated_at,
        })
    }
}
