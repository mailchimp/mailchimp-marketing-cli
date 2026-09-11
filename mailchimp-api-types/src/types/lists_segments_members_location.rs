pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Subscriber location information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListsSegmentsMembersLocation {
    /// The unique code for the location country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The offset for timezones where daylight saving time is observed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dstoff: Option<i64>,
    /// The time difference in hours from GMT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gmtoff: Option<i64>,
    /// The location latitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    /// The location longitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
    /// The timezone for the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl ListsSegmentsMembersLocation {
    pub fn builder() -> ListsSegmentsMembersLocationBuilder {
        <ListsSegmentsMembersLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListsSegmentsMembersLocationBuilder {
    country_code: Option<String>,
    dstoff: Option<i64>,
    gmtoff: Option<i64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    timezone: Option<String>,
}

impl ListsSegmentsMembersLocationBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn dstoff(mut self, value: i64) -> Self {
        self.dstoff = Some(value);
        self
    }

    pub fn gmtoff(mut self, value: i64) -> Self {
        self.gmtoff = Some(value);
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListsSegmentsMembersLocation`].
    pub fn build(self) -> Result<ListsSegmentsMembersLocation, BuildError> {
        Ok(ListsSegmentsMembersLocation {
            country_code: self.country_code,
            dstoff: self.dstoff,
            gmtoff: self.gmtoff,
            latitude: self.latitude,
            longitude: self.longitude,
            timezone: self.timezone,
        })
    }
}
