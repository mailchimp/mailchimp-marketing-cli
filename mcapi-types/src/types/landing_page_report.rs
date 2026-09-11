pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of an individual landing page's settings and content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LandingPageReport {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<LandingPageReportLinksItem>>,
    /// The number of clicks to this landing pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    /// The percentage of people who visited your landing page and were added to your list.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub conversion_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<LandingPageReportEcommerce>,
    /// A string that uniquely identifies this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The list id connected to this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// List Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// The name of this landing page the user will see.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The time this landing page was published.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub published_at: Option<DateTime<FixedOffset>>,
    /// A list of tags associated to the landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_tags: Option<Vec<LandingPageReportSignupTagsItem>>,
    /// The status of the landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The number of subscribes to this landing pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeseries: Option<LandingPageReportTimeseries>,
    /// The name of the landing page the user's customers will see.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The number of unique visits to this landing pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_visits: Option<i64>,
    /// The time this landing page was unpublished.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub unpublished_at: Option<DateTime<FixedOffset>>,
    /// The landing page url.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The number of visits to this landing pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visits: Option<i64>,
    /// The ID used in the Mailchimp web application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<i64>,
}

impl LandingPageReport {
    pub fn builder() -> LandingPageReportBuilder {
        <LandingPageReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportBuilder {
    links: Option<Vec<LandingPageReportLinksItem>>,
    clicks: Option<i64>,
    conversion_rate: Option<f64>,
    ecommerce: Option<LandingPageReportEcommerce>,
    id: Option<String>,
    list_id: Option<String>,
    list_name: Option<String>,
    name: Option<String>,
    published_at: Option<DateTime<FixedOffset>>,
    signup_tags: Option<Vec<LandingPageReportSignupTagsItem>>,
    status: Option<String>,
    subscribes: Option<i64>,
    timeseries: Option<LandingPageReportTimeseries>,
    title: Option<String>,
    unique_visits: Option<i64>,
    unpublished_at: Option<DateTime<FixedOffset>>,
    url: Option<String>,
    visits: Option<i64>,
    web_id: Option<i64>,
}

impl LandingPageReportBuilder {
    pub fn links(mut self, value: Vec<LandingPageReportLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn conversion_rate(mut self, value: f64) -> Self {
        self.conversion_rate = Some(value);
        self
    }

    pub fn ecommerce(mut self, value: LandingPageReportEcommerce) -> Self {
        self.ecommerce = Some(value);
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

    pub fn list_name(mut self, value: impl Into<String>) -> Self {
        self.list_name = Some(value.into());
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

    pub fn signup_tags(mut self, value: Vec<LandingPageReportSignupTagsItem>) -> Self {
        self.signup_tags = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn subscribes(mut self, value: i64) -> Self {
        self.subscribes = Some(value);
        self
    }

    pub fn timeseries(mut self, value: LandingPageReportTimeseries) -> Self {
        self.timeseries = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn unique_visits(mut self, value: i64) -> Self {
        self.unique_visits = Some(value);
        self
    }

    pub fn unpublished_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.unpublished_at = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn visits(mut self, value: i64) -> Self {
        self.visits = Some(value);
        self
    }

    pub fn web_id(mut self, value: i64) -> Self {
        self.web_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReport`].
    pub fn build(self) -> Result<LandingPageReport, BuildError> {
        Ok(LandingPageReport {
            links: self.links,
            clicks: self.clicks,
            conversion_rate: self.conversion_rate,
            ecommerce: self.ecommerce,
            id: self.id,
            list_id: self.list_id,
            list_name: self.list_name,
            name: self.name,
            published_at: self.published_at,
            signup_tags: self.signup_tags,
            status: self.status,
            subscribes: self.subscribes,
            timeseries: self.timeseries,
            title: self.title,
            unique_visits: self.unique_visits,
            unpublished_at: self.unpublished_at,
            url: self.url,
            visits: self.visits,
            web_id: self.web_id,
        })
    }
}
