pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// This object represents a link from the resource where it is found to another resource or action that may be performed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignLinksItem {
    /// This property contains a fully-qualified URL that can be called to retrieve the linked resource or perform the linked action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// The HTTP method that should be used when accessing the URL defined in 'href'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<CampaignLinksItemMethod>,
    /// As with an HTML 'rel' attribute, this describes the type of link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
    /// For HTTP methods that can receive bodies (POST and PUT), this is a URL representing the schema that the body should conform to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    /// For GETs, this is a URL representing the schema that the response should conform to.
    #[serde(rename = "targetSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_schema: Option<String>,
}

impl CampaignLinksItem {
    pub fn builder() -> CampaignLinksItemBuilder {
        <CampaignLinksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignLinksItemBuilder {
    href: Option<String>,
    method: Option<CampaignLinksItemMethod>,
    rel: Option<String>,
    schema: Option<String>,
    target_schema: Option<String>,
}

impl CampaignLinksItemBuilder {
    pub fn href(mut self, value: impl Into<String>) -> Self {
        self.href = Some(value.into());
        self
    }

    pub fn method(mut self, value: CampaignLinksItemMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn rel(mut self, value: impl Into<String>) -> Self {
        self.rel = Some(value.into());
        self
    }

    pub fn schema(mut self, value: impl Into<String>) -> Self {
        self.schema = Some(value.into());
        self
    }

    pub fn target_schema(mut self, value: impl Into<String>) -> Self {
        self.target_schema = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CampaignLinksItem`].
    pub fn build(self) -> Result<CampaignLinksItem, BuildError> {
        Ok(CampaignLinksItem {
            href: self.href,
            method: self.method,
            rel: self.rel,
            schema: self.schema,
            target_schema: self.target_schema,
        })
    }
}
