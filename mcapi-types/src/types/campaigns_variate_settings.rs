pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The settings specific to A/B test campaigns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsVariateSettings {
    /// Combinations of possible variables used to build emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combinations: Option<Vec<CampaignsVariateSettingsCombinationsItem>>,
    /// Descriptions of possible email contents. To set campaign contents, make a PUT request to /campaigns/{campaign_id}/content with the field 'variate_contents'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<String>>,
    /// The possible from names. The number of from_names provided must match the number of reply_to_addresses. If no from_names are provided, settings.from_name will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_names: Option<Vec<String>>,
    /// The possible reply-to addresses. The number of reply_to_addresses provided must match the number of from_names. If no reply_to_addresses are provided, settings.reply_to will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
    /// The possible send times to test. The times provided should be in the format YYYY-MM-DD HH:MM:SS. If send_times are provided to test, the test_size will be set to 100% and winner_criteria will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_times: Option<Vec<DateTime<FixedOffset>>>,
    /// The possible subject lines to test. If no subject lines are provided, settings.subject_line will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_lines: Option<Vec<String>>,
    /// The percentage of recipients to send the test combinations to, must be a value between 10 and 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_size: Option<i64>,
    /// The number of minutes to wait before choosing the winning campaign. The value of wait_time must be greater than 0 and in whole hours, specified in minutes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<i64>,
    /// The combination that performs the best. This may be determined automatically by click rate, open rate, or total revenue -- or you may choose manually based on the reporting data you find the most valuable. For Multivariate Campaigns testing send_time, winner_criteria is ignored. For Multivariate Campaigns with 'manual' as the winner_criteria, the winner must be chosen in the Mailchimp web application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winner_criteria: Option<CampaignsVariateSettingsWinnerCriteria>,
    /// ID of the campaign that was sent to the remaining recipients based on the winning combination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_campaign_id: Option<String>,
    /// ID for the winning combination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_combination_id: Option<String>,
}

impl CampaignsVariateSettings {
    pub fn builder() -> CampaignsVariateSettingsBuilder {
        <CampaignsVariateSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsVariateSettingsBuilder {
    combinations: Option<Vec<CampaignsVariateSettingsCombinationsItem>>,
    contents: Option<Vec<String>>,
    from_names: Option<Vec<String>>,
    reply_to_addresses: Option<Vec<String>>,
    send_times: Option<Vec<DateTime<FixedOffset>>>,
    subject_lines: Option<Vec<String>>,
    test_size: Option<i64>,
    wait_time: Option<i64>,
    winner_criteria: Option<CampaignsVariateSettingsWinnerCriteria>,
    winning_campaign_id: Option<String>,
    winning_combination_id: Option<String>,
}

impl CampaignsVariateSettingsBuilder {
    pub fn combinations(mut self, value: Vec<CampaignsVariateSettingsCombinationsItem>) -> Self {
        self.combinations = Some(value);
        self
    }

    pub fn contents(mut self, value: Vec<String>) -> Self {
        self.contents = Some(value);
        self
    }

    pub fn from_names(mut self, value: Vec<String>) -> Self {
        self.from_names = Some(value);
        self
    }

    pub fn reply_to_addresses(mut self, value: Vec<String>) -> Self {
        self.reply_to_addresses = Some(value);
        self
    }

    pub fn send_times(mut self, value: Vec<DateTime<FixedOffset>>) -> Self {
        self.send_times = Some(value);
        self
    }

    pub fn subject_lines(mut self, value: Vec<String>) -> Self {
        self.subject_lines = Some(value);
        self
    }

    pub fn test_size(mut self, value: i64) -> Self {
        self.test_size = Some(value);
        self
    }

    pub fn wait_time(mut self, value: i64) -> Self {
        self.wait_time = Some(value);
        self
    }

    pub fn winner_criteria(mut self, value: CampaignsVariateSettingsWinnerCriteria) -> Self {
        self.winner_criteria = Some(value);
        self
    }

    pub fn winning_campaign_id(mut self, value: impl Into<String>) -> Self {
        self.winning_campaign_id = Some(value.into());
        self
    }

    pub fn winning_combination_id(mut self, value: impl Into<String>) -> Self {
        self.winning_combination_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CampaignsVariateSettings`].
    pub fn build(self) -> Result<CampaignsVariateSettings, BuildError> {
        Ok(CampaignsVariateSettings {
            combinations: self.combinations,
            contents: self.contents,
            from_names: self.from_names,
            reply_to_addresses: self.reply_to_addresses,
            send_times: self.send_times,
            subject_lines: self.subject_lines,
            test_size: self.test_size,
            wait_time: self.wait_time,
            winner_criteria: self.winner_criteria,
            winning_campaign_id: self.winning_campaign_id,
            winning_combination_id: self.winning_combination_id,
        })
    }
}
