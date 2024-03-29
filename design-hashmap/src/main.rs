fn main() {
    let mut obj = MyHashMap::new();
    obj.put(1, 1);
    obj.put(2, 2);
    println!("{}", obj.get(1));
    println!("{}", obj.get(3));
    obj.put(2, 1);
    println!("{}", obj.get(2));
    obj.remove(2);
    println!("{}", obj.get(2));
}

// --

struct MyHashMap {
    memory: [i32; 1000001],
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        Self {
            memory: [-1; 1000001],
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.memory[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        return self.memory[key as usize];
    }
    
    fn remove(&mut self, key: i32) {
        self.memory[key as usize] = -1;
    }
}

