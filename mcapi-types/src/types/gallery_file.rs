pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An individual file listed in the File Manager.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GalleryFile {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<GalleryFileLinksItem>>,
    /// The date and time a file was added to the File Manager in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The username of the profile that uploaded the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The id of the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    /// The url of the full-size file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_size_url: Option<String>,
    /// The height of an image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// The unique id of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The size of the file in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// The url of the thumbnail preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// The type of file in the File Manager.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GalleryFileType>,
    /// The width of the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl GalleryFile {
    pub fn builder() -> GalleryFileBuilder {
        <GalleryFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GalleryFileBuilder {
    links: Option<Vec<GalleryFileLinksItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    folder_id: Option<i64>,
    full_size_url: Option<String>,
    height: Option<i64>,
    id: Option<i64>,
    name: Option<String>,
    size: Option<i64>,
    thumbnail_url: Option<String>,
    r#type: Option<GalleryFileType>,
    width: Option<i64>,
}

impl GalleryFileBuilder {
    pub fn links(mut self, value: Vec<GalleryFileLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn folder_id(mut self, value: i64) -> Self {
        self.folder_id = Some(value);
        self
    }

    pub fn full_size_url(mut self, value: impl Into<String>) -> Self {
        self.full_size_url = Some(value.into());
        self
    }

    pub fn height(mut self, value: i64) -> Self {
        self.height = Some(value);
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

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.thumbnail_url = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: GalleryFileType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GalleryFile`].
    pub fn build(self) -> Result<GalleryFile, BuildError> {
        Ok(GalleryFile {
            links: self.links,
            created_at: self.created_at,
            created_by: self.created_by,
            folder_id: self.folder_id,
            full_size_url: self.full_size_url,
            height: self.height,
            id: self.id,
            name: self.name,
            size: self.size,
            thumbnail_url: self.thumbnail_url,
            r#type: self.r#type,
            width: self.width,
        })
    }
}
