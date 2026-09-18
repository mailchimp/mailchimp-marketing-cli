pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Individuals who are currently or have been previously subscribed to this list, including members who have bounced or unsubscribed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListsPost {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListsPostLinksItem>>,
    /// A unique ID for the contact record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// Email address for a subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The list member's email client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_client: Option<String>,
    /// Type of email this member asked to get ('html' or 'text').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of this object's properties is the ID of the interest in question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<HashMap<String, bool>>,
    /// The IP address the subscriber used to confirm their opt-in status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_opt: Option<String>,
    /// IP address the subscriber signed up from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_signup: Option<String>,
    /// If set/detected, the [subscriber's language](https://mailchimp.com/help/view-and-edit-contact-languages/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The date and time the member's info was last changed in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_changed: Option<DateTime<FixedOffset>>,
    /// The most recent Note added about this member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_note: Option<ListsPostLastNote>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// Subscriber location information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ListsPostLocation>,
    /// Star rating for this member, between 1 and 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_rating: Option<i64>,
    /// An individual merge var and value for a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, serde_json::Value>>,
    /// Open and click rates for this subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<ListsPostStats>,
    /// Subscriber's current status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListsPostStatus>,
    /// The tags applied to this member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListsPostTagsItem>>,
    /// The number of tags applied to this member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_count: Option<i64>,
    /// The date and time the subscriber confirmed their opt-in status in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp_opt: Option<DateTime<FixedOffset>>,
    /// The date and time the subscriber signed up for the list in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp_signup: Option<DateTime<FixedOffset>>,
    /// An identifier for the address across all of Mailchimp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_email_id: Option<String>,
    /// [VIP status](https://mailchimp.com/help/designate-and-send-to-vip-contacts/) for subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip: Option<bool>,
}

impl ListsPost {
    pub fn builder() -> ListsPostBuilder {
        <ListsPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListsPostBuilder {
    links: Option<Vec<ListsPostLinksItem>>,
    contact_id: Option<String>,
    email_address: Option<String>,
    email_client: Option<String>,
    email_type: Option<String>,
    id: Option<String>,
    interests: Option<HashMap<String, bool>>,
    ip_opt: Option<String>,
    ip_signup: Option<String>,
    language: Option<String>,
    last_changed: Option<DateTime<FixedOffset>>,
    last_note: Option<ListsPostLastNote>,
    list_id: Option<String>,
    location: Option<ListsPostLocation>,
    member_rating: Option<i64>,
    merge_fields: Option<HashMap<String, serde_json::Value>>,
    stats: Option<ListsPostStats>,
    status: Option<ListsPostStatus>,
    tags: Option<Vec<ListsPostTagsItem>>,
    tags_count: Option<i64>,
    timestamp_opt: Option<DateTime<FixedOffset>>,
    timestamp_signup: Option<DateTime<FixedOffset>>,
    unique_email_id: Option<String>,
    vip: Option<bool>,
}

impl ListsPostBuilder {
    pub fn links(mut self, value: Vec<ListsPostLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn contact_id(mut self, value: impl Into<String>) -> Self {
        self.contact_id = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn email_client(mut self, value: impl Into<String>) -> Self {
        self.email_client = Some(value.into());
        self
    }

    pub fn email_type(mut self, value: impl Into<String>) -> Self {
        self.email_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn interests(mut self, value: HashMap<String, bool>) -> Self {
        self.interests = Some(value);
        self
    }

    pub fn ip_opt(mut self, value: impl Into<String>) -> Self {
        self.ip_opt = Some(value.into());
        self
    }

    pub fn ip_signup(mut self, value: impl Into<String>) -> Self {
        self.ip_signup = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn last_changed(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_changed = Some(value);
        self
    }

    pub fn last_note(mut self, value: ListsPostLastNote) -> Self {
        self.last_note = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn location(mut self, value: ListsPostLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn member_rating(mut self, value: i64) -> Self {
        self.member_rating = Some(value);
        self
    }

    pub fn merge_fields(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn stats(mut self, value: ListsPostStats) -> Self {
        self.stats = Some(value);
        self
    }

    pub fn status(mut self, value: ListsPostStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<ListsPostTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn tags_count(mut self, value: i64) -> Self {
        self.tags_count = Some(value);
        self
    }

    pub fn timestamp_opt(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp_opt = Some(value);
        self
    }

    pub fn timestamp_signup(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp_signup = Some(value);
        self
    }

    pub fn unique_email_id(mut self, value: impl Into<String>) -> Self {
        self.unique_email_id = Some(value.into());
        self
    }

    pub fn vip(mut self, value: bool) -> Self {
        self.vip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListsPost`].
    pub fn build(self) -> Result<ListsPost, BuildError> {
        Ok(ListsPost {
            links: self.links,
            contact_id: self.contact_id,
            email_address: self.email_address,
            email_client: self.email_client,
            email_type: self.email_type,
            id: self.id,
            interests: self.interests,
            ip_opt: self.ip_opt,
            ip_signup: self.ip_signup,
            language: self.language,
            last_changed: self.last_changed,
            last_note: self.last_note,
            list_id: self.list_id,
            location: self.location,
            member_rating: self.member_rating,
            merge_fields: self.merge_fields,
            stats: self.stats,
            status: self.status,
            tags: self.tags,
            tags_count: self.tags_count,
            timestamp_opt: self.timestamp_opt,
            timestamp_signup: self.timestamp_signup,
            unique_email_id: self.unique_email_id,
            vip: self.vip,
        })
    }
}
