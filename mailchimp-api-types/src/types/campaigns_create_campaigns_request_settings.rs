pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The settings for your campaign, including subject, from name, reply-to address, and more.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCampaignsRequestSettings {
    /// Whether Mailchimp [authenticated](https://mailchimp.com/help/about-email-authentication/) the campaign. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticate: Option<bool>,
    /// An array of [Facebook](https://mailchimp.com/help/connect-or-disconnect-the-facebook-integration/) page ids to auto-post to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_fb_post: Option<Vec<String>>,
    /// Automatically append Mailchimp's [default footer](https://mailchimp.com/help/about-campaign-footers/) to the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_footer: Option<bool>,
    /// Automatically tweet a link to the [campaign archive](https://mailchimp.com/help/about-email-campaign-archives-and-pages/) page when the campaign is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tweet: Option<bool>,
    /// Allows Facebook comments on the campaign (also force-enables the Campaign Archive toolbar). Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fb_comments: Option<bool>,
    /// If the campaign is listed in a folder, the id for that folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// The 'from' name on the campaign (not an email address).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Automatically inline the CSS included with the campaign content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_css: Option<bool>,
    /// The preview text for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    /// The reply-to email address for the campaign. Note: while this field is not required for campaign creation, it is required for sending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// The subject line for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_line: Option<String>,
    /// The id of the template to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<i64>,
    /// The title of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The campaign's custom 'To' name. Typically the first name [audience field](https://mailchimp.com/help/getting-started-with-merge-tags/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_name: Option<String>,
    /// Use Mailchimp Conversation feature to manage out-of-office replies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_conversation: Option<bool>,
}

impl CreateCampaignsRequestSettings {
    pub fn builder() -> CreateCampaignsRequestSettingsBuilder {
        <CreateCampaignsRequestSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCampaignsRequestSettingsBuilder {
    authenticate: Option<bool>,
    auto_fb_post: Option<Vec<String>>,
    auto_footer: Option<bool>,
    auto_tweet: Option<bool>,
    fb_comments: Option<bool>,
    folder_id: Option<String>,
    from_name: Option<String>,
    inline_css: Option<bool>,
    preview_text: Option<String>,
    reply_to: Option<String>,
    subject_line: Option<String>,
    template_id: Option<i64>,
    title: Option<String>,
    to_name: Option<String>,
    use_conversation: Option<bool>,
}

impl CreateCampaignsRequestSettingsBuilder {
    pub fn authenticate(mut self, value: bool) -> Self {
        self.authenticate = Some(value);
        self
    }

    pub fn auto_fb_post(mut self, value: Vec<String>) -> Self {
        self.auto_fb_post = Some(value);
        self
    }

    pub fn auto_footer(mut self, value: bool) -> Self {
        self.auto_footer = Some(value);
        self
    }

    pub fn auto_tweet(mut self, value: bool) -> Self {
        self.auto_tweet = Some(value);
        self
    }

    pub fn fb_comments(mut self, value: bool) -> Self {
        self.fb_comments = Some(value);
        self
    }

    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn from_name(mut self, value: impl Into<String>) -> Self {
        self.from_name = Some(value.into());
        self
    }

    pub fn inline_css(mut self, value: bool) -> Self {
        self.inline_css = Some(value);
        self
    }

    pub fn preview_text(mut self, value: impl Into<String>) -> Self {
        self.preview_text = Some(value.into());
        self
    }

    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
        self
    }

    pub fn subject_line(mut self, value: impl Into<String>) -> Self {
        self.subject_line = Some(value.into());
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn to_name(mut self, value: impl Into<String>) -> Self {
        self.to_name = Some(value.into());
        self
    }

    pub fn use_conversation(mut self, value: bool) -> Self {
        self.use_conversation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCampaignsRequestSettings`].
    pub fn build(self) -> Result<CreateCampaignsRequestSettings, BuildError> {
        Ok(CreateCampaignsRequestSettings {
            authenticate: self.authenticate,
            auto_fb_post: self.auto_fb_post,
            auto_footer: self.auto_footer,
            auto_tweet: self.auto_tweet,
            fb_comments: self.fb_comments,
            folder_id: self.folder_id,
            from_name: self.from_name,
            inline_css: self.inline_css,
            preview_text: self.preview_text,
            reply_to: self.reply_to,
            subject_line: self.subject_line,
            template_id: self.template_id,
            title: self.title,
            to_name: self.to_name,
            use_conversation: self.use_conversation,
        })
    }
}
