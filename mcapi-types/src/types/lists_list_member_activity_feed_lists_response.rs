pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The member activity events for a given member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMemberActivityFeedListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberActivityFeedListsResponseLinksItem>>,
    /// An array of objects, each representing a contact event. There are multiple possible types, see the [activity schema documentation](https://mailchimp.com/developer/marketing/docs/alternative-schemas/#activity-schemas).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Vec<serde_json::Value>>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
}

impl ListMemberActivityFeedListsResponse {
    pub fn builder() -> ListMemberActivityFeedListsResponseBuilder {
        <ListMemberActivityFeedListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberActivityFeedListsResponseBuilder {
    links: Option<Vec<ListMemberActivityFeedListsResponseLinksItem>>,
    activity: Option<Vec<serde_json::Value>>,
    email_id: Option<String>,
    list_id: Option<String>,
}

impl ListMemberActivityFeedListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberActivityFeedListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn activity(mut self, value: Vec<serde_json::Value>) -> Self {
        self.activity = Some(value);
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMemberActivityFeedListsResponse`].
    pub fn build(self) -> Result<ListMemberActivityFeedListsResponse, BuildError> {
        Ok(ListMemberActivityFeedListsResponse {
            links: self.links,
            activity: self.activity,
            email_id: self.email_id,
            list_id: self.list_id,
        })
    }
}
