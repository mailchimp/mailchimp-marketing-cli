pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMemberTagRequest {
    /// When is_syncing is true, automations based on the tags in the request will not fire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_syncing: Option<bool>,
    /// A list of tags assigned to the list member.
    #[serde(default)]
    pub tags: Vec<CreateMemberTagRequestTagsItem>,
}

impl CreateMemberTagRequest {
    pub fn builder() -> CreateMemberTagRequestBuilder {
        <CreateMemberTagRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberTagRequestBuilder {
    is_syncing: Option<bool>,
    tags: Option<Vec<CreateMemberTagRequestTagsItem>>,
}

impl CreateMemberTagRequestBuilder {
    pub fn is_syncing(mut self, value: bool) -> Self {
        self.is_syncing = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<CreateMemberTagRequestTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberTagRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tags`](CreateMemberTagRequestBuilder::tags)
    pub fn build(self) -> Result<CreateMemberTagRequest, BuildError> {
        Ok(CreateMemberTagRequest {
            is_syncing: self.is_syncing,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
        })
    }
}

