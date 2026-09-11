pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateListsRequest {
    /// [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_defaults: Option<UpdateListsRequestCampaignDefaults>,
    /// [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<UpdateListsRequestContact>,
    /// Whether or not to require the subscriber to confirm subscription via email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_optin: Option<bool>,
    /// Whether the list supports [multiple formats for emails](https://mailchimp.com/help/audience-settings-and-defaults/). When set to `true`, subscribers can choose whether they want to receive HTML or plain-text emails. When set to `false`, subscribers will receive HTML emails, with a plain-text alternative backup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type_option: Option<bool>,
    /// Whether or not the list has marketing permissions (eg. GDPR) enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_permissions: Option<bool>,
    /// The name of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The email address to send [subscribe notifications](https://mailchimp.com/help/change-subscribe-and-unsubscribe-notifications/) to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_subscribe: Option<String>,
    /// The email address to send [unsubscribe notifications](https://mailchimp.com/help/change-subscribe-and-unsubscribe-notifications/) to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_on_unsubscribe: Option<String>,
    /// The [permission reminder](https://mailchimp.com/help/edit-the-permission-reminder/) for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_reminder: Option<String>,
    /// Whether campaigns for this list use the [Archive Bar](https://mailchimp.com/help/about-email-campaign-archives-and-pages/) in archives by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_archive_bar: Option<bool>,
}

impl UpdateListsRequest {
    pub fn builder() -> UpdateListsRequestBuilder {
        <UpdateListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateListsRequestBuilder {
    campaign_defaults: Option<UpdateListsRequestCampaignDefaults>,
    contact: Option<UpdateListsRequestContact>,
    double_optin: Option<bool>,
    email_type_option: Option<bool>,
    marketing_permissions: Option<bool>,
    name: Option<String>,
    notify_on_subscribe: Option<String>,
    notify_on_unsubscribe: Option<String>,
    permission_reminder: Option<String>,
    use_archive_bar: Option<bool>,
}

impl UpdateListsRequestBuilder {
    pub fn campaign_defaults(mut self, value: UpdateListsRequestCampaignDefaults) -> Self {
        self.campaign_defaults = Some(value);
        self
    }

    pub fn contact(mut self, value: UpdateListsRequestContact) -> Self {
        self.contact = Some(value);
        self
    }

    pub fn double_optin(mut self, value: bool) -> Self {
        self.double_optin = Some(value);
        self
    }

    pub fn email_type_option(mut self, value: bool) -> Self {
        self.email_type_option = Some(value);
        self
    }

    pub fn marketing_permissions(mut self, value: bool) -> Self {
        self.marketing_permissions = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notify_on_subscribe(mut self, value: impl Into<String>) -> Self {
        self.notify_on_subscribe = Some(value.into());
        self
    }

    pub fn notify_on_unsubscribe(mut self, value: impl Into<String>) -> Self {
        self.notify_on_unsubscribe = Some(value.into());
        self
    }

    pub fn permission_reminder(mut self, value: impl Into<String>) -> Self {
        self.permission_reminder = Some(value.into());
        self
    }

    pub fn use_archive_bar(mut self, value: bool) -> Self {
        self.use_archive_bar = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateListsRequest`].
    pub fn build(self) -> Result<UpdateListsRequest, BuildError> {
        Ok(UpdateListsRequest {
            campaign_defaults: self.campaign_defaults,
            contact: self.contact,
            double_optin: self.double_optin,
            email_type_option: self.email_type_option,
            marketing_permissions: self.marketing_permissions,
            name: self.name,
            notify_on_subscribe: self.notify_on_subscribe,
            notify_on_unsubscribe: self.notify_on_unsubscribe,
            permission_reminder: self.permission_reminder,
            use_archive_bar: self.use_archive_bar,
        })
    }
}

