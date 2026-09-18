pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateMemberRequest {
    /// Email address for a subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
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
    pub location: Option<UpdateMemberRequestLocation>,
    /// The marketing permissions for the subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_permissions: Option<Vec<UpdateMemberRequestMarketingPermissionsItem>>,
    /// A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, UpdateMemberRequestMergeFieldsValue>>,
    /// Subscriber's current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateMemberRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_opt: Option<UpdateMemberRequestTimestampOpt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_signup: Option<UpdateMemberRequestTimestampSignup>,
    /// [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<bool>,
    /// If skip_merge_validation is true, member data will be accepted without merge field values, even if the merge field is usually required. This defaults to false.
    #[serde(skip)]
    pub skip_merge_validation: Option<bool>,
}

impl UpdateMemberRequest {
    pub fn builder() -> UpdateMemberRequestBuilder {
        <UpdateMemberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMemberRequestBuilder {
    email_address: Option<String>,
    email_type: Option<String>,
    interests: Option<HashMap<String, bool>>,
    ip_opt: Option<String>,
    ip_signup: Option<String>,
    language: Option<String>,
    location: Option<UpdateMemberRequestLocation>,
    marketing_permissions: Option<Vec<UpdateMemberRequestMarketingPermissionsItem>>,
    merge_fields: Option<HashMap<String, UpdateMemberRequestMergeFieldsValue>>,
    status: Option<UpdateMemberRequestStatus>,
    timestamp_opt: Option<UpdateMemberRequestTimestampOpt>,
    timestamp_signup: Option<UpdateMemberRequestTimestampSignup>,
    vip: Option<bool>,
    skip_merge_validation: Option<bool>,
}

impl UpdateMemberRequestBuilder {
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

    pub fn location(mut self, value: UpdateMemberRequestLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn marketing_permissions(mut self, value: Vec<UpdateMemberRequestMarketingPermissionsItem>) -> Self {
        self.marketing_permissions = Some(value);
        self
    }

    pub fn merge_fields(mut self, value: HashMap<String, UpdateMemberRequestMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn status(mut self, value: UpdateMemberRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn timestamp_opt(mut self, value: UpdateMemberRequestTimestampOpt) -> Self {
        self.timestamp_opt = Some(value);
        self
    }

    pub fn timestamp_signup(mut self, value: UpdateMemberRequestTimestampSignup) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateMemberRequest`].
    pub fn build(self) -> Result<UpdateMemberRequest, BuildError> {
        Ok(UpdateMemberRequest {
            email_address: self.email_address,
            email_type: self.email_type,
            interests: self.interests,
            ip_opt: self.ip_opt,
            ip_signup: self.ip_signup,
            language: self.language,
            location: self.location,
            marketing_permissions: self.marketing_permissions,
            merge_fields: self.merge_fields,
            status: self.status,
            timestamp_opt: self.timestamp_opt,
            timestamp_signup: self.timestamp_signup,
            vip: self.vip,
            skip_merge_validation: self.skip_merge_validation,
        })
    }
}

