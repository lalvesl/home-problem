use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

use crate::Timed;

#[derive(PartialEq, Eq)]
enum TimedType {
    Common,
    Search,
}

impl Default for TimedType {
    fn default() -> Self {
        Self::Common
    }
}

struct TimedImpl {
    time: Timed,
    ty: TimedType,
}

impl TimedImpl {
    fn from(time: Timed) -> Self {
        Self {
            time,
            ty: TimedType::default(),
        }
    }

    fn search(time: Timed) -> Self {
        Self {
            time,
            ty: TimedType::Search,
        }
    }
}

impl PartialEq for TimedImpl {
    fn eq(&self, other: &Self) -> bool {
        if self.ty == TimedType::Search || other.ty == TimedType::Search {
            self.time <= other.time
        } else {
            self.time == other.time
        }
    }
}

impl Eq for TimedImpl {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for TimedImpl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut orded = other.time.cmp(&self.time);
        if self.ty == TimedType::Search || other.ty == TimedType::Search {
            orded = match orded {
                Ordering::Less => Ordering::Equal,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Greater,
            }
        }

        Some(orded)
    }
}

impl Ord for TimedImpl {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut orded = other.time.cmp(&self.time);
        if self.ty == TimedType::Search || other.ty == TimedType::Search {
            orded = match orded {
                Ordering::Less => Ordering::Equal,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Greater,
            }
        }

        orded
    }
}

/// FIXME: This method not work correcly when add morre elements for replacing
pub struct TimeTravelLingHashMapPartialOrd<T>(HashMap<String, BTreeMap<TimedImpl, T>>);

impl<T> TimeTravelLingHashMapPartialOrd<T> {
    pub fn new() -> TimeTravelLingHashMapPartialOrd<T> {
        Self(HashMap::new())
    }

    pub fn get<K: Into<String>>(&self, key: K, time: Timed) -> Option<&T> {
        self.0
            .get(&key.into())
            .and_then(|timed_values| timed_values.get(&TimedImpl::search(time)))
    }

    pub fn put<K: Into<String>>(&mut self, key: K, time: Timed, value: T) {
        let timed_values = self.0.entry(key.into()).or_default();

        timed_values.entry(TimedImpl::from(time)).or_insert(value);
    }
}

impl<T> Default for TimeTravelLingHashMapPartialOrd<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t_time_travel_ling_hashmap() {
        let mut tth = TimeTravelLingHashMapPartialOrd::new();

        tth.put("foo", 1, "car");
        tth.put("foo", 6, "jar");

        assert_eq!(tth.get("foo", 1).unwrap(), &"car".to_owned());
        assert_eq!(tth.get("foo", 6).unwrap(), &"jar".to_owned());
        // FIXME: Here the problem, if this code enter in loop, causing error
        assert_eq!(tth.get("foo", 3).unwrap(), &"car".to_owned());
        assert_eq!(tth.get("foo", 8).unwrap(), &"jar".to_owned());

        tth.put("foo", 3, "bus");
        assert_eq!(tth.get("foo", 3).unwrap(), &"bus".to_owned());
        assert_eq!(tth.get("foo", 2).unwrap(), &"car".to_owned());
    }
}
