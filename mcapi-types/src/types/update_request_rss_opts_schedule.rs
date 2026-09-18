pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The schedule for sending the RSS Campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateRequestRssOptsSchedule {
    /// The days of the week to send a daily RSS Campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_send: Option<UpdateRequestRssOptsScheduleDailySend>,
    /// The hour to send the campaign in local time. Acceptable hours are 0-23. For example, '4' would be 4am in [your account's default time zone](https://mailchimp.com/help/set-account-details/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<i64>,
    /// The day of the month to send a monthly RSS Campaign. Acceptable days are 0-31, where '0' is always the last day of a month. Months with fewer than the selected number of days will not have an RSS campaign sent out that day. For example, RSS Campaigns set to send on the 30th will not go out in February.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub monthly_send_date: Option<f64>,
    /// The day of the week to send a weekly RSS Campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_send_day: Option<UpdateRequestRssOptsScheduleWeeklySendDay>,
}

impl UpdateRequestRssOptsSchedule {
    pub fn builder() -> UpdateRequestRssOptsScheduleBuilder {
        <UpdateRequestRssOptsScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateRequestRssOptsScheduleBuilder {
    daily_send: Option<UpdateRequestRssOptsScheduleDailySend>,
    hour: Option<i64>,
    monthly_send_date: Option<f64>,
    weekly_send_day: Option<UpdateRequestRssOptsScheduleWeeklySendDay>,
}

impl UpdateRequestRssOptsScheduleBuilder {
    pub fn daily_send(mut self, value: UpdateRequestRssOptsScheduleDailySend) -> Self {
        self.daily_send = Some(value);
        self
    }

    pub fn hour(mut self, value: i64) -> Self {
        self.hour = Some(value);
        self
    }

    pub fn monthly_send_date(mut self, value: f64) -> Self {
        self.monthly_send_date = Some(value);
        self
    }

    pub fn weekly_send_day(mut self, value: UpdateRequestRssOptsScheduleWeeklySendDay) -> Self {
        self.weekly_send_day = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateRequestRssOptsSchedule`].
    pub fn build(self) -> Result<UpdateRequestRssOptsSchedule, BuildError> {
        Ok(UpdateRequestRssOptsSchedule {
            daily_send: self.daily_send,
            hour: self.hour,
            monthly_send_date: self.monthly_send_date,
            weekly_send_day: self.weekly_send_day,
        })
    }
}
