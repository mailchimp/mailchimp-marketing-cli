pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTemplatesRequest {
    /// The id of the folder the template is currently in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// The raw HTML for the template. We  support the Mailchimp [Template Language](https://mailchimp.com/help/getting-started-with-mailchimps-template-language/) in any HTML code passed via the API.
    #[serde(default)]
    pub html: String,
    /// The name of the template.
    #[serde(default)]
    pub name: String,
}

impl CreateTemplatesRequest {
    pub fn builder() -> CreateTemplatesRequestBuilder {
        <CreateTemplatesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTemplatesRequestBuilder {
    folder_id: Option<String>,
    html: Option<String>,
    name: Option<String>,
}

impl CreateTemplatesRequestBuilder {
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

    /// Consumes the builder and constructs a [`CreateTemplatesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`html`](CreateTemplatesRequestBuilder::html)
    /// - [`name`](CreateTemplatesRequestBuilder::name)
    pub fn build(self) -> Result<CreateTemplatesRequest, BuildError> {
        Ok(CreateTemplatesRequest {
            folder_id: self.folder_id,
            html: self.html.ok_or_else(|| BuildError::missing_field("html"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

