use crate::degree_trait::BachelorsDegree;
use crate::degree_trait::MasterDegree;
#[derive(Clone)]
pub struct MasterStudent {
    pub name: String,
}
impl BachelorsDegree for MasterStudent {
    fn certification(&self) -> String {
        format!("{} has a bachelors degree", self.name)
    }
}
impl MasterDegree for MasterStudent {
    fn certification(&self) -> String {
        format!("{} has a master degree", self.name)
    }
}