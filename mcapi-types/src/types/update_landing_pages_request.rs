pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateLandingPagesRequest {
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
    /// The title of this landing page seen in the browser's title bar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The tracking settings applied to this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<UpdateLandingPagesRequestTracking>,
}

impl UpdateLandingPagesRequest {
    pub fn builder() -> UpdateLandingPagesRequestBuilder {
        <UpdateLandingPagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLandingPagesRequestBuilder {
    description: Option<String>,
    list_id: Option<String>,
    name: Option<String>,
    store_id: Option<String>,
    title: Option<String>,
    tracking: Option<UpdateLandingPagesRequestTracking>,
}

impl UpdateLandingPagesRequestBuilder {
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

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn tracking(mut self, value: UpdateLandingPagesRequestTracking) -> Self {
        self.tracking = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateLandingPagesRequest`].
    pub fn build(self) -> Result<UpdateLandingPagesRequest, BuildError> {
        Ok(UpdateLandingPagesRequest {
            description: self.description,
            list_id: self.list_id,
            name: self.name,
            store_id: self.store_id,
            title: self.title,
            tracking: self.tracking,
        })
    }
}

