pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One day's worth of list activity. Doesn't include Automation activity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListActivityResponseActivityItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListActivityResponseActivityItemLinksItem>>,
    /// The date for the activity summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<String>,
    /// The total number of emails sent on the date for the activity summary.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<i64>,
    /// The number of hard bounces.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard_bounce: Option<i64>,
    /// The number of subscribers who may have been added outside of the [double opt-in process](https://mailchimp.com/help/about-double-opt-in/), such as imports or API activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_adds: Option<i64>,
    /// The number of subscribers who may have been removed outside of unsubscribing or reporting an email as spam (for example, deleted subscribers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_removes: Option<i64>,
    /// The number of clicks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_clicks: Option<i64>,
    /// The number of soft bounces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_bounce: Option<i64>,
    /// The number of subscribes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subs: Option<i64>,
    /// The number of unique opens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i64>,
    /// The number of unsubscribes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubs: Option<i64>,
}

impl ListActivityResponseActivityItem {
    pub fn builder() -> ListActivityResponseActivityItemBuilder {
        <ListActivityResponseActivityItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListActivityResponseActivityItemBuilder {
    links: Option<Vec<ListActivityResponseActivityItemLinksItem>>,
    day: Option<String>,
    emails_sent: Option<i64>,
    hard_bounce: Option<i64>,
    other_adds: Option<i64>,
    other_removes: Option<i64>,
    recipient_clicks: Option<i64>,
    soft_bounce: Option<i64>,
    subs: Option<i64>,
    unique_opens: Option<i64>,
    unsubs: Option<i64>,
}

impl ListActivityResponseActivityItemBuilder {
    pub fn links(mut self, value: Vec<ListActivityResponseActivityItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn day(mut self, value: impl Into<String>) -> Self {
        self.day = Some(value.into());
        self
    }

    pub fn emails_sent(mut self, value: i64) -> Self {
        self.emails_sent = Some(value);
        self
    }

    pub fn hard_bounce(mut self, value: i64) -> Self {
        self.hard_bounce = Some(value);
        self
    }

    pub fn other_adds(mut self, value: i64) -> Self {
        self.other_adds = Some(value);
        self
    }

    pub fn other_removes(mut self, value: i64) -> Self {
        self.other_removes = Some(value);
        self
    }

    pub fn recipient_clicks(mut self, value: i64) -> Self {
        self.recipient_clicks = Some(value);
        self
    }

    pub fn soft_bounce(mut self, value: i64) -> Self {
        self.soft_bounce = Some(value);
        self
    }

    pub fn subs(mut self, value: i64) -> Self {
        self.subs = Some(value);
        self
    }

    pub fn unique_opens(mut self, value: i64) -> Self {
        self.unique_opens = Some(value);
        self
    }

    pub fn unsubs(mut self, value: i64) -> Self {
        self.unsubs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListActivityResponseActivityItem`].
    pub fn build(self) -> Result<ListActivityResponseActivityItem, BuildError> {
        Ok(ListActivityResponseActivityItem {
            links: self.links,
            day: self.day,
            emails_sent: self.emails_sent,
            hard_bounce: self.hard_bounce,
            other_adds: self.other_adds,
            other_removes: self.other_removes,
            recipient_clicks: self.recipient_clicks,
            soft_bounce: self.soft_bounce,
            subs: self.subs,
            unique_opens: self.unique_opens,
            unsubs: self.unsubs,
        })
    }
}
