pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Add or remove tags on a member by declaring a tag either active or inactive on a member.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateMemberTagRequestTagsItem {
    /// The name of the tag.
    #[serde(default)]
    pub name: String,
    /// The status for the tag on the member, pass in active to add a tag or inactive to remove it.
    pub status: CreateMemberTagRequestTagsItemStatus,
}

impl CreateMemberTagRequestTagsItem {
    pub fn builder() -> CreateMemberTagRequestTagsItemBuilder {
        <CreateMemberTagRequestTagsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberTagRequestTagsItemBuilder {
    name: Option<String>,
    status: Option<CreateMemberTagRequestTagsItemStatus>,
}

impl CreateMemberTagRequestTagsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateMemberTagRequestTagsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberTagRequestTagsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateMemberTagRequestTagsItemBuilder::name)
    /// - [`status`](CreateMemberTagRequestTagsItemBuilder::status)
    pub fn build(self) -> Result<CreateMemberTagRequestTagsItem, BuildError> {
        Ok(CreateMemberTagRequestTagsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
