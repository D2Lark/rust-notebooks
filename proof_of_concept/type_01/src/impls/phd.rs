use crate::degree_trait::BachelorsDegree;
use crate::degree_trait::MasterDegree;
use crate::degree_trait::PhDDegree;
#[derive(Clone)]
pub struct PhdStudent {
    pub name: String,
}

impl BachelorsDegree for PhdStudent {
    fn certification(&self) -> String {
        format!("{} has a bachelors degree", self.name)
    }
}
impl MasterDegree for PhdStudent {
    fn certification(&self) -> String {
        format!("{} has a master degree", self.name)
    }
}
impl PhDDegree for PhdStudent {
    fn certification(&self) -> String {
        format!("{} has a phd degree", self.name)
    }
}
