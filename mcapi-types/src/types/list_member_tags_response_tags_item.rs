pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberTagsResponseTagsItem {
    /// The date and time the tag was added to the list member in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub date_added: Option<DateTime<FixedOffset>>,
    /// The unique id for the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListMemberTagsResponseTagsItem {
    pub fn builder() -> ListMemberTagsResponseTagsItemBuilder {
        <ListMemberTagsResponseTagsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberTagsResponseTagsItemBuilder {
    date_added: Option<DateTime<FixedOffset>>,
    id: Option<i64>,
    name: Option<String>,
}

impl ListMemberTagsResponseTagsItemBuilder {
    pub fn date_added(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date_added = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMemberTagsResponseTagsItem`].
    pub fn build(self) -> Result<ListMemberTagsResponseTagsItem, BuildError> {
        Ok(ListMemberTagsResponseTagsItem {
            date_added: self.date_added,
            id: self.id,
            name: self.name,
        })
    }
}
