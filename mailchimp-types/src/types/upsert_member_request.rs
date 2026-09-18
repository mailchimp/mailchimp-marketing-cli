pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpsertMemberRequest {
    /// Email address for a subscriber. This value is required only if the email address is not already present on the list.
    #[serde(default)]
    pub email_address: String,
    /// Type of email this member asked to get ('html' or 'text').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    /// The key of this object's properties is the ID of the interest in question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<HashMap<String, bool>>,
    /// The IP address the subscriber used to confirm their opt-in status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_opt: Option<String>,
    /// IP address the subscriber signed up from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_signup: Option<String>,
    /// If set/detected, the [subscriber's language](https://mailchimp.com/help/view-and-edit-contact-languages/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Subscriber location information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<UpsertMemberRequestLocation>,
    /// The marketing permissions for the subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_permissions: Option<Vec<UpsertMemberRequestMarketingPermissionsItem>>,
    /// A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, UpsertMemberRequestMergeFieldsValue>>,
    /// Subscriber's current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpsertMemberRequestStatus>,
    /// Subscriber's status. This value is required only if the email address is not already present on the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_if_new: Option<UpsertMemberRequestStatusIfNew>,
    /// The tags that are associated with a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_opt: Option<UpsertMemberRequestTimestampOpt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_signup: Option<UpsertMemberRequestTimestampSignup>,
    /// [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<bool>,
    /// If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    #[serde(skip)]
    pub skip_merge_validation: Option<bool>,
}

impl UpsertMemberRequest {
    pub fn builder() -> UpsertMemberRequestBuilder {
        <UpsertMemberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertMemberRequestBuilder {
    email_address: Option<String>,
    email_type: Option<String>,
    interests: Option<HashMap<String, bool>>,
    ip_opt: Option<String>,
    ip_signup: Option<String>,
    language: Option<String>,
    location: Option<UpsertMemberRequestLocation>,
    marketing_permissions: Option<Vec<UpsertMemberRequestMarketingPermissionsItem>>,
    merge_fields: Option<HashMap<String, UpsertMemberRequestMergeFieldsValue>>,
    status: Option<UpsertMemberRequestStatus>,
    status_if_new: Option<UpsertMemberRequestStatusIfNew>,
    tags: Option<Vec<String>>,
    timestamp_opt: Option<UpsertMemberRequestTimestampOpt>,
    timestamp_signup: Option<UpsertMemberRequestTimestampSignup>,
    vip: Option<bool>,
    skip_merge_validation: Option<bool>,
}

impl UpsertMemberRequestBuilder {
    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn email_type(mut self, value: impl Into<String>) -> Self {
        self.email_type = Some(value.into());
        self
    }

    pub fn interests(mut self, value: HashMap<String, bool>) -> Self {
        self.interests = Some(value);
        self
    }

    pub fn ip_opt(mut self, value: impl Into<String>) -> Self {
        self.ip_opt = Some(value.into());
        self
    }

    pub fn ip_signup(mut self, value: impl Into<String>) -> Self {
        self.ip_signup = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn location(mut self, value: UpsertMemberRequestLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn marketing_permissions(mut self, value: Vec<UpsertMemberRequestMarketingPermissionsItem>) -> Self {
        self.marketing_permissions = Some(value);
        self
    }

    pub fn merge_fields(mut self, value: HashMap<String, UpsertMemberRequestMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn status(mut self, value: UpsertMemberRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_if_new(mut self, value: UpsertMemberRequestStatusIfNew) -> Self {
        self.status_if_new = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn timestamp_opt(mut self, value: UpsertMemberRequestTimestampOpt) -> Self {
        self.timestamp_opt = Some(value);
        self
    }

    pub fn timestamp_signup(mut self, value: UpsertMemberRequestTimestampSignup) -> Self {
        self.timestamp_signup = Some(value);
        self
    }

    pub fn vip(mut self, value: bool) -> Self {
        self.vip = Some(value);
        self
    }

    pub fn skip_merge_validation(mut self, value: bool) -> Self {
        self.skip_merge_validation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertMemberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_address`](UpsertMemberRequestBuilder::email_address)
    pub fn build(self) -> Result<UpsertMemberRequest, BuildError> {
        Ok(UpsertMemberRequest {
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
            email_type: self.email_type,
            interests: self.interests,
            ip_opt: self.ip_opt,
            ip_signup: self.ip_signup,
            language: self.language,
            location: self.location,
            marketing_permissions: self.marketing_permissions,
            merge_fields: self.merge_fields,
            status: self.status,
            status_if_new: self.status_if_new,
            tags: self.tags,
            timestamp_opt: self.timestamp_opt,
            timestamp_signup: self.timestamp_signup,
            vip: self.vip,
            skip_merge_validation: self.skip_merge_validation,
        })
    }
}

