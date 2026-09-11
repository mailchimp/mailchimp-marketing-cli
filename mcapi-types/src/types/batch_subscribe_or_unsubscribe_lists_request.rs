pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchSubscribeOrUnsubscribeListsRequest {
    /// An array of objects, each representing an email address and the subscription status for a specific list. Up to 500 members may be added or updated with each API call.
    #[serde(default)]
    pub members: Vec<BatchSubscribeOrUnsubscribeListsRequestMembersItem>,
    /// Whether this batch operation will replace all existing tags with tags in request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_tags: Option<bool>,
    /// Whether this batch operation will change existing members' subscription status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_existing: Option<bool>,
    /// If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    #[serde(skip)]
    pub skip_merge_validation: Option<bool>,
    /// If skip_duplicate_check is true, we will ignore duplicates sent in the request when using the batch sub/unsub on the lists endpoint. The status of the first appearance in the request will be saved. This defaults to false.
    #[serde(skip)]
    pub skip_duplicate_check: Option<bool>,
}

impl BatchSubscribeOrUnsubscribeListsRequest {
    pub fn builder() -> BatchSubscribeOrUnsubscribeListsRequestBuilder {
        <BatchSubscribeOrUnsubscribeListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchSubscribeOrUnsubscribeListsRequestBuilder {
    members: Option<Vec<BatchSubscribeOrUnsubscribeListsRequestMembersItem>>,
    sync_tags: Option<bool>,
    update_existing: Option<bool>,
    skip_merge_validation: Option<bool>,
    skip_duplicate_check: Option<bool>,
}

impl BatchSubscribeOrUnsubscribeListsRequestBuilder {
    pub fn members(mut self, value: Vec<BatchSubscribeOrUnsubscribeListsRequestMembersItem>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn sync_tags(mut self, value: bool) -> Self {
        self.sync_tags = Some(value);
        self
    }

    pub fn update_existing(mut self, value: bool) -> Self {
        self.update_existing = Some(value);
        self
    }

    pub fn skip_merge_validation(mut self, value: bool) -> Self {
        self.skip_merge_validation = Some(value);
        self
    }

    pub fn skip_duplicate_check(mut self, value: bool) -> Self {
        self.skip_duplicate_check = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchSubscribeOrUnsubscribeListsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`members`](BatchSubscribeOrUnsubscribeListsRequestBuilder::members)
    pub fn build(self) -> Result<BatchSubscribeOrUnsubscribeListsRequest, BuildError> {
        Ok(BatchSubscribeOrUnsubscribeListsRequest {
            members: self.members.ok_or_else(|| BuildError::missing_field("members"))?,
            sync_tags: self.sync_tags,
            update_existing: self.update_existing,
            skip_merge_validation: self.skip_merge_validation,
            skip_duplicate_check: self.skip_duplicate_check,
        })
    }
}

