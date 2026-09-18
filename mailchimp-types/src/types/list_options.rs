pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The conditions of the segment. Static segments (tags) and fuzzy segments don't have conditions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<SegmentType>,
    /// Match type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<ListOptionsMatch>,
}

impl ListOptions {
    pub fn builder() -> ListOptionsBuilder {
        <ListOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOptionsBuilder {
    conditions: Option<SegmentType>,
    r#match: Option<ListOptionsMatch>,
}

impl ListOptionsBuilder {
    pub fn conditions(mut self, value: SegmentType) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn r#match(mut self, value: ListOptionsMatch) -> Self {
        self.r#match = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOptions`].
    pub fn build(self) -> Result<ListOptions, BuildError> {
        Ok(ListOptions {
            conditions: self.conditions,
            r#match: self.r#match,
        })
    }
}
