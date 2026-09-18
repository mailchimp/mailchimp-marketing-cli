pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific template.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TemplateInstance {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<TemplateInstanceLinksItem>>,
    /// User templates are not 'deleted,' but rather marked as 'inactive.' Returns whether the template is still active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// If available, the category the template is listed in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// How the template's content is put together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<TemplateInstanceContentType>,
    /// The login name for template's creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The date and time the template was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub date_created: Option<DateTime<FixedOffset>>,
    /// The date and time the template was edited in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub date_edited: Option<DateTime<FixedOffset>>,
    /// Whether the template uses the drag and drop editor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drag_and_drop: Option<bool>,
    /// The login name who last edited the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_by: Option<String>,
    /// The id of the folder the template is currently in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// The individual id for the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the template contains media queries to make it responsive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsive: Option<bool>,
    /// The URL used for [template sharing](https://mailchimp.com/help/share-a-template/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
    /// If available, the URL for a thumbnail of the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<String>,
    /// The type of template (user, base, or gallery).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl TemplateInstance {
    pub fn builder() -> TemplateInstanceBuilder {
        <TemplateInstanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateInstanceBuilder {
    links: Option<Vec<TemplateInstanceLinksItem>>,
    active: Option<bool>,
    category: Option<String>,
    content_type: Option<TemplateInstanceContentType>,
    created_by: Option<String>,
    date_created: Option<DateTime<FixedOffset>>,
    date_edited: Option<DateTime<FixedOffset>>,
    drag_and_drop: Option<bool>,
    edited_by: Option<String>,
    folder_id: Option<String>,
    id: Option<i64>,
    name: Option<String>,
    responsive: Option<bool>,
    share_url: Option<String>,
    thumbnail: Option<String>,
    r#type: Option<String>,
}

impl TemplateInstanceBuilder {
    pub fn links(mut self, value: Vec<TemplateInstanceLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn content_type(mut self, value: TemplateInstanceContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn date_created(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date_created = Some(value);
        self
    }

    pub fn date_edited(mut self, value: DateTime<FixedOffset>) -> Self {
        self.date_edited = Some(value);
        self
    }

    pub fn drag_and_drop(mut self, value: bool) -> Self {
        self.drag_and_drop = Some(value);
        self
    }

    pub fn edited_by(mut self, value: impl Into<String>) -> Self {
        self.edited_by = Some(value.into());
        self
    }

    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn responsive(mut self, value: bool) -> Self {
        self.responsive = Some(value);
        self
    }

    pub fn share_url(mut self, value: impl Into<String>) -> Self {
        self.share_url = Some(value.into());
        self
    }

    pub fn thumbnail(mut self, value: impl Into<String>) -> Self {
        self.thumbnail = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TemplateInstance`].
    pub fn build(self) -> Result<TemplateInstance, BuildError> {
        Ok(TemplateInstance {
            links: self.links,
            active: self.active,
            category: self.category,
            content_type: self.content_type,
            created_by: self.created_by,
            date_created: self.date_created,
            date_edited: self.date_edited,
            drag_and_drop: self.drag_and_drop,
            edited_by: self.edited_by,
            folder_id: self.folder_id,
            id: self.id,
            name: self.name,
            responsive: self.responsive,
            share_url: self.share_url,
            thumbnail: self.thumbnail,
            r#type: self.r#type,
        })
    }
}
