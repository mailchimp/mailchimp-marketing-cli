pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchAddOrRemoveMembersRequest {
    /// An array of emails to be used for a static segment. Any emails provided that are not present on the list will be ignored. A maximum of 500 members can be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_to_add: Option<Vec<String>>,
    /// An array of emails to be used for a static segment. Any emails provided that are not present on the list will be ignored. A maximum of 500 members can be sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_to_remove: Option<Vec<String>>,
}

impl BatchAddOrRemoveMembersRequest {
    pub fn builder() -> BatchAddOrRemoveMembersRequestBuilder {
        <BatchAddOrRemoveMembersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchAddOrRemoveMembersRequestBuilder {
    members_to_add: Option<Vec<String>>,
    members_to_remove: Option<Vec<String>>,
}

impl BatchAddOrRemoveMembersRequestBuilder {
    pub fn members_to_add(mut self, value: Vec<String>) -> Self {
        self.members_to_add = Some(value);
        self
    }

    pub fn members_to_remove(mut self, value: Vec<String>) -> Self {
        self.members_to_remove = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchAddOrRemoveMembersRequest`].
    pub fn build(self) -> Result<BatchAddOrRemoveMembersRequest, BuildError> {
        Ok(BatchAddOrRemoveMembersRequest {
            members_to_add: self.members_to_add,
            members_to_remove: self.members_to_remove,
        })
    }
}

