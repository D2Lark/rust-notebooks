use crate::plan_nodes::PlanNode;

#[derive(Debug, Clone)]
pub struct LogicalProjection{}
impl PlanNode for LogicalProjection{}