pub trait BachelorsDegree {
    fn certification(&self) -> String;
}

pub trait MasterDegree: BachelorsDegree {
    fn certification(&self) -> String;
}

pub trait PhDDegree: MasterDegree {
    fn certification(&self) -> String;
}