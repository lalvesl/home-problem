use std::collections::{BTreeMap, HashMap};

pub type Timed = usize;

pub struct TimeTravelLingHashMap<T>(HashMap<String, BTreeMap<Timed, T>>);

impl<T> TimeTravelLingHashMap<T> {
    pub fn new() -> TimeTravelLingHashMap<T> {
        Self(HashMap::new())
    }

    /// Returns a reference to the value corresponding to the key and the time
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
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
            .and_then(|timed_values| timed_values.range(..=time).last().map(|(_, value)| value))
    }

    /// Gets the given key's corresponding entry in the map for in-place manipulation.
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
        let timed_values = self.0.entry(key.into()).or_default();

        timed_values.entry(time).or_insert(value);
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
    fn t_time_travel_ling_hashmap() {
        let mut tth = TimeTravelLingHashMap::new();

        tth.put("foo", 1, "car");
        tth.put("foo", 6, "jar");

        assert_eq!(tth.get("foo", 1).unwrap(), &"car".to_owned());
        assert_eq!(tth.get("foo", 6).unwrap(), &"jar".to_owned());
        assert_eq!(tth.get("foo", 3).unwrap(), &"car".to_owned());
        assert_eq!(tth.get("foo", 8).unwrap(), &"jar".to_owned());

        tth.put("foo", 3, "bus");
        assert_eq!(tth.get("foo", 3).unwrap(), &"bus".to_owned());
        assert_eq!(tth.get("foo", 2).unwrap(), &"car".to_owned());
    }
}
