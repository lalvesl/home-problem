use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    time::Duration,
};
use tokio::time::sleep;

//This piece of code track all allocations and deallocations of heap memory
//With this logs can be increase or decrease memory of Kubernetes
struct Counter;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static MAX_ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counter {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Relaxed);
            if ALLOCATED.load(Relaxed) > MAX_ALLOCATED.load(Relaxed) {
                MAX_ALLOCATED.store(ALLOCATED.load(Relaxed), Relaxed);
            }
        }
        ret
    }

    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc_zeroed(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Relaxed);
        }
        ret
    }

    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), Relaxed);
    }

    #[inline(always)]
    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        let ret = System.realloc(ptr, layout, new_size);
        let s = layout.size();
        if s > new_size {
            ALLOCATED.fetch_sub(s - new_size, Relaxed);
        }
        if s < new_size {
            ALLOCATED.fetch_add(new_size - s, Relaxed);
            if ALLOCATED.load(Relaxed) > MAX_ALLOCATED.load(Relaxed) {
                MAX_ALLOCATED.store(ALLOCATED.load(Relaxed), Relaxed);
            }
        }
        ret
    }
}

#[global_allocator]
static GLOBAL_ALLOC: Counter = Counter;

#[inline(always)]
pub fn b_2_mb(inp: usize) -> usize {
    inp / (1024 * 1024)
}

pub async fn memory_logger() {
    loop {
        log::info!(
            "Memory usage: {0}MB, maximum: {1}MB",
            b_2_mb(ALLOCATED.load(Relaxed)),
            b_2_mb(MAX_ALLOCATED.load(Relaxed))
        );
        sleep(Duration::from_secs(60)).await;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_memory_allocation_feedback() {
        let amount_mem = 1024 * 1024 * 256;
        {
            let mut a: Vec<u8> = Vec::new();

            for _i in 0..amount_mem {
                a.push(122);
            }

            sleep(Duration::from_secs(1));
            assert!(MAX_ALLOCATED.load(Relaxed) >= amount_mem);
            assert!(ALLOCATED.load(Relaxed) >= amount_mem)
        }

        sleep(Duration::from_secs(10));
        assert!(MAX_ALLOCATED.load(Relaxed) >= amount_mem);
        assert!(ALLOCATED.load(Relaxed) <= amount_mem)
    }
}
