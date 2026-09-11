pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single instance of a campaign referral.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEepurlReportsResponseReferrersItem {
    /// The number of clicks a single referrer generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    /// The timestamp for the first click from this referrer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub first_click: Option<DateTime<FixedOffset>>,
    /// The timestamp for the last click from this referrer.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_click: Option<DateTime<FixedOffset>>,
    /// A referrer (truncated to 100 bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
}

impl ListEepurlReportsResponseReferrersItem {
    pub fn builder() -> ListEepurlReportsResponseReferrersItemBuilder {
        <ListEepurlReportsResponseReferrersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEepurlReportsResponseReferrersItemBuilder {
    clicks: Option<i64>,
    first_click: Option<DateTime<FixedOffset>>,
    last_click: Option<DateTime<FixedOffset>>,
    referrer: Option<String>,
}

impl ListEepurlReportsResponseReferrersItemBuilder {
    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn first_click(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_click = Some(value);
        self
    }

    pub fn last_click(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_click = Some(value);
        self
    }

    pub fn referrer(mut self, value: impl Into<String>) -> Self {
        self.referrer = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEepurlReportsResponseReferrersItem`].
    pub fn build(self) -> Result<ListEepurlReportsResponseReferrersItem, BuildError> {
        Ok(ListEepurlReportsResponseReferrersItem {
            clicks: self.clicks,
            first_click: self.first_click,
            last_click: self.last_click,
            referrer: self.referrer,
        })
    }
}
