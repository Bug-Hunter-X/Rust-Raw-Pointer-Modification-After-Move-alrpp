fn main() {
    let mut v = vec![1, 2, 3];
    let len = v.len();
    let ptr = v.as_mut_ptr();
    unsafe {
        for i in 0..len {
            *ptr.add(i) = 10 + i as i32; //modify in a safe manner!
        }
    }
    println!("Vector elements: {:?}", v);
}