pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSurveyListsRequest {
    /// The title of the survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Initial survey sections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<SurveySectionRequest>>,
}

impl CreateSurveyListsRequest {
    pub fn builder() -> CreateSurveyListsRequestBuilder {
        <CreateSurveyListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSurveyListsRequestBuilder {
    title: Option<String>,
    sections: Option<Vec<SurveySectionRequest>>,
}

impl CreateSurveyListsRequestBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn sections(mut self, value: Vec<SurveySectionRequest>) -> Self {
        self.sections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSurveyListsRequest`].
    pub fn build(self) -> Result<CreateSurveyListsRequest, BuildError> {
        Ok(CreateSurveyListsRequest {
            title: self.title,
            sections: self.sections,
        })
    }
}

