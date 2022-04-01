use enum_dispatch::enum_dispatch;

#[enum_dispatch(ArrayImpl)]
trait Array {
    fn get(&self, index: usize);
}

#[enum_dispatch]
enum ArrayImpl {
    StringArray(StringArray),
    IntArray(IntArray),
}

pub struct StringArray {
    data: Vec<u8>,
}
pub struct IntArray {
    data: Vec<i32>,
}

impl Array for StringArray {
    fn get(&self, index: usize) {
        println!("{} index of String array is {}", index, self.data[index]);
    }
}

impl Array for IntArray {
    fn get(&self, index: usize) {
        println!("{} index of Int array is {}", index, self.data[index]);
    }
}

#[test]
fn test_01() {
    let sa = StringArray {
        data: "daadddas".as_bytes().to_vec(),
    };
    let ia = IntArray {
        data: vec![1, 2, 3],
    };
    sa.get(0);
    ia.get(0);
}
