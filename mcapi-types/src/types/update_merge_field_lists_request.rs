pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMergeFieldListsRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Extra options for some merge field types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<UpdateMergeFieldListsRequestOptions>,
    /// Whether the merge field is displayed on the signup form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// Whether the merge field is required to import a contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// The merge tag used for Mailchimp campaigns and [adding contact information](https://mailchimp.com/developer/marketing/docs/merge-fields/#add-merge-data-to-contacts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl UpdateMergeFieldListsRequest {
    pub fn builder() -> UpdateMergeFieldListsRequestBuilder {
        <UpdateMergeFieldListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMergeFieldListsRequestBuilder {
    default_value: Option<String>,
    display_order: Option<i64>,
    help_text: Option<String>,
    name: Option<String>,
    options: Option<UpdateMergeFieldListsRequestOptions>,
    public: Option<bool>,
    required: Option<bool>,
    tag: Option<String>,
}

impl UpdateMergeFieldListsRequestBuilder {
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

    pub fn options(mut self, value: UpdateMergeFieldListsRequestOptions) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateMergeFieldListsRequest`].
    pub fn build(self) -> Result<UpdateMergeFieldListsRequest, BuildError> {
        Ok(UpdateMergeFieldListsRequest {
            default_value: self.default_value,
            display_order: self.display_order,
            help_text: self.help_text,
            name: self.name,
            options: self.options,
            public: self.public,
            required: self.required,
            tag: self.tag,
        })
    }
}

