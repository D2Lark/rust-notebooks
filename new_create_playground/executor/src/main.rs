use downcast_rs::{impl_downcast, Downcast};
use std::sync::Arc;
pub trait PlanNode: Downcast {}
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

fn main() {}
