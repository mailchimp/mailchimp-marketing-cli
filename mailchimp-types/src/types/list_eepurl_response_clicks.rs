pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of the click-throughs on the campaign's URL.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEepurlResponseClicks {
    /// The total number of clicks to the campaign's URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    /// The timestamp for the first click to the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub first_click: Option<DateTime<FixedOffset>>,
    /// The timestamp for the last click to the URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_click: Option<DateTime<FixedOffset>>,
    /// A summary of the top click locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<ListEepurlResponseClicksLocationsItem>>,
}

impl ListEepurlResponseClicks {
    pub fn builder() -> ListEepurlResponseClicksBuilder {
        <ListEepurlResponseClicksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEepurlResponseClicksBuilder {
    clicks: Option<i64>,
    first_click: Option<DateTime<FixedOffset>>,
    last_click: Option<DateTime<FixedOffset>>,
    locations: Option<Vec<ListEepurlResponseClicksLocationsItem>>,
}

impl ListEepurlResponseClicksBuilder {
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

    pub fn locations(mut self, value: Vec<ListEepurlResponseClicksLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEepurlResponseClicks`].
    pub fn build(self) -> Result<ListEepurlResponseClicks, BuildError> {
        Ok(ListEepurlResponseClicks {
            clicks: self.clicks,
            first_click: self.first_click,
            last_click: self.last_click,
            locations: self.locations,
        })
    }
}
