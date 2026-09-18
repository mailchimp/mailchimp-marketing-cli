pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMembersTagsItem {
    /// The tag id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListMembersTagsItem {
    pub fn builder() -> ListMembersTagsItemBuilder {
        <ListMembersTagsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersTagsItemBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl ListMembersTagsItemBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMembersTagsItem`].
    pub fn build(self) -> Result<ListMembersTagsItem, BuildError> {
        Ok(ListMembersTagsItem {
            id: self.id,
            name: self.name,
        })
    }
}
