#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let m = Man;
        let wm = Woman;
        let boy = Boy;
        let girl = Girl;
        interview(m, Child::Boy(boy));
        interview(wm, Child::Girl(girl));
    }

    pub fn interview<T: Actor>(a: T, child: Child) {
        println!("{}", a.act());
        a.training(child);
    }
    pub trait Actor {
        fn training(&self, child: Child) {
            match child {
                Child::Boy(boy) => {
                    println!("Teaching {:?} Sing :{}", boy, <Self as Actor>::sing())
                }
                Child::Girl(girl) => {
                    println!("Teaching {:?} Sing :{}", girl, <Self as Actor>::dance())
                }
            }
        }
        fn sing() -> String {
            String::from("lalala")
        }
        fn dance() -> String {
            String::from("dancing")
        }
        fn act(&self) -> String {
            <Self as Actor>::sing() + " " + &<Self as Actor>::dance()
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
        fn dance() -> String {
            String::from("dengdengdeng")
        }
    }
    #[derive(Debug)]
    pub struct Boy;
    #[derive(Debug)]
    pub struct Girl;

    pub enum Child {
        Boy(Boy),
        Girl(Girl),
    }
}
