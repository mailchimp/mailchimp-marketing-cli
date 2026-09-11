pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of a specific list's growth activity for a specific month and year.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GrowthHistory {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<GrowthHistoryLinksItem>>,
    /// Newly cleaned (hard-bounced) members on the list for a specific month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleaned: Option<i64>,
    /// Newly deleted members on the list for a specific month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<i64>,
    /// (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing: Option<i64>,
    /// (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imports: Option<i64>,
    /// The list id for the growth activity report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The month that the growth history is describing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<String>,
    /// (deprecated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optins: Option<i64>,
    /// Pending members on the list for a specific month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    /// Newly reconfirmed members on the list for a specific month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconfirm: Option<i64>,
    /// Total subscribed members on the list at the end of the month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<i64>,
    /// Subscribers that have been sent transactional emails via Mandrill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional: Option<i64>,
    /// Newly unsubscribed members on the list for a specific month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
}

impl GrowthHistory {
    pub fn builder() -> GrowthHistoryBuilder {
        <GrowthHistoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GrowthHistoryBuilder {
    links: Option<Vec<GrowthHistoryLinksItem>>,
    cleaned: Option<i64>,
    deleted: Option<i64>,
    existing: Option<i64>,
    imports: Option<i64>,
    list_id: Option<String>,
    month: Option<String>,
    optins: Option<i64>,
    pending: Option<i64>,
    reconfirm: Option<i64>,
    subscribed: Option<i64>,
    transactional: Option<i64>,
    unsubscribed: Option<i64>,
}

impl GrowthHistoryBuilder {
    pub fn links(mut self, value: Vec<GrowthHistoryLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn cleaned(mut self, value: i64) -> Self {
        self.cleaned = Some(value);
        self
    }

    pub fn deleted(mut self, value: i64) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn existing(mut self, value: i64) -> Self {
        self.existing = Some(value);
        self
    }

    pub fn imports(mut self, value: i64) -> Self {
        self.imports = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn month(mut self, value: impl Into<String>) -> Self {
        self.month = Some(value.into());
        self
    }

    pub fn optins(mut self, value: i64) -> Self {
        self.optins = Some(value);
        self
    }

    pub fn pending(mut self, value: i64) -> Self {
        self.pending = Some(value);
        self
    }

    pub fn reconfirm(mut self, value: i64) -> Self {
        self.reconfirm = Some(value);
        self
    }

    pub fn subscribed(mut self, value: i64) -> Self {
        self.subscribed = Some(value);
        self
    }

    pub fn transactional(mut self, value: i64) -> Self {
        self.transactional = Some(value);
        self
    }

    pub fn unsubscribed(mut self, value: i64) -> Self {
        self.unsubscribed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GrowthHistory`].
    pub fn build(self) -> Result<GrowthHistory, BuildError> {
        Ok(GrowthHistory {
            links: self.links,
            cleaned: self.cleaned,
            deleted: self.deleted,
            existing: self.existing,
            imports: self.imports,
            list_id: self.list_id,
            month: self.month,
            optins: self.optins,
            pending: self.pending,
            reconfirm: self.reconfirm,
            subscribed: self.subscribed,
            transactional: self.transactional,
            unsubscribed: self.unsubscribed,
        })
    }
}
