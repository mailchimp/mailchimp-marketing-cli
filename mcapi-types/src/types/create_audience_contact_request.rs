pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAudienceContactRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_channel: Option<CreateAudienceContactRequestEmailChannel>,
    /// The contact's detected language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, CreateAudienceContactRequestMergeFieldsValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_channel: Option<CreateAudienceContactRequestSmsChannel>,
    /// An array of tags to add to the contact. Accepts tag name strings or objects with name and status. This operation is append-only; existing tags will be preserved, and only new tags from this array will be added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CreateAudienceContactRequestTagsItem>>,
    /// If a contact already exists, update them instead of returning a conflict error. When `true` and a matching contact is found (by email or phone), the existing contact is updated with the provided channel data. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_existing: Option<bool>,
    /// Defines how merge field validation is handled. When set to `ignore_required_checks`, the API does not raise an error if required merge fields are missing from the request. When set to `strict`, the API enforces validation and returns an error if any required merge field is not provided. If this setting is omitted, `strict` is applied by default.
    #[serde(skip)]
    pub merge_field_validation_mode: Option<CreateAudienceContactRequestMergeFieldValidationMode>,
    /// Indicates the data processing mode. In `historical` mode, contact data changes do not trigger automations or webhooks. In `live mode`, such changes do trigger them.
    #[serde(skip)]
    pub data_mode: Option<CreateAudienceContactRequestDataMode>,
}

impl CreateAudienceContactRequest {
    pub fn builder() -> CreateAudienceContactRequestBuilder {
        <CreateAudienceContactRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudienceContactRequestBuilder {
    email_channel: Option<CreateAudienceContactRequestEmailChannel>,
    language: Option<String>,
    merge_fields: Option<HashMap<String, CreateAudienceContactRequestMergeFieldsValue>>,
    sms_channel: Option<CreateAudienceContactRequestSmsChannel>,
    tags: Option<Vec<CreateAudienceContactRequestTagsItem>>,
    update_existing: Option<bool>,
    merge_field_validation_mode: Option<CreateAudienceContactRequestMergeFieldValidationMode>,
    data_mode: Option<CreateAudienceContactRequestDataMode>,
}

impl CreateAudienceContactRequestBuilder {
    pub fn email_channel(mut self, value: CreateAudienceContactRequestEmailChannel) -> Self {
        self.email_channel = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn merge_fields(mut self, value: HashMap<String, CreateAudienceContactRequestMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn sms_channel(mut self, value: CreateAudienceContactRequestSmsChannel) -> Self {
        self.sms_channel = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<CreateAudienceContactRequestTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn update_existing(mut self, value: bool) -> Self {
        self.update_existing = Some(value);
        self
    }

    pub fn merge_field_validation_mode(mut self, value: CreateAudienceContactRequestMergeFieldValidationMode) -> Self {
        self.merge_field_validation_mode = Some(value);
        self
    }

    pub fn data_mode(mut self, value: CreateAudienceContactRequestDataMode) -> Self {
        self.data_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAudienceContactRequest`].
    pub fn build(self) -> Result<CreateAudienceContactRequest, BuildError> {
        Ok(CreateAudienceContactRequest {
            email_channel: self.email_channel,
            language: self.language,
            merge_fields: self.merge_fields,
            sms_channel: self.sms_channel,
            tags: self.tags,
            update_existing: self.update_existing,
            merge_field_validation_mode: self.merge_field_validation_mode,
            data_mode: self.data_mode,
        })
    }
}

