pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateMergeFieldRequest {
    /// The default value for the merge field if `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The order that the merge field displays on the list signup form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// Extra text to help the subscriber fill out the form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    /// The name of the merge field (audience field).
    #[serde(default)]
    pub name: String,
    /// Extra options for some merge field types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateMergeFieldRequestOptions>,
    /// Whether the merge field is displayed on the signup form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// Whether the merge field is required to import a contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// The merge tag used for Mailchimp campaigns and [adding contact information](https://mailchimp.com/developer/marketing/docs/merge-fields/#add-merge-data-to-contacts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// The [type](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for the merge field.
    pub r#type: CreateMergeFieldRequestType,
}

impl CreateMergeFieldRequest {
    pub fn builder() -> CreateMergeFieldRequestBuilder {
        <CreateMergeFieldRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMergeFieldRequestBuilder {
    default_value: Option<String>,
    display_order: Option<i64>,
    help_text: Option<String>,
    name: Option<String>,
    options: Option<CreateMergeFieldRequestOptions>,
    public: Option<bool>,
    required: Option<bool>,
    tag: Option<String>,
    r#type: Option<CreateMergeFieldRequestType>,
}

impl CreateMergeFieldRequestBuilder {
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn display_order(mut self, value: i64) -> Self {
        self.display_order = Some(value);
        self
    }

    pub fn help_text(mut self, value: impl Into<String>) -> Self {
        self.help_text = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn options(mut self, value: CreateMergeFieldRequestOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn public(mut self, value: bool) -> Self {
        self.public = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CreateMergeFieldRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMergeFieldRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateMergeFieldRequestBuilder::name)
    /// - [`r#type`](CreateMergeFieldRequestBuilder::r#type)
    pub fn build(self) -> Result<CreateMergeFieldRequest, BuildError> {
        Ok(CreateMergeFieldRequest {
            default_value: self.default_value,
            display_order: self.display_order,
            help_text: self.help_text,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            options: self.options,
            public: self.public,
            required: self.required,
            tag: self.tag,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}

