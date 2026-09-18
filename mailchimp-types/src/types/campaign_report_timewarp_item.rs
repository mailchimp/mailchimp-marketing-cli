pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignReportTimewarpItem {
    /// The number of bounces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounces: Option<i64>,
    /// The number of clicks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    /// For campaigns sent with timewarp, the time zone group the member is apart of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmt_offset: Option<i64>,
    /// The date and time of the last click in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_click: Option<DateTime<FixedOffset>>,
    /// The date and time of the last open in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_open: Option<DateTime<FixedOffset>>,
    /// The number of opens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<i64>,
    /// The number of unique clicks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_clicks: Option<i64>,
    /// The number of unique opens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i64>,
}

impl CampaignReportTimewarpItem {
    pub fn builder() -> CampaignReportTimewarpItemBuilder {
        <CampaignReportTimewarpItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportTimewarpItemBuilder {
    bounces: Option<i64>,
    clicks: Option<i64>,
    gmt_offset: Option<i64>,
    last_click: Option<DateTime<FixedOffset>>,
    last_open: Option<DateTime<FixedOffset>>,
    opens: Option<i64>,
    unique_clicks: Option<i64>,
    unique_opens: Option<i64>,
}

impl CampaignReportTimewarpItemBuilder {
    pub fn bounces(mut self, value: i64) -> Self {
        self.bounces = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn gmt_offset(mut self, value: i64) -> Self {
        self.gmt_offset = Some(value);
        self
    }

    pub fn last_click(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_click = Some(value);
        self
    }

    pub fn last_open(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_open = Some(value);
        self
    }

    pub fn opens(mut self, value: i64) -> Self {
        self.opens = Some(value);
        self
    }

    pub fn unique_clicks(mut self, value: i64) -> Self {
        self.unique_clicks = Some(value);
        self
    }

    pub fn unique_opens(mut self, value: i64) -> Self {
        self.unique_opens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportTimewarpItem`].
    pub fn build(self) -> Result<CampaignReportTimewarpItem, BuildError> {
        Ok(CampaignReportTimewarpItem {
            bounces: self.bounces,
            clicks: self.clicks,
            gmt_offset: self.gmt_offset,
            last_click: self.last_click,
            last_open: self.last_open,
            opens: self.opens,
            unique_clicks: self.unique_clicks,
            unique_opens: self.unique_opens,
        })
    }
}
