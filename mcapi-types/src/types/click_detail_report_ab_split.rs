pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A breakdown of clicks by different groups of an A/B Split campaign. Does not return information about Multivariate Campaigns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClickDetailReportAbSplit {
    /// Stats for Group A.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<ClickDetailReportAbSplitA>,
    /// Stats for Group B.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b: Option<ClickDetailReportAbSplitB>,
}

impl ClickDetailReportAbSplit {
    pub fn builder() -> ClickDetailReportAbSplitBuilder {
        <ClickDetailReportAbSplitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClickDetailReportAbSplitBuilder {
    a: Option<ClickDetailReportAbSplitA>,
    b: Option<ClickDetailReportAbSplitB>,
}

impl ClickDetailReportAbSplitBuilder {
    pub fn a(mut self, value: ClickDetailReportAbSplitA) -> Self {
        self.a = Some(value);
        self
    }

    pub fn b(mut self, value: ClickDetailReportAbSplitB) -> Self {
        self.b = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ClickDetailReportAbSplit`].
    pub fn build(self) -> Result<ClickDetailReportAbSplit, BuildError> {
        Ok(ClickDetailReportAbSplit {
            a: self.a,
            b: self.b,
        })
    }
}
