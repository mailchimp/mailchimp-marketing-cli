pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateListSurveyActionReplicateListsRequest {
    /// The title for the replicated survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The unique ID of the audience for the replicated survey. Defaults to the source survey audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
}

impl CreateListSurveyActionReplicateListsRequest {
    pub fn builder() -> CreateListSurveyActionReplicateListsRequestBuilder {
        <CreateListSurveyActionReplicateListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateListSurveyActionReplicateListsRequestBuilder {
    title: Option<String>,
    list_id: Option<String>,
}

impl CreateListSurveyActionReplicateListsRequestBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateListSurveyActionReplicateListsRequest`].
    pub fn build(self) -> Result<CreateListSurveyActionReplicateListsRequest, BuildError> {
        Ok(CreateListSurveyActionReplicateListsRequest {
            title: self.title,
            list_id: self.list_id,
        })
    }
}

