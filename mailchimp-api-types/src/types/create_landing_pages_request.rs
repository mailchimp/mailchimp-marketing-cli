pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateLandingPagesRequest {
    /// The description of this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The list's ID associated with this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The name of this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the store associated with this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The template_id of this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
    /// The title of this landing page seen in the browser's title bar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The tracking settings applied to this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<CreateLandingPagesRequestTracking>,
    /// The type of template the landing page has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CreateLandingPagesRequestType>,
    /// Will create the Landing Page using the account's Default List instead of requiring a list_id.
    #[serde(skip)]
    pub use_default_list: Option<bool>,
}

impl CreateLandingPagesRequest {
    pub fn builder() -> CreateLandingPagesRequestBuilder {
        <CreateLandingPagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLandingPagesRequestBuilder {
    description: Option<String>,
    list_id: Option<String>,
    name: Option<String>,
    store_id: Option<String>,
    template_id: Option<i64>,
    title: Option<String>,
    tracking: Option<CreateLandingPagesRequestTracking>,
    r#type: Option<CreateLandingPagesRequestType>,
    use_default_list: Option<bool>,
}

impl CreateLandingPagesRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn store_id(mut self, value: impl Into<String>) -> Self {
        self.store_id = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn tracking(mut self, value: CreateLandingPagesRequestTracking) -> Self {
        self.tracking = Some(value);
        self
    }

    pub fn r#type(mut self, value: CreateLandingPagesRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn use_default_list(mut self, value: bool) -> Self {
        self.use_default_list = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateLandingPagesRequest`].
    pub fn build(self) -> Result<CreateLandingPagesRequest, BuildError> {
        Ok(CreateLandingPagesRequest {
            description: self.description,
            list_id: self.list_id,
            name: self.name,
            store_id: self.store_id,
            template_id: self.template_id,
            title: self.title,
            tracking: self.tracking,
            r#type: self.r#type,
            use_default_list: self.use_default_list,
        })
    }
}

