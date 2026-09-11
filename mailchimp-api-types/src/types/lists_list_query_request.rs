pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListsListQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The number of records to return. Default value is 10. Maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Restrict response to lists created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date_created: Option<String>,
    /// Restrict results to lists created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_date_created: Option<String>,
    /// Restrict results to lists created before the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_campaign_last_sent: Option<String>,
    /// Restrict results to lists created after the last campaign send date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_campaign_last_sent: Option<String>,
    /// Restrict results to lists that include a specific subscriber's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Returns files sorted by the specified field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ListListsRequestSortField>,
    /// Determines the order direction for sorted results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<ListListsRequestSortDir>,
    /// Restrict results to lists that contain an active, connected, undeleted ecommerce store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_ecommerce_store: Option<bool>,
    /// Deprecated. Return the total_contacts field in the stats response, which contains an approximate count of subscribed, unsubscribed, and transactional contacts. For a complete audience contact count, use the /audiences endpoint instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total_contacts: Option<bool>,
}

impl ListsListQueryRequest {
    pub fn builder() -> ListsListQueryRequestBuilder {
        <ListsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListsListQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    before_date_created: Option<String>,
    since_date_created: Option<String>,
    before_campaign_last_sent: Option<String>,
    since_campaign_last_sent: Option<String>,
    email: Option<String>,
    sort_field: Option<ListListsRequestSortField>,
    sort_dir: Option<ListListsRequestSortDir>,
    has_ecommerce_store: Option<bool>,
    include_total_contacts: Option<bool>,
}

impl ListsListQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn before_date_created(mut self, value: impl Into<String>) -> Self {
        self.before_date_created = Some(value.into());
        self
    }

    pub fn since_date_created(mut self, value: impl Into<String>) -> Self {
        self.since_date_created = Some(value.into());
        self
    }

    pub fn before_campaign_last_sent(mut self, value: impl Into<String>) -> Self {
        self.before_campaign_last_sent = Some(value.into());
        self
    }

    pub fn since_campaign_last_sent(mut self, value: impl Into<String>) -> Self {
        self.since_campaign_last_sent = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn sort_field(mut self, value: ListListsRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort_dir(mut self, value: ListListsRequestSortDir) -> Self {
        self.sort_dir = Some(value);
        self
    }

    pub fn has_ecommerce_store(mut self, value: bool) -> Self {
        self.has_ecommerce_store = Some(value);
        self
    }

    pub fn include_total_contacts(mut self, value: bool) -> Self {
        self.include_total_contacts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListsListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListsListQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListsListQueryRequest, BuildError> {
        Ok(ListsListQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            before_date_created: self.before_date_created,
            since_date_created: self.since_date_created,
            before_campaign_last_sent: self.before_campaign_last_sent,
            since_campaign_last_sent: self.since_campaign_last_sent,
            email: self.email,
            sort_field: self.sort_field,
            sort_dir: self.sort_dir,
            has_ecommerce_store: self.has_ecommerce_store,
            include_total_contacts: self.include_total_contacts,
        })
    }
}

