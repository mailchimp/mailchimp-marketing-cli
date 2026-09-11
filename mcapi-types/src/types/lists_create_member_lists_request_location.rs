pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Subscriber location information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateMemberListsRequestLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<CreateMemberListsRequestLocationLatitude>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<CreateMemberListsRequestLocationLongitude>,
}

impl CreateMemberListsRequestLocation {
    pub fn builder() -> CreateMemberListsRequestLocationBuilder {
        <CreateMemberListsRequestLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberListsRequestLocationBuilder {
    latitude: Option<CreateMemberListsRequestLocationLatitude>,
    longitude: Option<CreateMemberListsRequestLocationLongitude>,
}

impl CreateMemberListsRequestLocationBuilder {
    pub fn latitude(mut self, value: CreateMemberListsRequestLocationLatitude) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: CreateMemberListsRequestLocationLongitude) -> Self {
        self.longitude = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberListsRequestLocation`].
    pub fn build(self) -> Result<CreateMemberListsRequestLocation, BuildError> {
        Ok(CreateMemberListsRequestLocation {
            latitude: self.latitude,
            longitude: self.longitude,
        })
    }
}
