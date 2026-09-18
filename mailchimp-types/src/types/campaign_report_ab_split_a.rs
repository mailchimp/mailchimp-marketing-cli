pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Stats for Campaign A.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignReportAbSplitA {
    /// Abuse reports for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_reports: Option<i64>,
    /// Bounces for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounces: Option<i64>,
    /// Forwards for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwards: Option<i64>,
    /// Opens from forwards for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwards_opens: Option<i64>,
    /// The last open for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_open: Option<String>,
    /// Opens for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<i64>,
    /// Recipient Clicks for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_clicks: Option<i64>,
    /// Unique opens for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i64>,
    /// Unsubscribes for Campaign A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubs: Option<i64>,
}

impl CampaignReportAbSplitA {
    pub fn builder() -> CampaignReportAbSplitABuilder {
        <CampaignReportAbSplitABuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportAbSplitABuilder {
    abuse_reports: Option<i64>,
    bounces: Option<i64>,
    forwards: Option<i64>,
    forwards_opens: Option<i64>,
    last_open: Option<String>,
    opens: Option<i64>,
    recipient_clicks: Option<i64>,
    unique_opens: Option<i64>,
    unsubs: Option<i64>,
}

impl CampaignReportAbSplitABuilder {
    pub fn abuse_reports(mut self, value: i64) -> Self {
        self.abuse_reports = Some(value);
        self
    }

    pub fn bounces(mut self, value: i64) -> Self {
        self.bounces = Some(value);
        self
    }

    pub fn forwards(mut self, value: i64) -> Self {
        self.forwards = Some(value);
        self
    }

    pub fn forwards_opens(mut self, value: i64) -> Self {
        self.forwards_opens = Some(value);
        self
    }

    pub fn last_open(mut self, value: impl Into<String>) -> Self {
        self.last_open = Some(value.into());
        self
    }

    pub fn opens(mut self, value: i64) -> Self {
        self.opens = Some(value);
        self
    }

    pub fn recipient_clicks(mut self, value: i64) -> Self {
        self.recipient_clicks = Some(value);
        self
    }

    pub fn unique_opens(mut self, value: i64) -> Self {
        self.unique_opens = Some(value);
        self
    }

    pub fn unsubs(mut self, value: i64) -> Self {
        self.unsubs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportAbSplitA`].
    pub fn build(self) -> Result<CampaignReportAbSplitA, BuildError> {
        Ok(CampaignReportAbSplitA {
            abuse_reports: self.abuse_reports,
            bounces: self.bounces,
            forwards: self.forwards,
            forwards_opens: self.forwards_opens,
            last_open: self.last_open,
            opens: self.opens,
            recipient_clicks: self.recipient_clicks,
            unique_opens: self.unique_opens,
            unsubs: self.unsubs,
        })
    }
}
