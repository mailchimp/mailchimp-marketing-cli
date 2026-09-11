pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Subscriber location information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateMemberListsRequestLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<UpdateMemberListsRequestLocationLatitude>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<UpdateMemberListsRequestLocationLongitude>,
}

impl UpdateMemberListsRequestLocation {
    pub fn builder() -> UpdateMemberListsRequestLocationBuilder {
        <UpdateMemberListsRequestLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMemberListsRequestLocationBuilder {
    latitude: Option<UpdateMemberListsRequestLocationLatitude>,
    longitude: Option<UpdateMemberListsRequestLocationLongitude>,
}

impl UpdateMemberListsRequestLocationBuilder {
    pub fn latitude(mut self, value: UpdateMemberListsRequestLocationLatitude) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: UpdateMemberListsRequestLocationLongitude) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMemberListsRequestLocation`].
    pub fn build(self) -> Result<UpdateMemberListsRequestLocation, BuildError> {
        Ok(UpdateMemberListsRequestLocation {
            latitude: self.latitude,
            longitude: self.longitude,
        })
    }
}
