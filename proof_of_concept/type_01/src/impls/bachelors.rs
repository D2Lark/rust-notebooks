use crate::degree_trait::BachelorsDegree;

#[derive(Clone)]
pub struct BachelorsStudent {
    pub name: String,
}
impl BachelorsDegree for BachelorsStudent {
    fn certification(&self) -> String {
        format!("{} has a bachelors degree", self.name)
    }
}
