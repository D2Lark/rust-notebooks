pub mod logical_projection;
pub mod logical_top_n;
pub mod logical_table_scan;
pub mod physical_projection;
pub mod physical_top_n;
pub mod physical_table_scan;

pub use logical_projection::*;
pub use logical_top_n::*;
pub use logical_table_scan::*;
pub use physical_projection::*;
pub use physical_top_n::*;
pub use physical_table_scan::*;
use std::fmt::Display;
use downcast_rs::{impl_downcast, Downcast};
use std::sync::Arc;
use std::fmt::Debug;
pub trait IntoPlanRef {
    fn into_plan_ref(self) -> PlanRef;
    fn clone_as_plan_ref(&self) -> PlanRef;
}
pub trait PlanNode:
WithPlanNodeType
+ Debug
+ Downcast
+ Send
+ Sync {}
impl_downcast!(PlanNode);
pub enum PlanNodeType {
    LogicalTableScan,
    LogicalProjection,
    LogicalTopN,
    PhysicalTableScan,
    PhysicalProjection,
    PhysicalTopN,
}
pub type PlanRef = Arc<dyn PlanNode>;

pub trait WithPlanNodeType {
    fn node_type(&self) -> PlanNodeType;
}

impl WithPlanNodeType for LogicalTableScan {
    fn node_type(&self) -> PlanNodeType {
        PlanNodeType::LogicalTableScan
    }
}
impl WithPlanNodeType for LogicalProjection {
    fn node_type(&self) -> PlanNodeType {
        PlanNodeType::LogicalProjection
    }
}
impl WithPlanNodeType for LogicalTopN {
    fn node_type(&self) -> PlanNodeType {
        PlanNodeType::LogicalTopN
    }
}
impl WithPlanNodeType for PhysicalTableScan {
    fn node_type(&self) -> PlanNodeType {
        PlanNodeType::PhysicalTableScan
    }
}
impl WithPlanNodeType for PhysicalProjection {
    fn node_type(&self) -> PlanNodeType {
        PlanNodeType::PhysicalProjection
    }
}
impl WithPlanNodeType for PhysicalTopN {
    fn node_type(&self) -> PlanNodeType {
        PlanNodeType::PhysicalTopN
    }
}

impl dyn PlanNode {
    #[allow(clippy::result_unit_err)]
    pub fn as_logical_table_scan(&self) -> std::result::Result<&LogicalTableScan, ()> {
        self.downcast_ref::<LogicalTableScan>().ok_or_else(|| ())
    }
    #[allow(clippy::result_unit_err)]
    pub fn as_logical_projection(&self) -> std::result::Result<&LogicalProjection, ()> {
        self.downcast_ref::<LogicalProjection>().ok_or_else(|| ())
    }
    #[allow(clippy::result_unit_err)]
    pub fn as_logical_top_n(&self) -> std::result::Result<&LogicalTopN, ()> {
        self.downcast_ref::<LogicalTopN>().ok_or_else(|| ())
    }
    #[allow(clippy::result_unit_err)]
    pub fn as_physical_table_scan(&self) -> std::result::Result<&PhysicalTableScan, ()> {
        self.downcast_ref::<PhysicalTableScan>().ok_or_else(|| ())
    }
    #[allow(clippy::result_unit_err)]
    pub fn as_physical_projection(&self) -> std::result::Result<&PhysicalProjection, ()> {
        self.downcast_ref::<PhysicalProjection>().ok_or_else(|| ())
    }
    #[allow(clippy::result_unit_err)]
    pub fn as_physical_top_n(&self) -> std::result::Result<&PhysicalTopN, ()> {
        self.downcast_ref::<PhysicalTopN>().ok_or_else(|| ())
    }
}