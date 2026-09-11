pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of an individual landing page's settings and content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPage {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<LandingPageLinksItem>>,
    /// The time this landing page was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// Created by mobile or web
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_source: Option<String>,
    /// The description of this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A string that uniquely identifies this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The list's ID associated with this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The name of this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The time this landing page was published.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub published_at: Option<DateTime<FixedOffset>>,
    /// The status of this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LandingPageStatus>,
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
    pub tracking: Option<LandingPageTracking>,
    /// The time this landing page was unpublished.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub unpublished_at: Option<DateTime<FixedOffset>>,
    /// The time this landing page was updated at.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
    /// The url of the published landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The ID used in the Mailchimp web application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<i64>,
}

impl LandingPage {
    pub fn builder() -> LandingPageBuilder {
        <LandingPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageBuilder {
    links: Option<Vec<LandingPageLinksItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by_source: Option<String>,
    description: Option<String>,
    id: Option<String>,
    list_id: Option<String>,
    name: Option<String>,
    published_at: Option<DateTime<FixedOffset>>,
    status: Option<LandingPageStatus>,
    store_id: Option<String>,
    template_id: Option<i64>,
    title: Option<String>,
    tracking: Option<LandingPageTracking>,
    unpublished_at: Option<DateTime<FixedOffset>>,
    updated_at: Option<DateTime<FixedOffset>>,
    url: Option<String>,
    web_id: Option<i64>,
}

impl LandingPageBuilder {
    pub fn links(mut self, value: Vec<LandingPageLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by_source(mut self, value: impl Into<String>) -> Self {
        self.created_by_source = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn published_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.published_at = Some(value);
        self
    }

    pub fn status(mut self, value: LandingPageStatus) -> Self {
        self.status = Some(value);
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

    pub fn tracking(mut self, value: LandingPageTracking) -> Self {
        self.tracking = Some(value);
        self
    }

    pub fn unpublished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.unpublished_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn web_id(mut self, value: i64) -> Self {
        self.web_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPage`].
    pub fn build(self) -> Result<LandingPage, BuildError> {
        Ok(LandingPage {
            links: self.links,
            created_at: self.created_at,
            created_by_source: self.created_by_source,
            description: self.description,
            id: self.id,
            list_id: self.list_id,
            name: self.name,
            published_at: self.published_at,
            status: self.status,
            store_id: self.store_id,
            template_id: self.template_id,
            title: self.title,
            tracking: self.tracking,
            unpublished_at: self.unpublished_at,
            updated_at: self.updated_at,
            url: self.url,
            web_id: self.web_id,
        })
    }
}
