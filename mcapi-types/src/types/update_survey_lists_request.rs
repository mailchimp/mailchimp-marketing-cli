pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateSurveyListsRequest {
    /// The title of the survey.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether responses are sent to Mailchimp Inbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_piped_to_inbox: Option<bool>,
    /// The complete survey section list in display order. On update, sections omitted from this array are deleted. Include section id to update an existing section; omit section id to add a new section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<SurveySectionRequest>>,
}

impl UpdateSurveyListsRequest {
    pub fn builder() -> UpdateSurveyListsRequestBuilder {
        <UpdateSurveyListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSurveyListsRequestBuilder {
    title: Option<String>,
    is_piped_to_inbox: Option<bool>,
    sections: Option<Vec<SurveySectionRequest>>,
}

impl UpdateSurveyListsRequestBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn is_piped_to_inbox(mut self, value: bool) -> Self {
        self.is_piped_to_inbox = Some(value);
        self
    }

    pub fn sections(mut self, value: Vec<SurveySectionRequest>) -> Self {
        self.sections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSurveyListsRequest`].
    pub fn build(self) -> Result<UpdateSurveyListsRequest, BuildError> {
        Ok(UpdateSurveyListsRequest {
            title: self.title,
            is_piped_to_inbox: self.is_piped_to_inbox,
            sections: self.sections,
        })
    }
}

