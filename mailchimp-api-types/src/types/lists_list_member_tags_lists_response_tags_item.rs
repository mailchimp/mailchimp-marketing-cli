pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberTagsListsResponseTagsItem {
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

impl ListMemberTagsListsResponseTagsItem {
    pub fn builder() -> ListMemberTagsListsResponseTagsItemBuilder {
        <ListMemberTagsListsResponseTagsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberTagsListsResponseTagsItemBuilder {
    date_added: Option<DateTime<FixedOffset>>,
    id: Option<i64>,
    name: Option<String>,
}

impl ListMemberTagsListsResponseTagsItemBuilder {
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

    /// Consumes the builder and constructs a [`ListMemberTagsListsResponseTagsItem`].
    pub fn build(self) -> Result<ListMemberTagsListsResponseTagsItem, BuildError> {
        Ok(ListMemberTagsListsResponseTagsItem {
            date_added: self.date_added,
            id: self.id,
            name: self.name,
        })
    }
}
