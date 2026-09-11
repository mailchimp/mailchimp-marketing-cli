pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsListQueryRequest {
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
    /// The campaign type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListCampaignsRequestType>,
    /// The status of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListCampaignsRequestStatus>,
    /// Restrict the response to campaigns sent before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub before_send_time: Option<DateTime<FixedOffset>>,
    /// Restrict the response to campaigns sent after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub since_send_time: Option<DateTime<FixedOffset>>,
    /// Restrict the response to campaigns created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub before_create_time: Option<DateTime<FixedOffset>>,
    /// Restrict the response to campaigns created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub since_create_time: Option<DateTime<FixedOffset>>,
    /// The unique id for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The unique folder id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// Retrieve campaigns sent to a particular list member. Member ID is The MD5 hash of the lowercase version of the list member’s email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Returns files sorted by the specified field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ListCampaignsRequestSortField>,
    /// Determines the order direction for sorted results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<ListCampaignsRequestSortDir>,
    /// Return the `resend_shortcut_eligibility` field in the response, which tells you if the campaign is eligible for the various Campaign Resend Shortcuts offered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resend_shortcut_eligibility: Option<bool>,
    /// Return the `resend_shortcut_usage` field in the response.  This includes information about campaigns related by a shortcut.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resend_shortcut_usage: Option<bool>,
}

impl CampaignsListQueryRequest {
    pub fn builder() -> CampaignsListQueryRequestBuilder {
        <CampaignsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsListQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    r#type: Option<ListCampaignsRequestType>,
    status: Option<ListCampaignsRequestStatus>,
    before_send_time: Option<DateTime<FixedOffset>>,
    since_send_time: Option<DateTime<FixedOffset>>,
    before_create_time: Option<DateTime<FixedOffset>>,
    since_create_time: Option<DateTime<FixedOffset>>,
    list_id: Option<String>,
    folder_id: Option<String>,
    member_id: Option<String>,
    sort_field: Option<ListCampaignsRequestSortField>,
    sort_dir: Option<ListCampaignsRequestSortDir>,
    include_resend_shortcut_eligibility: Option<bool>,
    include_resend_shortcut_usage: Option<bool>,
}

impl CampaignsListQueryRequestBuilder {
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

    pub fn r#type(mut self, value: ListCampaignsRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn status(mut self, value: ListCampaignsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn before_send_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before_send_time = Some(value);
        self
    }

    pub fn since_send_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since_send_time = Some(value);
        self
    }

    pub fn before_create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before_create_time = Some(value);
        self
    }

    pub fn since_create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since_create_time = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn sort_field(mut self, value: ListCampaignsRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort_dir(mut self, value: ListCampaignsRequestSortDir) -> Self {
        self.sort_dir = Some(value);
        self
    }

    pub fn include_resend_shortcut_eligibility(mut self, value: bool) -> Self {
        self.include_resend_shortcut_eligibility = Some(value);
        self
    }

    pub fn include_resend_shortcut_usage(mut self, value: bool) -> Self {
        self.include_resend_shortcut_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](CampaignsListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](CampaignsListQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<CampaignsListQueryRequest, BuildError> {
        Ok(CampaignsListQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            r#type: self.r#type,
            status: self.status,
            before_send_time: self.before_send_time,
            since_send_time: self.since_send_time,
            before_create_time: self.before_create_time,
            since_create_time: self.since_create_time,
            list_id: self.list_id,
            folder_id: self.folder_id,
            member_id: self.member_id,
            sort_field: self.sort_field,
            sort_dir: self.sort_dir,
            include_resend_shortcut_eligibility: self.include_resend_shortcut_eligibility,
            include_resend_shortcut_usage: self.include_resend_shortcut_usage,
        })
    }
}

