struct MyHashSet {
    internal_vector: Vec<i32>,
}

impl MyHashSet {

    fn new() -> Self {
        let internal_vector = Vec::new();
        Self { internal_vector }
    }
    
    fn add(&mut self, key: i32) {
        if self.contains(key) {
            return
        }
        self.internal_vector.push(key)
    }
    
    fn remove(&mut self, key: i32) {
        let index: usize = match self.internal_vector.iter().position(|&x| x == key) {
            Some(index) => index,
            None => return
        };
        self.internal_vector.remove(index);
        
    }
    
    fn contains(&self, key: i32) -> bool {
        self.internal_vector.contains(&key)
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

fn main() {

    let mut obj = MyHashSet::new();
    obj.add(1);
    obj.add(1);
    obj.remove(1);
    let answer = obj.contains(1);

    println!("{answer}");
}
