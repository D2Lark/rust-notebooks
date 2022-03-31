
pub trait Actor{
    fn sing() -> String{
        String::from("lalala")
    }
    fn dance() -> String{
        String::from("dancing")
    }
    fn act(&self) -> String{
        (<Self as Actor>::sing() + " " + &<Self as Actor>::dance()).to_string()
    }
}

pub struct Man;
pub struct Woman;

impl Actor for Man {
    fn sing() -> String {
        String::from("hahaha")
    }
}
impl Actor for Woman {
    fn dance() -> String{
        String::from("dengdengdeng")
    }
}

fn main(){
    let m = Man;
    let wm = Woman;   
    interview(m);
    interview(wm);
}

pub fn interview<T:Actor>(a:T){
    println!("{}",a.act());
}