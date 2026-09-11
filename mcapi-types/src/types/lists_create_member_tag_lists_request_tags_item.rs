pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Add or remove tags on a member by declaring a tag either active or inactive on a member.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateMemberTagListsRequestTagsItem {
    /// The name of the tag.
    #[serde(default)]
    pub name: String,
    /// The status for the tag on the member, pass in active to add a tag or inactive to remove it.
    pub status: CreateMemberTagListsRequestTagsItemStatus,
}

impl CreateMemberTagListsRequestTagsItem {
    pub fn builder() -> CreateMemberTagListsRequestTagsItemBuilder {
        <CreateMemberTagListsRequestTagsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberTagListsRequestTagsItemBuilder {
    name: Option<String>,
    status: Option<CreateMemberTagListsRequestTagsItemStatus>,
}

impl CreateMemberTagListsRequestTagsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateMemberTagListsRequestTagsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberTagListsRequestTagsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateMemberTagListsRequestTagsItemBuilder::name)
    /// - [`status`](CreateMemberTagListsRequestTagsItemBuilder::status)
    pub fn build(self) -> Result<CreateMemberTagListsRequestTagsItem, BuildError> {
        Ok(CreateMemberTagListsRequestTagsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
