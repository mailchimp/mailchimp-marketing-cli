pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Subscriber location information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpsertMemberRequestLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<UpsertMemberRequestLocationLatitude>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<UpsertMemberRequestLocationLongitude>,
}

impl UpsertMemberRequestLocation {
    pub fn builder() -> UpsertMemberRequestLocationBuilder {
        <UpsertMemberRequestLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertMemberRequestLocationBuilder {
    latitude: Option<UpsertMemberRequestLocationLatitude>,
    longitude: Option<UpsertMemberRequestLocationLongitude>,
}

impl UpsertMemberRequestLocationBuilder {
    pub fn latitude(mut self, value: UpsertMemberRequestLocationLatitude) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: UpsertMemberRequestLocationLongitude) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertMemberRequestLocation`].
    pub fn build(self) -> Result<UpsertMemberRequestLocation, BuildError> {
        Ok(UpsertMemberRequestLocation {
            latitude: self.latitude,
            longitude: self.longitude,
        })
    }
}
