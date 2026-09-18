pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An instance of a contact.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AudiencesContact {
    /// The unique ID for the audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    /// The date that the contact was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_channel: Option<AudiencesContactEmailChannel>,
    /// The unique ID for the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The contact's detected language. Empty string when no language has been detected or set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<AudiencesContactLanguage>,
    /// The date that the contact was last updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_updated_at: Option<DateTime<FixedOffset>>,
    /// A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, AudiencesContactMergeFieldsValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_channel: Option<AudiencesContactSmsChannel>,
    /// The source from which the parent's entity was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<AudiencesContactSource>,
    /// The status of a contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AudiencesContactStatus>,
    /// The tags assigned to this contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl AudiencesContact {
    pub fn builder() -> AudiencesContactBuilder {
        <AudiencesContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudiencesContactBuilder {
    audience_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    email_channel: Option<AudiencesContactEmailChannel>,
    id: Option<String>,
    language: Option<AudiencesContactLanguage>,
    last_updated_at: Option<DateTime<FixedOffset>>,
    merge_fields: Option<HashMap<String, AudiencesContactMergeFieldsValue>>,
    sms_channel: Option<AudiencesContactSmsChannel>,
    source: Option<AudiencesContactSource>,
    status: Option<AudiencesContactStatus>,
    tags: Option<Vec<String>>,
}

impl AudiencesContactBuilder {
    pub fn audience_id(mut self, value: impl Into<String>) -> Self {
        self.audience_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn email_channel(mut self, value: AudiencesContactEmailChannel) -> Self {
        self.email_channel = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn language(mut self, value: AudiencesContactLanguage) -> Self {
        self.language = Some(value);
        self
    }

    pub fn last_updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_updated_at = Some(value);
        self
    }

    pub fn merge_fields(mut self, value: HashMap<String, AudiencesContactMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn sms_channel(mut self, value: AudiencesContactSmsChannel) -> Self {
        self.sms_channel = Some(value);
        self
    }

    pub fn source(mut self, value: AudiencesContactSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: AudiencesContactStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudiencesContact`].
    pub fn build(self) -> Result<AudiencesContact, BuildError> {
        Ok(AudiencesContact {
            audience_id: self.audience_id,
            created_at: self.created_at,
            email_channel: self.email_channel,
            id: self.id,
            language: self.language,
            last_updated_at: self.last_updated_at,
            merge_fields: self.merge_fields,
            sms_channel: self.sms_channel,
            source: self.source,
            status: self.status,
            tags: self.tags,
        })
    }
}
