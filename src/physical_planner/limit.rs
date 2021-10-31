use super::*;
use crate::logical_planner::LogicalLimit;

/// The physical plan of limit operation.
#[derive(Debug, PartialEq, Clone)]
pub struct PhysicalLimit {
    pub offset: usize,
    pub limit: usize,
    pub child: Box<PhysicalPlan>,
}

impl PhysicalPlaner {
    pub fn plan_limit(&self, plan: LogicalLimit) -> Result<PhysicalPlan, PhysicalPlanError> {
        Ok(PhysicalPlan::Limit(PhysicalLimit {
            offset: plan.offset,
            limit: plan.limit,
            child: Box::new(self.plan(*plan.child)?),
        }))
    }
}