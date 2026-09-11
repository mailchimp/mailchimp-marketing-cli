pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Subscriber location information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMembersLocation {
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
    /// The region for the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The timezone for the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl ListMembersLocation {
    pub fn builder() -> ListMembersLocationBuilder {
        <ListMembersLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersLocationBuilder {
    country_code: Option<String>,
    dstoff: Option<i64>,
    gmtoff: Option<i64>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    region: Option<String>,
    timezone: Option<String>,
}

impl ListMembersLocationBuilder {
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

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMembersLocation`].
    pub fn build(self) -> Result<ListMembersLocation, BuildError> {
        Ok(ListMembersLocation {
            country_code: self.country_code,
            dstoff: self.dstoff,
            gmtoff: self.gmtoff,
            latitude: self.latitude,
            longitude: self.longitude,
            region: self.region,
            timezone: self.timezone,
        })
    }
}
