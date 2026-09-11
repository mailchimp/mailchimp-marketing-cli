pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Subscriber location information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpsertMemberListsRequestLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<UpsertMemberListsRequestLocationLatitude>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<UpsertMemberListsRequestLocationLongitude>,
}

impl UpsertMemberListsRequestLocation {
    pub fn builder() -> UpsertMemberListsRequestLocationBuilder {
        <UpsertMemberListsRequestLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertMemberListsRequestLocationBuilder {
    latitude: Option<UpsertMemberListsRequestLocationLatitude>,
    longitude: Option<UpsertMemberListsRequestLocationLongitude>,
}

impl UpsertMemberListsRequestLocationBuilder {
    pub fn latitude(mut self, value: UpsertMemberListsRequestLocationLatitude) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: UpsertMemberListsRequestLocationLongitude) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertMemberListsRequestLocation`].
    pub fn build(self) -> Result<UpsertMemberListsRequestLocation, BuildError> {
        Ok(UpsertMemberListsRequestLocation {
            latitude: self.latitude,
            longitude: self.longitude,
        })
    }
}
