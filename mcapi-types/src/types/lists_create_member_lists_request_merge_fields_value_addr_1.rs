pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMemberListsRequestMergeFieldsValueAddr1 {
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

impl CreateMemberListsRequestMergeFieldsValueAddr1 {
    pub fn builder() -> CreateMemberListsRequestMergeFieldsValueAddr1Builder {
        <CreateMemberListsRequestMergeFieldsValueAddr1Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberListsRequestMergeFieldsValueAddr1Builder {
    addr1: Option<String>,
    addr2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
}

impl CreateMemberListsRequestMergeFieldsValueAddr1Builder {
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

    /// Consumes the builder and constructs a [`CreateMemberListsRequestMergeFieldsValueAddr1`].
    /// This method will fail if any of the following fields are not set:
    /// - [`addr1`](CreateMemberListsRequestMergeFieldsValueAddr1Builder::addr1)
    /// - [`city`](CreateMemberListsRequestMergeFieldsValueAddr1Builder::city)
    /// - [`state`](CreateMemberListsRequestMergeFieldsValueAddr1Builder::state)
    /// - [`zip`](CreateMemberListsRequestMergeFieldsValueAddr1Builder::zip)
    pub fn build(self) -> Result<CreateMemberListsRequestMergeFieldsValueAddr1, BuildError> {
        Ok(CreateMemberListsRequestMergeFieldsValueAddr1 {
            addr1: self.addr1.ok_or_else(|| BuildError::missing_field("addr1"))?,
            addr2: self.addr2,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            zip: self.zip.ok_or_else(|| BuildError::missing_field("zip"))?,
            country: self.country,
        })
    }
}
