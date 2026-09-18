pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An array of objects, each representing a contact record.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAudienceContactListResponse {
    /// An array of objects, each representing a contact record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<AudiencesContact>>,
    /// A cursor pointing to the last item on this page of the collection. Paginate through a collection of records by setting the `cursor` parameter on a subsequent request to this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<GetAudienceContactListResponseLinksItem>>,
}

impl GetAudienceContactListResponse {
    pub fn builder() -> GetAudienceContactListResponseBuilder {
        <GetAudienceContactListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAudienceContactListResponseBuilder {
    contacts: Option<Vec<AudiencesContact>>,
    next_cursor: Option<String>,
    links: Option<Vec<GetAudienceContactListResponseLinksItem>>,
}

impl GetAudienceContactListResponseBuilder {
    pub fn contacts(mut self, value: Vec<AudiencesContact>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn links(mut self, value: Vec<GetAudienceContactListResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAudienceContactListResponse`].
    pub fn build(self) -> Result<GetAudienceContactListResponse, BuildError> {
        Ok(GetAudienceContactListResponse {
            contacts: self.contacts,
            next_cursor: self.next_cursor,
            links: self.links,
        })
    }
}
