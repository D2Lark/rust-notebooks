
fn main() {
    let strs = vec!["hello", "world", "rust"];
    let chars:String = strs.iter().flat_map(|f|f.chars()).collect();
    println!("{:?}", chars);
}
