pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// [A/B Testing](https://mailchimp.com/help/about-ab-tests/) options for a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AbTestingOptions {
    /// For campaigns split on 'From Name', the name for Group A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name_a: Option<String>,
    /// For campaigns split on 'From Name', the name for Group B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name_b: Option<String>,
    /// How we should evaluate a winner. Based on 'opens', 'clicks', or 'manual'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pick_winner: Option<AbTestingOptionsPickWinner>,
    /// For campaigns split on 'From Name', the reply-to address for Group A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_email_a: Option<String>,
    /// For campaigns split on 'From Name', the reply-to address for Group B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_email_b: Option<String>,
    /// The send time for Group A.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub send_time_a: Option<DateTime<FixedOffset>>,
    /// The send time for Group B.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub send_time_b: Option<DateTime<FixedOffset>>,
    /// The send time for the winning version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_time_winner: Option<String>,
    /// The size of the split groups. Campaigns split based on 'schedule' are forced to have a 50/50 split. Valid split integers are between 1-50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_size: Option<i64>,
    /// The type of AB split to run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_test: Option<AbTestingOptionsSplitTest>,
    /// For campaigns split on 'Subject Line', the subject line for Group A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_a: Option<String>,
    /// For campaigns split on 'Subject Line', the subject line for Group B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_b: Option<String>,
    /// The amount of time to wait before picking a winner. This cannot be changed after a campaign is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<i64>,
    /// How unit of time for measuring the winner ('hours' or 'days'). This cannot be changed after a campaign is sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_units: Option<AbTestingOptionsWaitUnits>,
}

impl AbTestingOptions {
    pub fn builder() -> AbTestingOptionsBuilder {
        <AbTestingOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AbTestingOptionsBuilder {
    from_name_a: Option<String>,
    from_name_b: Option<String>,
    pick_winner: Option<AbTestingOptionsPickWinner>,
    reply_email_a: Option<String>,
    reply_email_b: Option<String>,
    send_time_a: Option<DateTime<FixedOffset>>,
    send_time_b: Option<DateTime<FixedOffset>>,
    send_time_winner: Option<String>,
    split_size: Option<i64>,
    split_test: Option<AbTestingOptionsSplitTest>,
    subject_a: Option<String>,
    subject_b: Option<String>,
    wait_time: Option<i64>,
    wait_units: Option<AbTestingOptionsWaitUnits>,
}

impl AbTestingOptionsBuilder {
    pub fn from_name_a(mut self, value: impl Into<String>) -> Self {
        self.from_name_a = Some(value.into());
        self
    }

    pub fn from_name_b(mut self, value: impl Into<String>) -> Self {
        self.from_name_b = Some(value.into());
        self
    }

    pub fn pick_winner(mut self, value: AbTestingOptionsPickWinner) -> Self {
        self.pick_winner = Some(value);
        self
    }

    pub fn reply_email_a(mut self, value: impl Into<String>) -> Self {
        self.reply_email_a = Some(value.into());
        self
    }

    pub fn reply_email_b(mut self, value: impl Into<String>) -> Self {
        self.reply_email_b = Some(value.into());
        self
    }

    pub fn send_time_a(mut self, value: DateTime<FixedOffset>) -> Self {
        self.send_time_a = Some(value);
        self
    }

    pub fn send_time_b(mut self, value: DateTime<FixedOffset>) -> Self {
        self.send_time_b = Some(value);
        self
    }

    pub fn send_time_winner(mut self, value: impl Into<String>) -> Self {
        self.send_time_winner = Some(value.into());
        self
    }

    pub fn split_size(mut self, value: i64) -> Self {
        self.split_size = Some(value);
        self
    }

    pub fn split_test(mut self, value: AbTestingOptionsSplitTest) -> Self {
        self.split_test = Some(value);
        self
    }

    pub fn subject_a(mut self, value: impl Into<String>) -> Self {
        self.subject_a = Some(value.into());
        self
    }

    pub fn subject_b(mut self, value: impl Into<String>) -> Self {
        self.subject_b = Some(value.into());
        self
    }

    pub fn wait_time(mut self, value: i64) -> Self {
        self.wait_time = Some(value);
        self
    }

    pub fn wait_units(mut self, value: AbTestingOptionsWaitUnits) -> Self {
        self.wait_units = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AbTestingOptions`].
    pub fn build(self) -> Result<AbTestingOptions, BuildError> {
        Ok(AbTestingOptions {
            from_name_a: self.from_name_a,
            from_name_b: self.from_name_b,
            pick_winner: self.pick_winner,
            reply_email_a: self.reply_email_a,
            reply_email_b: self.reply_email_b,
            send_time_a: self.send_time_a,
            send_time_b: self.send_time_b,
            send_time_winner: self.send_time_winner,
            split_size: self.split_size,
            split_test: self.split_test,
            subject_a: self.subject_a,
            subject_b: self.subject_b,
            wait_time: self.wait_time,
            wait_units: self.wait_units,
        })
    }
}
