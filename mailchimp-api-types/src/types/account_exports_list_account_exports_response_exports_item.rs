pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An account export.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAccountExportsResponseExportsItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListAccountExportsResponseExportsItemLinksItem>>,
    /// If the export is finished, the download URL for an export. URLs are only valid for 90 days after the export completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// The ID for the export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<i64>,
    /// If finished, the finish time for the export.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub finished: Option<DateTime<FixedOffset>>,
    /// The size of the uncompressed export in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    /// Start time for the export.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub started: Option<DateTime<FixedOffset>>,
}

impl ListAccountExportsResponseExportsItem {
    pub fn builder() -> ListAccountExportsResponseExportsItemBuilder {
        <ListAccountExportsResponseExportsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAccountExportsResponseExportsItemBuilder {
    links: Option<Vec<ListAccountExportsResponseExportsItemLinksItem>>,
    download_url: Option<String>,
    export_id: Option<i64>,
    finished: Option<DateTime<FixedOffset>>,
    size_in_bytes: Option<i64>,
    started: Option<DateTime<FixedOffset>>,
}

impl ListAccountExportsResponseExportsItemBuilder {
    pub fn links(mut self, value: Vec<ListAccountExportsResponseExportsItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn download_url(mut self, value: impl Into<String>) -> Self {
        self.download_url = Some(value.into());
        self
    }

    pub fn export_id(mut self, value: i64) -> Self {
        self.export_id = Some(value);
        self
    }

    pub fn finished(mut self, value: DateTime<FixedOffset>) -> Self {
        self.finished = Some(value);
        self
    }

    pub fn size_in_bytes(mut self, value: i64) -> Self {
        self.size_in_bytes = Some(value);
        self
    }

    pub fn started(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAccountExportsResponseExportsItem`].
    pub fn build(self) -> Result<ListAccountExportsResponseExportsItem, BuildError> {
        Ok(ListAccountExportsResponseExportsItem {
            links: self.links,
            download_url: self.download_url,
            export_id: self.export_id,
            finished: self.finished,
            size_in_bytes: self.size_in_bytes,
            started: self.started,
        })
    }
}
