pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Subscriber location information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateMemberRequestLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<CreateMemberRequestLocationLatitude>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<CreateMemberRequestLocationLongitude>,
}

impl CreateMemberRequestLocation {
    pub fn builder() -> CreateMemberRequestLocationBuilder {
        <CreateMemberRequestLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberRequestLocationBuilder {
    latitude: Option<CreateMemberRequestLocationLatitude>,
    longitude: Option<CreateMemberRequestLocationLongitude>,
}

impl CreateMemberRequestLocationBuilder {
    pub fn latitude(mut self, value: CreateMemberRequestLocationLatitude) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: CreateMemberRequestLocationLongitude) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberRequestLocation`].
    pub fn build(self) -> Result<CreateMemberRequestLocation, BuildError> {
        Ok(CreateMemberRequestLocation {
            latitude: self.latitude,
            longitude: self.longitude,
        })
    }
}
