use std::fmt::Display;

struct HashMap<K, V> {
    keys:   Vec<Option<K>>,
    values: Vec<V>,
    count: usize,
}

trait Hashable {
    fn hash(self: &Self) -> usize;
}

impl Hashable for String {
    fn hash(self: &Self) -> usize {
        let mut hash: usize = 5381;
        for c in self.bytes() {
            hash = (hash << 5).wrapping_add(hash).wrapping_add(c.into());
        }
        hash
    }
}

impl Hashable for &str {
    fn hash(self: &Self) -> usize {
        let mut hash: usize = 5381;
        for c in self.bytes() {
            hash = (hash << 5).wrapping_add(hash).wrapping_add(c.into());
        }
        hash
    }
}

impl<K: Hashable + Display + Clone + PartialEq, V: Display + Clone + Default> HashMap<K, V> {
    fn empty() -> Self {
        Self{
            keys:   Vec::<Option<K>>::new(),
            values: Vec::<V>::new(),
            count: 0,
        }
    }

    fn with_capacity(size: usize) -> Self {
        Self{
            keys:   vec![None; size],
            values: vec![V::default(); size],
            count: 0,
        }
    }

    fn insert(self: &mut Self, key: K, value: V) -> Result<(), &str> {
        if self.count == self.keys.len() {
            return Err("ERROR: hash map is full");
        }
        let index = key.hash() % self.keys.len();
        if self.does_key_exists(&key) {
            return Err("ERROR: can only insert unique key");
        }
        match &self.keys[index] {
            None => { 
                self.insert_into(index, key, value); 
                self.count += 1;
                return Ok(());
            },
            Some(_) => {
                let mut i = index;
                loop {
                    if self.keys[i % self.keys.len()].is_none() {
                        self.insert_into(i, key, value); 
                        self.count += 1;
                        return Ok(());
                    }
                    i+=1;
                }
            },
        }
    }

    fn insert_into(self: &mut Self, index: usize, key: K, value: V) {
        self.keys[index] = Some(key);
        self.values[index] = value;
    }

    fn get(self: &Self, key: K) -> Option<&V> {
        let index = key.hash() % self.keys.len();
        match self.keys[index] {
            Some(_) => Some(&self.values[index]),
            None => None,
        }
    }

    fn delete(self: &mut Self, key: K) {
        let index = key.hash() % self.keys.len();
        match &self.keys[index] {
            Some(_) => { 
                self.keys[index] = None; 
                self.count -= 1;
            },
            None => (),
        }
    }

    fn does_key_exists(self: &Self, key: &K) -> bool {
        let index = key.hash() % self.keys.len();
        match &self.keys[index] {
            Some(k) if key == k => {
                true
            },
            _ => false,
        }
    }

    fn display(self: &Self) {
        println!("---------------------------------------------");
        for (index, key) in self.keys.iter().enumerate() {
            match key {
                Some(k) => {
                    println!("{} -- {}", k, self.values[index]);
                },
                None => println!("..."),
            }
        }
        println!("---------------------------------------------");
    }
}

fn main() {
    let mut h = HashMap::<&str, &str>::with_capacity(5);
    h.insert("a", "a").unwrap();
    h.insert("b", "b").unwrap();
    h.insert("c", "c").unwrap();
    h.insert("d", "d").unwrap();
    h.insert("e", "e").unwrap();
    h.display();
    h.delete("e");
    h.display();
    h.insert("f", "f").unwrap();
    h.display();
}

