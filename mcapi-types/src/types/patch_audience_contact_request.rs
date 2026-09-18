pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchAudienceContactRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_channel: Option<PatchAudienceContactRequestEmailChannel>,
    /// The contact's detected language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// A dictionary of merge fields where the keys are the merge tags. See the [Merge Fields documentation](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for more about the structure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<HashMap<String, PatchAudienceContactRequestMergeFieldsValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_channel: Option<PatchAudienceContactRequestSmsChannel>,
    /// An array of tags to add to the contact. Accepts tag name strings or objects with name and status. This operation is append-only; existing tags will be preserved, and only new tags from this array will be added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<PatchAudienceContactRequestTagsItem>>,
    /// Defines how merge field validation is handled. When set to `ignore_required_checks`, the API does not raise an error if required merge fields are missing from the request. When set to `strict`, the API enforces validation and returns an error if any required merge field is not provided. If this setting is omitted, `strict` is applied by default.
    #[serde(skip)]
    pub merge_field_validation_mode: Option<PatchAudienceContactRequestMergeFieldValidationMode>,
    /// Indicates the data processing mode. In `historical` mode, contact data changes do not trigger automations or webhooks. In `live mode`, such changes do trigger them.
    #[serde(skip)]
    pub data_mode: Option<PatchAudienceContactRequestDataMode>,
}

impl PatchAudienceContactRequest {
    pub fn builder() -> PatchAudienceContactRequestBuilder {
        <PatchAudienceContactRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchAudienceContactRequestBuilder {
    email_channel: Option<PatchAudienceContactRequestEmailChannel>,
    language: Option<String>,
    merge_fields: Option<HashMap<String, PatchAudienceContactRequestMergeFieldsValue>>,
    sms_channel: Option<PatchAudienceContactRequestSmsChannel>,
    tags: Option<Vec<PatchAudienceContactRequestTagsItem>>,
    merge_field_validation_mode: Option<PatchAudienceContactRequestMergeFieldValidationMode>,
    data_mode: Option<PatchAudienceContactRequestDataMode>,
}

impl PatchAudienceContactRequestBuilder {
    pub fn email_channel(mut self, value: PatchAudienceContactRequestEmailChannel) -> Self {
        self.email_channel = Some(value);
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn merge_fields(mut self, value: HashMap<String, PatchAudienceContactRequestMergeFieldsValue>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn sms_channel(mut self, value: PatchAudienceContactRequestSmsChannel) -> Self {
        self.sms_channel = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<PatchAudienceContactRequestTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn merge_field_validation_mode(mut self, value: PatchAudienceContactRequestMergeFieldValidationMode) -> Self {
        self.merge_field_validation_mode = Some(value);
        self
    }

    pub fn data_mode(mut self, value: PatchAudienceContactRequestDataMode) -> Self {
        self.data_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchAudienceContactRequest`].
    pub fn build(self) -> Result<PatchAudienceContactRequest, BuildError> {
        Ok(PatchAudienceContactRequest {
            email_channel: self.email_channel,
            language: self.language,
            merge_fields: self.merge_fields,
            sms_channel: self.sms_channel,
            tags: self.tags,
            merge_field_validation_mode: self.merge_field_validation_mode,
            data_mode: self.data_mode,
        })
    }
}

