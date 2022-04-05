use crate::plan_nodes::PlanNode;

#[derive(Debug, Clone)]
pub struct PhysicalTopN{}
impl PlanNode for PhysicalTopN{}