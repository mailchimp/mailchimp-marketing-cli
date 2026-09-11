pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A survey section. On PATCH, include the section id to update it; omit the section from the sections array to delete it (and any question it contains).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SurveySectionRequest {
    /// The section ID. On PATCH, include to update an existing section; omit to add a new section.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The section type.
    pub r#type: SurveySectionRequestType,
    /// Rich text content for introduction or context sections.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Additional section options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, serde_json::Value>>,
    /// A survey question. On PATCH, include the question id to update it. Omitting question id creates a new question; it does not delete an existing one. To delete a question, omit its section from the sections array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<SurveySectionRequestQuestion>,
}

impl SurveySectionRequest {
    pub fn builder() -> SurveySectionRequestBuilder {
        <SurveySectionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurveySectionRequestBuilder {
    id: Option<String>,
    r#type: Option<SurveySectionRequestType>,
    text: Option<String>,
    options: Option<HashMap<String, serde_json::Value>>,
    question: Option<SurveySectionRequestQuestion>,
}

impl SurveySectionRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: SurveySectionRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn question(mut self, value: SurveySectionRequestQuestion) -> Self {
        self.question = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurveySectionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](SurveySectionRequestBuilder::r#type)
    pub fn build(self) -> Result<SurveySectionRequest, BuildError> {
        Ok(SurveySectionRequest {
            id: self.id,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            text: self.text,
            options: self.options,
            question: self.question,
        })
    }
}
