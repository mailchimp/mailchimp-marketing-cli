pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-members
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMembersQueryRequest {
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
    /// The email type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    /// The subscriber's status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListMembersRequestStatus>,
    /// Restrict results to subscribers who opted-in after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_timestamp_opt: Option<String>,
    /// Restrict results to subscribers who opted-in before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_timestamp_opt: Option<String>,
    /// Restrict results to subscribers whose information changed after the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_last_changed: Option<String>,
    /// Restrict results to subscribers whose information changed before the set timeframe. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_last_changed: Option<String>,
    /// A unique identifier for the email address across all Mailchimp lists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_email_id: Option<String>,
    /// A filter to return only the list's VIP members. Passing `true` will restrict results to VIP list members, passing `false` will return all list members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_only: Option<bool>,
    /// The unique id for the interest category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_category_id: Option<String>,
    /// Used to filter list members by interests. Must be accompanied by interest_category_id and interest_match. The value must be a comma separated list of interest ids present for any supplied interest categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_ids: Option<String>,
    /// Used to filter list members by interests. Must be accompanied by interest_category_id and interest_ids. "any" will match a member with any of the interest supplied, "all" will only match members with every interest supplied, and "none" will match members without any of the interest supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_match: Option<ListMembersRequestInterestMatch>,
    /// Returns files sorted by the specified field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ListMembersRequestSortField>,
    /// Determines the order direction for sorted results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<ListMembersRequestSortDir>,
    /// Filter subscribers by those subscribed/unsubscribed/pending/cleaned since last email campaign send. Member status is required to use this filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_last_campaign: Option<bool>,
    /// Filter subscribers by those unsubscribed since a specific date. Using any status other than unsubscribed with this filter will result in an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed_since: Option<String>,
}

impl ListMembersQueryRequest {
    pub fn builder() -> ListMembersQueryRequestBuilder {
        <ListMembersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    email_type: Option<String>,
    status: Option<ListMembersRequestStatus>,
    since_timestamp_opt: Option<String>,
    before_timestamp_opt: Option<String>,
    since_last_changed: Option<String>,
    before_last_changed: Option<String>,
    unique_email_id: Option<String>,
    vip_only: Option<bool>,
    interest_category_id: Option<String>,
    interest_ids: Option<String>,
    interest_match: Option<ListMembersRequestInterestMatch>,
    sort_field: Option<ListMembersRequestSortField>,
    sort_dir: Option<ListMembersRequestSortDir>,
    since_last_campaign: Option<bool>,
    unsubscribed_since: Option<String>,
}

impl ListMembersQueryRequestBuilder {
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

    pub fn email_type(mut self, value: impl Into<String>) -> Self {
        self.email_type = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListMembersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn since_timestamp_opt(mut self, value: impl Into<String>) -> Self {
        self.since_timestamp_opt = Some(value.into());
        self
    }

    pub fn before_timestamp_opt(mut self, value: impl Into<String>) -> Self {
        self.before_timestamp_opt = Some(value.into());
        self
    }

    pub fn since_last_changed(mut self, value: impl Into<String>) -> Self {
        self.since_last_changed = Some(value.into());
        self
    }

    pub fn before_last_changed(mut self, value: impl Into<String>) -> Self {
        self.before_last_changed = Some(value.into());
        self
    }

    pub fn unique_email_id(mut self, value: impl Into<String>) -> Self {
        self.unique_email_id = Some(value.into());
        self
    }

    pub fn vip_only(mut self, value: bool) -> Self {
        self.vip_only = Some(value);
        self
    }

    pub fn interest_category_id(mut self, value: impl Into<String>) -> Self {
        self.interest_category_id = Some(value.into());
        self
    }

    pub fn interest_ids(mut self, value: impl Into<String>) -> Self {
        self.interest_ids = Some(value.into());
        self
    }

    pub fn interest_match(mut self, value: ListMembersRequestInterestMatch) -> Self {
        self.interest_match = Some(value);
        self
    }

    pub fn sort_field(mut self, value: ListMembersRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort_dir(mut self, value: ListMembersRequestSortDir) -> Self {
        self.sort_dir = Some(value);
        self
    }

    pub fn since_last_campaign(mut self, value: bool) -> Self {
        self.since_last_campaign = Some(value);
        self
    }

    pub fn unsubscribed_since(mut self, value: impl Into<String>) -> Self {
        self.unsubscribed_since = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMembersQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListMembersQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListMembersQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListMembersQueryRequest, BuildError> {
        Ok(ListMembersQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            email_type: self.email_type,
            status: self.status,
            since_timestamp_opt: self.since_timestamp_opt,
            before_timestamp_opt: self.before_timestamp_opt,
            since_last_changed: self.since_last_changed,
            before_last_changed: self.before_last_changed,
            unique_email_id: self.unique_email_id,
            vip_only: self.vip_only,
            interest_category_id: self.interest_category_id,
            interest_ids: self.interest_ids,
            interest_match: self.interest_match,
            sort_field: self.sort_field,
            sort_dir: self.sort_dir,
            since_last_campaign: self.since_last_campaign,
            unsubscribed_since: self.unsubscribed_since,
        })
    }
}

