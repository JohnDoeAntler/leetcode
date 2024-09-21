fn main() {
    let mut obj = MyHashSet::new();
    obj.add(1);
    obj.remove(2);
    println!("{}", obj.contains(1));
    println!("{}", obj.contains(3));
    obj.add(2);
    println!("{}", obj.contains(2));
    obj.remove(2);
    println!("{}", obj.contains(2));
}

// --

struct MyHashSet {
    memory: [bool; 1000001],
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        Self {
            memory: [false; 1000001],
        }
    }
    
    fn add(&mut self, key: i32) {
        self.memory[key as usize] = true;
    }
    
    fn remove(&mut self, key: i32) {
        self.memory[key as usize] = false;
    }
    
    fn contains(&self, key: i32) -> bool {
        return self.memory[key as usize];
    }
}
