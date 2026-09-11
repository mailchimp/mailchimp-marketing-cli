pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignVariateSettingsCombinationsItem {
    /// The index of `variate_settings.contents` used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_description: Option<i64>,
    /// The index of `variate_settings.from_names` used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<i64>,
    /// Unique ID for the combination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of recipients for this combination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<i64>,
    /// The index of `variate_settings.reply_to_addresses` used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<i64>,
    /// The index of `variate_settings.send_times` used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_time: Option<i64>,
    /// The index of `variate_settings.subject_lines` used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_line: Option<i64>,
}

impl CampaignVariateSettingsCombinationsItem {
    pub fn builder() -> CampaignVariateSettingsCombinationsItemBuilder {
        <CampaignVariateSettingsCombinationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignVariateSettingsCombinationsItemBuilder {
    content_description: Option<i64>,
    from_name: Option<i64>,
    id: Option<String>,
    recipients: Option<i64>,
    reply_to: Option<i64>,
    send_time: Option<i64>,
    subject_line: Option<i64>,
}

impl CampaignVariateSettingsCombinationsItemBuilder {
    pub fn content_description(mut self, value: i64) -> Self {
        self.content_description = Some(value);
        self
    }

    pub fn from_name(mut self, value: i64) -> Self {
        self.from_name = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn recipients(mut self, value: i64) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn reply_to(mut self, value: i64) -> Self {
        self.reply_to = Some(value);
        self
    }

    pub fn send_time(mut self, value: i64) -> Self {
        self.send_time = Some(value);
        self
    }

    pub fn subject_line(mut self, value: i64) -> Self {
        self.subject_line = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignVariateSettingsCombinationsItem`].
    pub fn build(self) -> Result<CampaignVariateSettingsCombinationsItem, BuildError> {
        Ok(CampaignVariateSettingsCombinationsItem {
            content_description: self.content_description,
            from_name: self.from_name,
            id: self.id,
            recipients: self.recipients,
            reply_to: self.reply_to,
            send_time: self.send_time,
            subject_line: self.subject_line,
        })
    }
}
