#![allow(dead_code)]
mod hashtable {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::vec;
    #[derive(Debug, Default)]
    pub struct HashMapEntry<K, V> {
        pub key: K,
        pub value: V,
    }

    impl<K, V: Copy> HashMapEntry<K, V> {
        pub fn new(key: K, value: V) -> Self {
            HashMapEntry { key, value }
        }

        fn replace(&mut self, value: V) -> V {
            let previous_value = self.value;
            self.value = value;
            previous_value
        }
    }

    impl<K: Clone, V: Copy> Clone for HashMapEntry<K, V> {
        fn clone(&self) -> Self {
            HashMapEntry {
                key: self.key.clone(),
                value: self.value,
            }
        }
    }
    // HashMap will not handle collision.
    // It will require a function f(V) -> V on how to handle the previous_value
    #[derive(Debug)]
    pub struct HashMap<K, V> {
        amount: usize,
        entries: Vec<Option<HashMapEntry<K, V>>>,
    }

    impl<K, V> HashMap<K, V>
    where
        K: PartialEq + Eq + Default + Hash + Clone,
        V: Default + Copy,
    {
        pub fn new(size: usize) -> Self {
            HashMap {
                amount: 0,
                entries: vec![None; size],
            }
        }

        pub fn size(&self) -> usize {
            return self.entries.len();
        }

        pub fn amount(&self) -> usize {
            self.amount
        }

        pub fn remaining_entries(&self) -> usize {
            self.size() - self.amount()
        }

        #[inline]
        pub fn hash_key(key: &K) -> u64 {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            hasher.finish()
        }

        #[inline]
        pub fn get_index(&mut self, key: &K) -> usize {
            (HashMap::<K, V>::hash_key(&key) as usize) % self.size()
        }

        pub fn get(&mut self, key: &K) -> Option<V> {
            let mut index = self.get_index(key);
            loop {
                match &self.entries[index] {
                    None => return None, // Stop when we find an empty slot
                    Some(entry) => {
                        if entry.key == *key {
                            return Some(entry.value);
                        }
                        // else roll the index
                        index = ((5 * index) + 1) % self.size();
                    }
                }
            }
        }

        pub fn insert(&mut self, key: K, value: V) {
            // if there is no space left to insert, extend the size of the number of available entries in memory
            if self.remaining_entries() == 0 {
                self.extend(self.size());
            }
            let mut index = self.get_index(&key);
            loop {
                match &self.entries[index] {
                    None => {
                        self.entries[index] = Some(HashMapEntry::<K, V>::new(key.clone(), value));
                        self.amount += 1;
                        break;
                    }
                    Some(entry) => {
                        if entry.key == key {
                            self.entries[index] =
                                Some(HashMapEntry::<K, V>::new(key.clone(), value));
                            break;
                        } else {
                            index = ((5 * index) + 1) % self.size();
                        }
                    }
                }
            }
        }

        pub fn delete(&mut self, key: &K) -> Option<HashMapEntry<K, V>> {
            let mut index = self.get_index(key);
            loop {
                match &mut self.entries[index] {
                    None => return None, // Stop when we find an empty slot
                    Some(entry) => {
                        if entry.key == *key {
                            let popped_entry = Some(entry.clone());
                            self.entries[index] = None;
                            self.amount -= 1;
                            return popped_entry;
                        }
                        // Update the index
                        index = ((5 * index) + 1) % self.size();
                    }
                }
            }
        }
        // Extends the capacity of the hashmap with a new value;
        pub fn extend(&mut self, new_size: usize) {
            let mut new_entries = vec![None; self.size() + new_size];
            for entry in self.entries.iter() {
                match entry {
                    None => continue,
                    Some(entry) => {
                        let new_index =
                            HashMap::<K, V>::hash_key(&entry.key) as usize % new_entries.len();
                        new_entries[new_index] = Some(entry.clone());
                    }
                }
            }
            self.entries = new_entries;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::hashtable::*;
    #[test]
    fn build_hash_map_entry() {
        let entry = HashMapEntry::new("Hello", 0);
        let another_entry = HashMapEntry::new("World", 1);
        assert!(entry.key == "Hello");
        assert!(another_entry.key == "World");
        assert!(entry.value == 0);
        assert!(another_entry.value == 1);
    }
    #[test]
    fn build_hash_map() {
        let size = 10;
        let mut hash_map = HashMap::<&str, i32>::new(size);
        assert_eq!(hash_map.size(), size);
        hash_map.extend(size * 2);
        // extended the size by twice so we have size + 2*size = 3*size
        assert!(hash_map.size() == size * 3);
    }

    fn add_value(prev_value: usize, value: usize) -> usize {
        let new_value = prev_value + value;
        new_value
    }

    #[test]
    fn test_amount() {
        let size = 32;
        let mut hash_map = HashMap::<&str, usize>::new(size);
        let words = vec!["Hello", "World", "the", "hash", "table", "can", "word"];
        for w in words.iter() {
            hash_map.insert(w, 1);
        }
        hash_map.insert(&"World", 1);
        hash_map.insert(&"World", 1);
        hash_map.insert(&"the", 1);
        assert_eq!(hash_map.amount(), words.len());
    }
    #[test]
    fn test_insert() {
        let size = 32;
        let mut hash_map = HashMap::<&str, usize>::new(size);
        let words = vec!["Hello", "World", "the", "hash", "table", "can", "word"];
        for w in words {
            hash_map.insert(w, 1);
        }
        hash_map.insert(&"World", 2);
        hash_map.insert(&"World", 3);
        hash_map.insert(&"the", 2);
        assert_eq!(hash_map.get(&"World"), Some(3));
        assert_eq!(hash_map.get(&"the"), Some(2));
        assert_eq!(hash_map.get(&"can"), Some(1));
    }

    #[test]
    fn test_delete() {
        let size = 32;
        let mut hash_map = HashMap::<&str, usize>::new(size);
        let words = vec!["Hello", "World", "the", "hash", "table", "can", "word"];
        for w in words {
            hash_map.insert(w, 1);
        }
        hash_map.insert(&"World", 1);
        hash_map.insert(&"World", 2);
        hash_map.insert(&"the", 1);
        hash_map.delete(&"World");
        assert_eq!(hash_map.get(&"World"), None);
    }
}
