use criterion::{Criterion, criterion_group, criterion_main};
use time_travel_ling_hashmap::{
    TimeTravelLingHashMap, partial_ord_method::TimeTravelLingHashMapPartialOrd,
};

macro_rules! impl_bench {
    ($function_name: ident, $method:ident) => {
        fn $function_name() {
            let mut tth = $method::new();

            for _ in 0..10000 {
                tth.put("foo", 1, "car");
                tth.put("foo", 6, "jar");

                assert_eq!(tth.get("foo", 1).unwrap(), &"car".to_owned());
                assert_eq!(tth.get("foo", 6).unwrap(), &"jar".to_owned());
                assert_eq!(tth.get("foo", 8).unwrap(), &"jar".to_owned());

                tth.put("foo", 3, "bus");
                assert_eq!(tth.get("foo", 3).unwrap(), &"bus".to_owned());
                assert_eq!(tth.get("foo", 2).unwrap(), &"car".to_owned());
            }
        }
    };
}

impl_bench!(time_travel_ling_hashmap, TimeTravelLingHashMap);
impl_bench!(
    time_travel_ling_hashmap_partial_ord,
    TimeTravelLingHashMapPartialOrd
);

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("optimal", |b| b.iter(time_travel_ling_hashmap));
    c.bench_function("not_work_correcly", |b| {
        b.iter(time_travel_ling_hashmap_partial_ord)
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
