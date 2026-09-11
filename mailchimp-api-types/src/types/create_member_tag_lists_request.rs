pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMemberTagListsRequest {
    /// When is_syncing is true, automations based on the tags in the request will not fire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_syncing: Option<bool>,
    /// A list of tags assigned to the list member.
    #[serde(default)]
    pub tags: Vec<CreateMemberTagListsRequestTagsItem>,
}

impl CreateMemberTagListsRequest {
    pub fn builder() -> CreateMemberTagListsRequestBuilder {
        <CreateMemberTagListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberTagListsRequestBuilder {
    is_syncing: Option<bool>,
    tags: Option<Vec<CreateMemberTagListsRequestTagsItem>>,
}

impl CreateMemberTagListsRequestBuilder {
    pub fn is_syncing(mut self, value: bool) -> Self {
        self.is_syncing = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<CreateMemberTagListsRequestTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberTagListsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tags`](CreateMemberTagListsRequestBuilder::tags)
    pub fn build(self) -> Result<CreateMemberTagListsRequest, BuildError> {
        Ok(CreateMemberTagListsRequest {
            is_syncing: self.is_syncing,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
        })
    }
}

