pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The settings specific to A/B test campaigns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateRequestVariateSettings {
    /// The possible from names. The number of from_names provided must match the number of reply_to_addresses. If no from_names are provided, settings.from_name will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_names: Option<Vec<String>>,
    /// The possible reply-to addresses. The number of reply_to_addresses provided must match the number of from_names. If no reply_to_addresses are provided, settings.reply_to will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
    /// The possible send times to test. The times provided should be in the format YYYY-MM-DD HH:MM:SS or ISO 8601 date-time format. If send_times are provided to test, the test_size will be set to 100% and winner_criteria will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_times: Option<Vec<UpdateRequestVariateSettingsSendTimesItem>>,
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
    pub winner_criteria: Option<UpdateRequestVariateSettingsWinnerCriteria>,
}

impl UpdateRequestVariateSettings {
    pub fn builder() -> UpdateRequestVariateSettingsBuilder {
        <UpdateRequestVariateSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRequestVariateSettingsBuilder {
    from_names: Option<Vec<String>>,
    reply_to_addresses: Option<Vec<String>>,
    send_times: Option<Vec<UpdateRequestVariateSettingsSendTimesItem>>,
    subject_lines: Option<Vec<String>>,
    test_size: Option<i64>,
    wait_time: Option<i64>,
    winner_criteria: Option<UpdateRequestVariateSettingsWinnerCriteria>,
}

impl UpdateRequestVariateSettingsBuilder {
    pub fn from_names(mut self, value: Vec<String>) -> Self {
        self.from_names = Some(value);
        self
    }

    pub fn reply_to_addresses(mut self, value: Vec<String>) -> Self {
        self.reply_to_addresses = Some(value);
        self
    }

    pub fn send_times(mut self, value: Vec<UpdateRequestVariateSettingsSendTimesItem>) -> Self {
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

    pub fn winner_criteria(mut self, value: UpdateRequestVariateSettingsWinnerCriteria) -> Self {
        self.winner_criteria = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateRequestVariateSettings`].
    pub fn build(self) -> Result<UpdateRequestVariateSettings, BuildError> {
        Ok(UpdateRequestVariateSettings {
            from_names: self.from_names,
            reply_to_addresses: self.reply_to_addresses,
            send_times: self.send_times,
            subject_lines: self.subject_lines,
            test_size: self.test_size,
            wait_time: self.wait_time,
            winner_criteria: self.winner_criteria,
        })
    }
}
