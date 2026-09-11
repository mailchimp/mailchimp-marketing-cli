pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscriberList {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<SubscriberListLinksItem>>,
    /// The list's [Email Beamer](https://mailchimp.com/help/use-email-beamer-to-create-a-campaign/) address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beamer_address: Option<String>,
    /// [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_defaults: Option<SubscriberListCampaignDefaults>,
    /// [Contact information displayed in campaign footers](https://mailchimp.com/help/about-campaign-footers/) to comply with international spam laws.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<SubscriberListContact>,
    /// The date and time that this list was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub date_created: Option<DateTime<FixedOffset>>,
    /// Whether or not to require the subscriber to confirm subscription via email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_optin: Option<bool>,
    /// Whether the list supports [multiple formats for emails](https://mailchimp.com/help/audience-settings-and-defaults/). When set to `true`, subscribers can choose whether they want to receive HTML or plain-text emails. When set to `false`, subscribers will receive HTML emails, with a plain-text alternative backup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type_option: Option<bool>,
    /// Whether or not this list has a welcome automation connected. Welcome Automations: welcomeSeries, singleWelcome, emailFollowup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_welcome: Option<bool>,
    /// A string that uniquely identifies this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// An auto-generated activity score for the list (0-5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_rating: Option<i64>,
    /// Whether or not the list has marketing permissions (eg. GDPR) enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_permissions: Option<bool>,
    /// Any list-specific modules installed for this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<String>>,
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
    /// Stats for the list. Many of these are cached for at least five minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<SubscriberListStats>,
    /// The full version of this list's subscribe form (host will vary).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_url_long: Option<String>,
    /// Our [url shortened](https://mailchimp.com/help/share-your-signup-form/) version of this list's subscribe form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_url_short: Option<String>,
    /// Whether campaigns for this list use the [Archive Bar](https://mailchimp.com/help/about-email-campaign-archives-and-pages/) in archives by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_archive_bar: Option<bool>,
    /// Legacy - visibility settings are no longer used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<SubscriberListVisibility>,
    /// The ID used in the Mailchimp web application. View this list in your Mailchimp account at `https://{dc}.admin.mailchimp.com/lists/members/?id={web_id}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<i64>,
}

impl SubscriberList {
    pub fn builder() -> SubscriberListBuilder {
        <SubscriberListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriberListBuilder {
    links: Option<Vec<SubscriberListLinksItem>>,
    beamer_address: Option<String>,
    campaign_defaults: Option<SubscriberListCampaignDefaults>,
    contact: Option<SubscriberListContact>,
    date_created: Option<DateTime<FixedOffset>>,
    double_optin: Option<bool>,
    email_type_option: Option<bool>,
    has_welcome: Option<bool>,
    id: Option<String>,
    list_rating: Option<i64>,
    marketing_permissions: Option<bool>,
    modules: Option<Vec<String>>,
    name: Option<String>,
    notify_on_subscribe: Option<String>,
    notify_on_unsubscribe: Option<String>,
    permission_reminder: Option<String>,
    stats: Option<SubscriberListStats>,
    subscribe_url_long: Option<String>,
    subscribe_url_short: Option<String>,
    use_archive_bar: Option<bool>,
    visibility: Option<SubscriberListVisibility>,
    web_id: Option<i64>,
}

impl SubscriberListBuilder {
    pub fn links(mut self, value: Vec<SubscriberListLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn beamer_address(mut self, value: impl Into<String>) -> Self {
        self.beamer_address = Some(value.into());
        self
    }

    pub fn campaign_defaults(mut self, value: SubscriberListCampaignDefaults) -> Self {
        self.campaign_defaults = Some(value);
        self
    }

    pub fn contact(mut self, value: SubscriberListContact) -> Self {
        self.contact = Some(value);
        self
    }

    pub fn date_created(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date_created = Some(value);
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

    pub fn has_welcome(mut self, value: bool) -> Self {
        self.has_welcome = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn list_rating(mut self, value: i64) -> Self {
        self.list_rating = Some(value);
        self
    }

    pub fn marketing_permissions(mut self, value: bool) -> Self {
        self.marketing_permissions = Some(value);
        self
    }

    pub fn modules(mut self, value: Vec<String>) -> Self {
        self.modules = Some(value);
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

    pub fn stats(mut self, value: SubscriberListStats) -> Self {
        self.stats = Some(value);
        self
    }

    pub fn subscribe_url_long(mut self, value: impl Into<String>) -> Self {
        self.subscribe_url_long = Some(value.into());
        self
    }

    pub fn subscribe_url_short(mut self, value: impl Into<String>) -> Self {
        self.subscribe_url_short = Some(value.into());
        self
    }

    pub fn use_archive_bar(mut self, value: bool) -> Self {
        self.use_archive_bar = Some(value);
        self
    }

    pub fn visibility(mut self, value: SubscriberListVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    pub fn web_id(mut self, value: i64) -> Self {
        self.web_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriberList`].
    pub fn build(self) -> Result<SubscriberList, BuildError> {
        Ok(SubscriberList {
            links: self.links,
            beamer_address: self.beamer_address,
            campaign_defaults: self.campaign_defaults,
            contact: self.contact,
            date_created: self.date_created,
            double_optin: self.double_optin,
            email_type_option: self.email_type_option,
            has_welcome: self.has_welcome,
            id: self.id,
            list_rating: self.list_rating,
            marketing_permissions: self.marketing_permissions,
            modules: self.modules,
            name: self.name,
            notify_on_subscribe: self.notify_on_subscribe,
            notify_on_unsubscribe: self.notify_on_unsubscribe,
            permission_reminder: self.permission_reminder,
            stats: self.stats,
            subscribe_url_long: self.subscribe_url_long,
            subscribe_url_short: self.subscribe_url_short,
            use_archive_bar: self.use_archive_bar,
            visibility: self.visibility,
            web_id: self.web_id,
        })
    }
}
