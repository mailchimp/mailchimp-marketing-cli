pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Subscriber location information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateMemberRequestLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<UpdateMemberRequestLocationLatitude>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<UpdateMemberRequestLocationLongitude>,
}

impl UpdateMemberRequestLocation {
    pub fn builder() -> UpdateMemberRequestLocationBuilder {
        <UpdateMemberRequestLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMemberRequestLocationBuilder {
    latitude: Option<UpdateMemberRequestLocationLatitude>,
    longitude: Option<UpdateMemberRequestLocationLongitude>,
}

impl UpdateMemberRequestLocationBuilder {
    pub fn latitude(mut self, value: UpdateMemberRequestLocationLatitude) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: UpdateMemberRequestLocationLongitude) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMemberRequestLocation`].
    pub fn build(self) -> Result<UpdateMemberRequestLocation, BuildError> {
        Ok(UpdateMemberRequestLocation {
            latitude: self.latitude,
            longitude: self.longitude,
        })
    }
}
