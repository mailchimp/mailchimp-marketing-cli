pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignRecipientsSegmentOpts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<SegmentType>,
    /// Segment match type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#match: Option<CampaignRecipientsSegmentOptsMatch>,
    /// The prebuilt segment id, if a prebuilt segment has been designated for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prebuilt_segment_id: Option<String>,
    /// The id for an existing saved segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved_segment_id: Option<CampaignRecipientsSegmentOptsSavedSegmentId>,
}

impl CampaignRecipientsSegmentOpts {
    pub fn builder() -> CampaignRecipientsSegmentOptsBuilder {
        <CampaignRecipientsSegmentOptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignRecipientsSegmentOptsBuilder {
    conditions: Option<SegmentType>,
    r#match: Option<CampaignRecipientsSegmentOptsMatch>,
    prebuilt_segment_id: Option<String>,
    saved_segment_id: Option<CampaignRecipientsSegmentOptsSavedSegmentId>,
}

impl CampaignRecipientsSegmentOptsBuilder {
    pub fn conditions(mut self, value: SegmentType) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn r#match(mut self, value: CampaignRecipientsSegmentOptsMatch) -> Self {
        self.r#match = Some(value);
        self
    }

    pub fn prebuilt_segment_id(mut self, value: impl Into<String>) -> Self {
        self.prebuilt_segment_id = Some(value.into());
        self
    }

    pub fn saved_segment_id(mut self, value: CampaignRecipientsSegmentOptsSavedSegmentId) -> Self {
        self.saved_segment_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignRecipientsSegmentOpts`].
    pub fn build(self) -> Result<CampaignRecipientsSegmentOpts, BuildError> {
        Ok(CampaignRecipientsSegmentOpts {
            conditions: self.conditions,
            r#match: self.r#match,
            prebuilt_segment_id: self.prebuilt_segment_id,
            saved_segment_id: self.saved_segment_id,
        })
    }
}
