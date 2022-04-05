use crate::plan_nodes::*;
pub trait PlanVisitor<R>{
    fn visit(&mut self, plan: PlanRef) ->Option<R>;
}