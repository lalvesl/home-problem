use std::cmp::Ordering;

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
}

enum IndexType {
    Exacly(usize),
    Near(usize),
    None,
}

#[derive(Debug)]
pub struct TheBinaryTree<K, V> {
    bucket: Vec<Node<K, V>>,
}

impl<K, V> TheBinaryTree<K, V>
where
    K: Ord,
{
    pub fn new() -> TheBinaryTree<K, V> {
        Self { bucket: vec![] }
    }

    fn find_index(&self, key: &K) -> IndexType {
        if self.bucket.is_empty() {
            return IndexType::None;
        }
        let mut start = 0;
        let mut end = self.bucket.len();
        let mut mid;

        loop {
            mid = (end + start) / 2;
            match self.bucket[mid].key.cmp(key) {
                Ordering::Less => {
                    start = mid + 1;
                }
                Ordering::Equal => break IndexType::Exacly(mid),
                Ordering::Greater => end = mid,
            };
            if end <= start {
                break IndexType::Near(start);
            }
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        match self.find_index(&key) {
            IndexType::Exacly(index) => self.bucket[index].value = value,
            IndexType::None => self.bucket.push(Node { key, value }),
            IndexType::Near(index_insert) => self.bucket.insert(index_insert, Node { key, value }),
        };
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.find_index(key) {
            IndexType::None | IndexType::Near(_) => None,
            IndexType::Exacly(index) => Some(&self.bucket[index].value),
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        match self.find_index(key) {
            IndexType::None | IndexType::Near(_) => None,
            IndexType::Exacly(index) => Some(&mut self.bucket[index].value),
        }
    }

    pub fn get_or_lower_near(&self, key: &K) -> Option<&V> {
        match self.find_index(key) {
            IndexType::None => None,
            IndexType::Near(index) => {
                if 0 == index {
                    Some(&self.bucket[index].value)
                } else {
                    Some(&self.bucket[index - 1].value)
                }
            }
            IndexType::Exacly(index) => Some(&self.bucket[index].value),
        }
    }
}

impl<K, V> Default for TheBinaryTree<K, V>
where
    K: Ord,
{
    fn default() -> TheBinaryTree<K, V> {
        TheBinaryTree::new()
    }
}

pub type Timed = usize;

#[derive(Debug)]
pub struct TimeTravelLingHashMap<T>(TheBinaryTree<String, TheBinaryTree<Timed, T>>);

impl<T> TimeTravelLingHashMap<T> {
    pub fn new() -> TimeTravelLingHashMap<T> {
        Self(TheBinaryTree::new())
    }

    /// Returns a reference to the value corresponding to the key and the time
    ///
    /// The key may be any borrowed form of the map's key type, but
    ///
    /// # Examples
    ///
    /// ```
    /// use time_travel_ling_hashmap::TimeTravelLingHashMap;
    ///
    /// let mut tth = TimeTravelLingHashMap::new();
    ///
    /// tth.put("foo", 1, "car");
    /// tth.put("foo", 6, "jar");
    ///
    /// assert_eq!(tth.get("foo", 1).unwrap(), &"car".to_owned());
    /// assert_eq!(tth.get("foo", 6).unwrap(), &"jar".to_owned());
    /// assert_eq!(tth.get("foo", 3).unwrap(), &"car".to_owned());
    /// assert_eq!(tth.get("foo", 8).unwrap(), &"jar".to_owned());
    ///
    /// tth.put("foo", 3, "bus");
    /// assert_eq!(tth.get("foo", 3).unwrap(), &"bus".to_owned());
    /// assert_eq!(tth.get("foo", 2).unwrap(), &"car".to_owned());
    /// ```
    #[inline]
    pub fn get<K: Into<String>>(&self, key: K, time: Timed) -> Option<&T> {
        self.0
            .get(&key.into())
            .and_then(|timed_values| timed_values.get_or_lower_near(&time))
    }

    /// Add elements inside of the TimeTravelLingHashMap
    /// And get the put the value in time tracker
    ///
    /// # Examples
    ///
    /// ```
    /// use time_travel_ling_hashmap::TimeTravelLingHashMap;
    ///
    /// let mut tth = TimeTravelLingHashMap::new();
    ///
    /// tth.put("foo", 1, "car");
    /// tth.put("foo", 6, "jar");
    ///
    /// assert_eq!(tth.get("foo", 1).unwrap(), &"car".to_owned());
    /// assert_eq!(tth.get("foo", 6).unwrap(), &"jar".to_owned());
    /// assert_eq!(tth.get("foo", 3).unwrap(), &"car".to_owned());
    /// assert_eq!(tth.get("foo", 8).unwrap(), &"jar".to_owned());
    ///
    /// tth.put("foo", 3, "bus");
    /// assert_eq!(tth.get("foo", 3).unwrap(), &"bus".to_owned());
    /// assert_eq!(tth.get("foo", 2).unwrap(), &"car".to_owned());
    ///
    /// ```
    #[inline]
    pub fn put<K: Into<String>>(&mut self, key: K, time: Timed, value: T) {
        let key = key.into();

        if let Some(timed) = self.0.get_mut(&key) {
            timed.put(time, value);
            return;
        }

        let mut timed = TheBinaryTree::new();
        timed.put(time, value);
        self.0.put(key, timed);
    }
}

impl<T> Default for TimeTravelLingHashMap<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_the_binary_tree() {
        let mut bt = TheBinaryTree::new();

        (0..10).for_each(|i| bt.put(i.to_string(), i));

        println!("{bt:?}");

        let four = bt.get(&"4".to_string()).unwrap();
        let seven = bt.get(&"7".to_string()).unwrap();
        assert_eq!(*four, 4);
        assert_eq!(*seven, 7);
    }

    #[test]
    fn t_time_travel_ling_hashmap() {
        let mut tth = TimeTravelLingHashMap::new();

        tth.put("foo", 1, "car");
        tth.put("foo", 6, "jar");

        assert_eq!(tth.get("foo", 1).unwrap(), &"car".to_owned());
        assert_eq!(tth.get("foo", 6).unwrap(), &"jar".to_owned());
        println!("{tth:?}");
        assert_eq!(tth.get("foo", 3).unwrap(), &"car".to_owned());
        assert_eq!(tth.get("foo", 8).unwrap(), &"jar".to_owned());

        tth.put("foo", 3, "bus");
        assert_eq!(tth.get("foo", 3).unwrap(), &"bus".to_owned());
        assert_eq!(tth.get("foo", 2).unwrap(), &"car".to_owned());
    }
}
