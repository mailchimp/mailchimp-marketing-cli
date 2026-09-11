pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A report of links clicked in a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClickDetailReport {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ClickDetailReportLinksItem>>,
    /// A breakdown of clicks by different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ab_split: Option<ClickDetailReportAbSplit>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The percentage of total clicks a link generated for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_percentage: Option<f64>,
    /// The unique id for the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The date and time for the last recorded click for a link in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_click: Option<DateTime<FixedOffset>>,
    /// The number of total clicks for a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_clicks: Option<i64>,
    /// The percentage of unique clicks a link generated for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unique_click_percentage: Option<f64>,
    /// Number of unique clicks for a link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_clicks: Option<i64>,
    /// The URL for the link in the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ClickDetailReport {
    pub fn builder() -> ClickDetailReportBuilder {
        <ClickDetailReportBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClickDetailReportBuilder {
    links: Option<Vec<ClickDetailReportLinksItem>>,
    ab_split: Option<ClickDetailReportAbSplit>,
    campaign_id: Option<String>,
    click_percentage: Option<f64>,
    id: Option<String>,
    last_click: Option<DateTime<FixedOffset>>,
    total_clicks: Option<i64>,
    unique_click_percentage: Option<f64>,
    unique_clicks: Option<i64>,
    url: Option<String>,
}

impl ClickDetailReportBuilder {
    pub fn links(mut self, value: Vec<ClickDetailReportLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn ab_split(mut self, value: ClickDetailReportAbSplit) -> Self {
        self.ab_split = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn click_percentage(mut self, value: f64) -> Self {
        self.click_percentage = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_click(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_click = Some(value);
        self
    }

    pub fn total_clicks(mut self, value: i64) -> Self {
        self.total_clicks = Some(value);
        self
    }

    pub fn unique_click_percentage(mut self, value: f64) -> Self {
        self.unique_click_percentage = Some(value);
        self
    }

    pub fn unique_clicks(mut self, value: i64) -> Self {
        self.unique_clicks = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClickDetailReport`].
    pub fn build(self) -> Result<ClickDetailReport, BuildError> {
        Ok(ClickDetailReport {
            links: self.links,
            ab_split: self.ab_split,
            campaign_id: self.campaign_id,
            click_percentage: self.click_percentage,
            id: self.id,
            last_click: self.last_click,
            total_clicks: self.total_clicks,
            unique_click_percentage: self.unique_click_percentage,
            unique_clicks: self.unique_clicks,
            url: self.url,
        })
    }
}
