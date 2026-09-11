pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Stats for the list. Many of these are cached for at least five minutes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscriberListStats {
    /// The average number of subscriptions per month for the list (not returned if we haven't calculated it yet).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_sub_rate: Option<f64>,
    /// The average number of unsubscriptions per month for the list (not returned if we haven't calculated it yet).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub avg_unsub_rate: Option<f64>,
    /// The number of campaigns in any status that use this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_count: Option<i64>,
    /// The date and time the last campaign was sent to this list in ISO 8601 format. This is updated when a campaign is sent to 10 or more recipients.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub campaign_last_sent: Option<DateTime<FixedOffset>>,
    /// The number of members cleaned from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleaned_count: Option<i64>,
    /// The number of members cleaned from the list since the last campaign was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleaned_count_since_send: Option<i64>,
    /// The average click rate (a percentage represented as a number between 0 and 100) per campaign for the list (not returned if we haven't calculated it yet).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_rate: Option<f64>,
    /// The date and time of the last time someone subscribed to this list in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_sub_date: Option<DateTime<FixedOffset>>,
    /// The date and time of the last time someone unsubscribed from this list in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_unsub_date: Option<DateTime<FixedOffset>>,
    /// The number of active members in the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i64>,
    /// The number of active members in the list since the last campaign was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count_since_send: Option<i64>,
    /// The number of merge fields ([audience field](https://mailchimp.com/help/getting-started-with-merge-tags/)) for this list (doesn't include EMAIL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_field_count: Option<i64>,
    /// The average open rate (a percentage represented as a number between 0 and 100) per campaign for the list (not returned if we haven't calculated it yet).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub open_rate: Option<f64>,
    /// The target number of subscriptions per month for the list to keep it growing (not returned if we haven't calculated it yet).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub target_sub_rate: Option<f64>,
    /// An approximate count of subscribed, unsubscribed, and transactional contacts in the list. Does not include cleaned, archived, pending, or contacts that need to be reconfirmed. Requires the (deprecated) include_total_contacts query parameter to be included; for a complete audience contact count, use the /audiences endpoint instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_contacts: Option<i64>,
    /// The number of members who have unsubscribed from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_count: Option<i64>,
    /// The number of members who have unsubscribed since the last campaign was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_count_since_send: Option<i64>,
}

impl SubscriberListStats {
    pub fn builder() -> SubscriberListStatsBuilder {
        <SubscriberListStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriberListStatsBuilder {
    avg_sub_rate: Option<f64>,
    avg_unsub_rate: Option<f64>,
    campaign_count: Option<i64>,
    campaign_last_sent: Option<DateTime<FixedOffset>>,
    cleaned_count: Option<i64>,
    cleaned_count_since_send: Option<i64>,
    click_rate: Option<f64>,
    last_sub_date: Option<DateTime<FixedOffset>>,
    last_unsub_date: Option<DateTime<FixedOffset>>,
    member_count: Option<i64>,
    member_count_since_send: Option<i64>,
    merge_field_count: Option<i64>,
    open_rate: Option<f64>,
    target_sub_rate: Option<f64>,
    total_contacts: Option<i64>,
    unsubscribe_count: Option<i64>,
    unsubscribe_count_since_send: Option<i64>,
}

impl SubscriberListStatsBuilder {
    pub fn avg_sub_rate(mut self, value: f64) -> Self {
        self.avg_sub_rate = Some(value);
        self
    }

    pub fn avg_unsub_rate(mut self, value: f64) -> Self {
        self.avg_unsub_rate = Some(value);
        self
    }

    pub fn campaign_count(mut self, value: i64) -> Self {
        self.campaign_count = Some(value);
        self
    }

    pub fn campaign_last_sent(mut self, value: DateTime<FixedOffset>) -> Self {
        self.campaign_last_sent = Some(value);
        self
    }

    pub fn cleaned_count(mut self, value: i64) -> Self {
        self.cleaned_count = Some(value);
        self
    }

    pub fn cleaned_count_since_send(mut self, value: i64) -> Self {
        self.cleaned_count_since_send = Some(value);
        self
    }

    pub fn click_rate(mut self, value: f64) -> Self {
        self.click_rate = Some(value);
        self
    }

    pub fn last_sub_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_sub_date = Some(value);
        self
    }

    pub fn last_unsub_date(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_unsub_date = Some(value);
        self
    }

    pub fn member_count(mut self, value: i64) -> Self {
        self.member_count = Some(value);
        self
    }

    pub fn member_count_since_send(mut self, value: i64) -> Self {
        self.member_count_since_send = Some(value);
        self
    }

    pub fn merge_field_count(mut self, value: i64) -> Self {
        self.merge_field_count = Some(value);
        self
    }

    pub fn open_rate(mut self, value: f64) -> Self {
        self.open_rate = Some(value);
        self
    }

    pub fn target_sub_rate(mut self, value: f64) -> Self {
        self.target_sub_rate = Some(value);
        self
    }

    pub fn total_contacts(mut self, value: i64) -> Self {
        self.total_contacts = Some(value);
        self
    }

    pub fn unsubscribe_count(mut self, value: i64) -> Self {
        self.unsubscribe_count = Some(value);
        self
    }

    pub fn unsubscribe_count_since_send(mut self, value: i64) -> Self {
        self.unsubscribe_count_since_send = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriberListStats`].
    pub fn build(self) -> Result<SubscriberListStats, BuildError> {
        Ok(SubscriberListStats {
            avg_sub_rate: self.avg_sub_rate,
            avg_unsub_rate: self.avg_unsub_rate,
            campaign_count: self.campaign_count,
            campaign_last_sent: self.campaign_last_sent,
            cleaned_count: self.cleaned_count,
            cleaned_count_since_send: self.cleaned_count_since_send,
            click_rate: self.click_rate,
            last_sub_date: self.last_sub_date,
            last_unsub_date: self.last_unsub_date,
            member_count: self.member_count,
            member_count_since_send: self.member_count_since_send,
            merge_field_count: self.merge_field_count,
            open_rate: self.open_rate,
            target_sub_rate: self.target_sub_rate,
            total_contacts: self.total_contacts,
            unsubscribe_count: self.unsubscribe_count,
            unsubscribe_count_since_send: self.unsubscribe_count_since_send,
        })
    }
}
