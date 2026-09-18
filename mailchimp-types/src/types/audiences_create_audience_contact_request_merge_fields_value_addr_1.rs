pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAudienceContactRequestMergeFieldsValueAddr1 {
    #[serde(default)]
    pub addr1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addr2: Option<String>,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub zip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl CreateAudienceContactRequestMergeFieldsValueAddr1 {
    pub fn builder() -> CreateAudienceContactRequestMergeFieldsValueAddr1Builder {
        <CreateAudienceContactRequestMergeFieldsValueAddr1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudienceContactRequestMergeFieldsValueAddr1Builder {
    addr1: Option<String>,
    addr2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
}

impl CreateAudienceContactRequestMergeFieldsValueAddr1Builder {
    pub fn addr1(mut self, value: impl Into<String>) -> Self {
        self.addr1 = Some(value.into());
        self
    }

    pub fn addr2(mut self, value: impl Into<String>) -> Self {
        self.addr2 = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn zip(mut self, value: impl Into<String>) -> Self {
        self.zip = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAudienceContactRequestMergeFieldsValueAddr1`].
    /// This method will fail if any of the following fields are not set:
    /// - [`addr1`](CreateAudienceContactRequestMergeFieldsValueAddr1Builder::addr1)
    /// - [`city`](CreateAudienceContactRequestMergeFieldsValueAddr1Builder::city)
    /// - [`state`](CreateAudienceContactRequestMergeFieldsValueAddr1Builder::state)
    /// - [`zip`](CreateAudienceContactRequestMergeFieldsValueAddr1Builder::zip)
    pub fn build(self) -> Result<CreateAudienceContactRequestMergeFieldsValueAddr1, BuildError> {
        Ok(CreateAudienceContactRequestMergeFieldsValueAddr1 {
            addr1: self.addr1.ok_or_else(|| BuildError::missing_field("addr1"))?,
            addr2: self.addr2,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            zip: self.zip.ok_or_else(|| BuildError::missing_field("zip"))?,
            country: self.country,
        })
    }
}
