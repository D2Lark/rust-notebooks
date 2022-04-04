use std::{result, vec};
#[cfg(test)]
mod tests
{
    #[test]
    fn main() {
        let strs_a = vec!["123", "456", "789"];
        let strs_b = vec!["1","35", "68"];
        let mut chars_a = strs_a.iter().flat_map(|f| f.chars()).peekable();
        let mut chars_b = strs_b.iter().flat_map(|f| f.chars()).peekable();
        let mut a = chars_a.next().unwrap();
        let mut b = chars_b.next().unwrap();
        let mut result = Vec::new();
        loop {
            match a.partial_cmp(&b).unwrap() {
                std::cmp::Ordering::Equal => {
                    println!("a:{} ==  b:{}", a, b);
                    result.push(a);
                    if chars_a.peek().is_none() {
                        break;
                    }
                    a = chars_a.next().unwrap();
                }
                std::cmp::Ordering::Greater => {
                    println!("a:{} > b:{}", a, b);
                    if chars_b.peek().is_none() {
                        break;
                    }
                    b = chars_b.next().unwrap();
                }
                std::cmp::Ordering::Less => {
                    println!("a:{} < b:{}", a, b);
                    if chars_a.peek().is_none() {
                        break;
                    }
                    a = chars_a.next().unwrap();
                }
            }
        }
        println!("result:{:?}", result);
    }
}

