use proc_macro::TokenStream;
use quote::quote;

pub fn enable() -> TokenStream {
    let code = quote! {
        use utils::memory::b_2_mb;
        use std::alloc::{GlobalAlloc, Layout, System};
        use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

        // This piece of code track all allocations and deallocations of heap memory
        // With this logs can be increase or decrease memory of Kubernetes
        struct Counter;

        static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
        static MAX_ALLOCATED: AtomicUsize = AtomicUsize::new(0);

        unsafe impl GlobalAlloc for Counter {
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

            unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
                System.dealloc(ptr, layout);
                ALLOCATED.fetch_sub(layout.size(), Relaxed);
            }
        }

        #[global_allocator]
        static A: Counter = Counter;
    };

    code.into()
}
