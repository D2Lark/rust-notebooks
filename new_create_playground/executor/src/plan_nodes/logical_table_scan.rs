use crate::plan_nodes::PlanNode;

#[derive(Debug, Clone)]
pub struct LogicalTableScan{}
impl PlanNode for LogicalTableScan{}