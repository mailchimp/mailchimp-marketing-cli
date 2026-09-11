pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTemplatesRequest {
    /// The id of the folder the template is currently in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// The raw HTML for the template. We  support the Mailchimp [Template Language](https://mailchimp.com/help/getting-started-with-mailchimps-template-language/) in any HTML code passed via the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// The name of the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateTemplatesRequest {
    pub fn builder() -> UpdateTemplatesRequestBuilder {
        <UpdateTemplatesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTemplatesRequestBuilder {
    folder_id: Option<String>,
    html: Option<String>,
    name: Option<String>,
}

impl UpdateTemplatesRequestBuilder {
    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateTemplatesRequest`].
    pub fn build(self) -> Result<UpdateTemplatesRequest, BuildError> {
        Ok(UpdateTemplatesRequest {
            folder_id: self.folder_id,
            html: self.html,
            name: self.name,
        })
    }
}

