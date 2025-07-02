pub trait ReVec {
    #[allow(unused)]
    fn to_sort(&self) -> Self;
    // fn to_sort_by<F>(&self, compare: F) -> Self;
}

// pub fn sort_by<F>(&mut self, mut compare: F)
//     where
//         F: FnMut(&T, &T) -> Ordering,
//     {
//         stable_sort(self, |a, b| compare(a, b) == Less);
//     }

impl<T: Ord + PartialEq + Clone + Eq + PartialEq> ReVec for Vec<T> {
    fn to_sort(&self) -> Self {
        let mut self_cloned = self.clone();
        self_cloned.sort();
        self_cloned
    }

    // fn to_sort_by<F>(&self, compare: F) -> Self
    // where
    //     for<'a> F: Fn(&'a T, &'a T) -> Ordering,
    // {
    //     let mut self_cloned = self.clone();
    //     self_cloned.sort_by(&mut compare);
    //     self_cloned
    // }
}
