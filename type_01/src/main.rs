use type_01::impls::bachelors::BachelorsStudent;
use type_01::impls::master::MasterStudent;
use type_01::impls::phd::PhdStudent;
use type_01::degree_trait::BachelorsDegree;


pub enum BachelorsDegreeImpl {
    Bachelors(BachelorsStudent),
    Master(MasterStudent),
    PhD(PhdStudent),   
}
//
pub fn reward_bachelors_degree_dyn_trait(degree: Box<dyn BachelorsDegree>) -> String {
    degree.certification()
}

pub fn reward_bachelors_degree_enum(degree: &BachelorsDegreeImpl) -> String {
    match degree {
        BachelorsDegreeImpl::Bachelors(student) => student.certification(),
        BachelorsDegreeImpl::Master(student) => todo!(),
        BachelorsDegreeImpl::PhD(student) => todo!(),
    }
}

fn main() {
    let b_student = BachelorsStudent {
        name: "John".to_string(),
    };
    let m_student = MasterStudent {
        name: "bob".to_string(),
    };

    let p_student = PhdStudent {
        name: "joe".to_string(),
    };

    let students :Vec<BachelorsDegreeImpl> = vec![BachelorsDegreeImpl::Bachelors(b_student.clone()), 
    BachelorsDegreeImpl::Master(m_student.clone()), BachelorsDegreeImpl::PhD(p_student.clone())];
    for student in students {
        println!("{}", reward_bachelors_degree_enum(&student));
    }

    let students_dyn_trait :Vec<Box<dyn BachelorsDegree>> = vec![Box::new(b_student),Box::new(m_student),Box::new(p_student)];
    for student in students_dyn_trait {
        println!("{}", reward_bachelors_degree_dyn_trait(student));
    }
    
}

