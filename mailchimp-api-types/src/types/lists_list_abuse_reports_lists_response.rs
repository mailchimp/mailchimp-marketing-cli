pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of abuse complaints for a specific list. An abuse complaint occurs when your recipient clicks to 'report spam' in their email program.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAbuseReportsListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListAbuseReportsListsResponseLinksItem>>,
    /// An array of objects, each representing an abuse report resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_reports: Option<Vec<ListsAbuseReports>>,
    /// The list id for the abuse report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListAbuseReportsListsResponse {
    pub fn builder() -> ListAbuseReportsListsResponseBuilder {
        <ListAbuseReportsListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAbuseReportsListsResponseBuilder {
    links: Option<Vec<ListAbuseReportsListsResponseLinksItem>>,
    abuse_reports: Option<Vec<ListsAbuseReports>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListAbuseReportsListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListAbuseReportsListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn abuse_reports(mut self, value: Vec<ListsAbuseReports>) -> Self {
        self.abuse_reports = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAbuseReportsListsResponse`].
    pub fn build(self) -> Result<ListAbuseReportsListsResponse, BuildError> {
        Ok(ListAbuseReportsListsResponse {
            links: self.links,
            abuse_reports: self.abuse_reports,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
