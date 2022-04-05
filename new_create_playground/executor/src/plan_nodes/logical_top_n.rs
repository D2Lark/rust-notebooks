use crate::plan_nodes::PlanNode;

#[derive(Debug, Clone)]
pub struct LogicalTopN{}
impl PlanNode for LogicalTopN{}