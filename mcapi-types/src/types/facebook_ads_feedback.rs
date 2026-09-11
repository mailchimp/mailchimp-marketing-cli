pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Check if this ad is connected to a facebook page
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FacebookAdsFeedback {
    /// Feedback regarding the audience of this Ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    /// Feedback regarding the budget of this Ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<String>,
    /// Feedback regarding the compliance of this Ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<String>,
    /// Feedback regarding the content of this Ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl FacebookAdsFeedback {
    pub fn builder() -> FacebookAdsFeedbackBuilder {
        <FacebookAdsFeedbackBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdsFeedbackBuilder {
    audience: Option<String>,
    budget: Option<String>,
    compliance: Option<String>,
    content: Option<String>,
}

impl FacebookAdsFeedbackBuilder {
    pub fn audience(mut self, value: impl Into<String>) -> Self {
        self.audience = Some(value.into());
        self
    }

    pub fn budget(mut self, value: impl Into<String>) -> Self {
        self.budget = Some(value.into());
        self
    }

    pub fn compliance(mut self, value: impl Into<String>) -> Self {
        self.compliance = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdsFeedback`].
    pub fn build(self) -> Result<FacebookAdsFeedback, BuildError> {
        Ok(FacebookAdsFeedback {
            audience: self.audience,
            budget: self.budget,
            compliance: self.compliance,
            content: self.content,
        })
    }
}
