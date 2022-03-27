macro_rules! myvec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_myvec(){
        let a = myvec![1,2,3];
        let b = myvec!["hello", "world"];
        println!("{:?}", a);
        println!("{:?}", b);
        
    }
}