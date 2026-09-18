pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A single instance of a goal activity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberGoalsResponseGoalsItem {
    /// Any extra data passed with the Goal event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// The name/type of Goal event triggered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// The id for a Goal event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_id: Option<i64>,
    /// The date and time the user last triggered the Goal event in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_visited_at: Option<DateTime<FixedOffset>>,
}

impl ListMemberGoalsResponseGoalsItem {
    pub fn builder() -> ListMemberGoalsResponseGoalsItemBuilder {
        <ListMemberGoalsResponseGoalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberGoalsResponseGoalsItemBuilder {
    data: Option<String>,
    event: Option<String>,
    goal_id: Option<i64>,
    last_visited_at: Option<DateTime<FixedOffset>>,
}

impl ListMemberGoalsResponseGoalsItemBuilder {
    pub fn data(mut self, value: impl Into<String>) -> Self {
        self.data = Some(value.into());
        self
    }

    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn goal_id(mut self, value: i64) -> Self {
        self.goal_id = Some(value);
        self
    }

    pub fn last_visited_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_visited_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMemberGoalsResponseGoalsItem`].
    pub fn build(self) -> Result<ListMemberGoalsResponseGoalsItem, BuildError> {
        Ok(ListMemberGoalsResponseGoalsItem {
            data: self.data,
            event: self.event,
            goal_id: self.goal_id,
            last_visited_at: self.last_visited_at,
        })
    }
}
