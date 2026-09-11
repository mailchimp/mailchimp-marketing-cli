pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Do particular authorization constraints around this collection limit creation of new instances?
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListListsResponseConstraints {
    /// How many total instances of this resource are already in use? This is independent of any filter conditions applied to the query. Value may be larger than max_instances. As a special case, -1 is returned when access is unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_total_instances: Option<i64>,
    /// How many total instances of this resource are allowed? This is independent of any filter conditions applied to the query. As a special case, -1 indicates unlimited.
    #[serde(default)]
    pub max_instances: i64,
    /// May the user create additional instances of this resource?
    #[serde(default)]
    pub may_create: bool,
}

impl ListListsResponseConstraints {
    pub fn builder() -> ListListsResponseConstraintsBuilder {
        <ListListsResponseConstraintsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListListsResponseConstraintsBuilder {
    current_total_instances: Option<i64>,
    max_instances: Option<i64>,
    may_create: Option<bool>,
}

impl ListListsResponseConstraintsBuilder {
    pub fn current_total_instances(mut self, value: i64) -> Self {
        self.current_total_instances = Some(value);
        self
    }

    pub fn max_instances(mut self, value: i64) -> Self {
        self.max_instances = Some(value);
        self
    }

    pub fn may_create(mut self, value: bool) -> Self {
        self.may_create = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListListsResponseConstraints`].
    /// This method will fail if any of the following fields are not set:
    /// - [`max_instances`](ListListsResponseConstraintsBuilder::max_instances)
    /// - [`may_create`](ListListsResponseConstraintsBuilder::may_create)
    pub fn build(self) -> Result<ListListsResponseConstraints, BuildError> {
        Ok(ListListsResponseConstraints {
            current_total_instances: self.current_total_instances,
            max_instances: self.max_instances.ok_or_else(|| BuildError::missing_field("max_instances"))?,
            may_create: self.may_create.ok_or_else(|| BuildError::missing_field("may_create"))?,
        })
    }
}
