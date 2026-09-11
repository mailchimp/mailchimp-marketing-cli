pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A [merge field](https://mailchimp.com/developer/marketing/docs/merge-fields/) for an audience.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MergeField {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<MergeFieldLinksItem>>,
    /// The default value for the merge field if `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// The order that the merge field displays on the list signup form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<i64>,
    /// Extra text to help the subscriber fill out the form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    /// The ID that identifies this merge field's audience'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The maximum number of merge fields this audience can hold. The limit is determined by the account's plan. Returned on POST responses only. Subtract `total_items` from this value to derive the remaining capacity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_field_limit: Option<i64>,
    /// An unchanging id for the merge field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_id: Option<i64>,
    /// The name of the merge field (audience field).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Extra options for some merge field types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<MergeFieldOptions>,
    /// Whether the merge field is displayed on the signup form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// The boolean value if the merge field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// The merge tag used for Mailchimp campaigns and [adding contact information](https://mailchimp.com/developer/marketing/docs/merge-fields/#add-merge-data-to-contacts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// The total number of merge fields on the audience after this field was created. Returned on POST responses only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// The [type](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for the merge field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<MergeFieldType>,
}

impl MergeField {
    pub fn builder() -> MergeFieldBuilder {
        <MergeFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MergeFieldBuilder {
    links: Option<Vec<MergeFieldLinksItem>>,
    default_value: Option<String>,
    display_order: Option<i64>,
    help_text: Option<String>,
    list_id: Option<String>,
    merge_field_limit: Option<i64>,
    merge_id: Option<i64>,
    name: Option<String>,
    options: Option<MergeFieldOptions>,
    public: Option<bool>,
    required: Option<bool>,
    tag: Option<String>,
    total_items: Option<i64>,
    r#type: Option<MergeFieldType>,
}

impl MergeFieldBuilder {
    pub fn links(mut self, value: Vec<MergeFieldLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

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

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn merge_field_limit(mut self, value: i64) -> Self {
        self.merge_field_limit = Some(value);
        self
    }

    pub fn merge_id(mut self, value: i64) -> Self {
        self.merge_id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn options(mut self, value: MergeFieldOptions) -> Self {
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

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn r#type(mut self, value: MergeFieldType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MergeField`].
    pub fn build(self) -> Result<MergeField, BuildError> {
        Ok(MergeField {
            links: self.links,
            default_value: self.default_value,
            display_order: self.display_order,
            help_text: self.help_text,
            list_id: self.list_id,
            merge_field_limit: self.merge_field_limit,
            merge_id: self.merge_id,
            name: self.name,
            options: self.options,
            public: self.public,
            required: self.required,
            tag: self.tag,
            total_items: self.total_items,
            r#type: self.r#type,
        })
    }
}
