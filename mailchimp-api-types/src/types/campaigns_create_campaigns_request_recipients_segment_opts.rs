pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCampaignsRequestRecipientsSegmentOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<SegmentType>,
    /// Segment match type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<CreateCampaignsRequestRecipientsSegmentOptsMatch>,
    /// The id for an existing saved segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_segment_id: Option<i64>,
}

impl CreateCampaignsRequestRecipientsSegmentOpts {
    pub fn builder() -> CreateCampaignsRequestRecipientsSegmentOptsBuilder {
        <CreateCampaignsRequestRecipientsSegmentOptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCampaignsRequestRecipientsSegmentOptsBuilder {
    conditions: Option<SegmentType>,
    r#match: Option<CreateCampaignsRequestRecipientsSegmentOptsMatch>,
    saved_segment_id: Option<i64>,
}

impl CreateCampaignsRequestRecipientsSegmentOptsBuilder {
    pub fn conditions(mut self, value: SegmentType) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn r#match(mut self, value: CreateCampaignsRequestRecipientsSegmentOptsMatch) -> Self {
        self.r#match = Some(value);
        self
    }

    pub fn saved_segment_id(mut self, value: i64) -> Self {
        self.saved_segment_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCampaignsRequestRecipientsSegmentOpts`].
    pub fn build(self) -> Result<CreateCampaignsRequestRecipientsSegmentOpts, BuildError> {
        Ok(CreateCampaignsRequestRecipientsSegmentOpts {
            conditions: self.conditions,
            r#match: self.r#match,
            saved_segment_id: self.saved_segment_id,
        })
    }
}
