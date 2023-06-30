cfg_alloc! {
    use core::alloc::Layout;
    use core::ptr::NonNull;

    pub fn ax_alloc(layout: Layout) -> Option<NonNull<u8>> {
        if let Ok(vaddr) = axalloc::global_allocator().alloc(layout.size(), layout.align()) {
            NonNull::new(vaddr as _)
        } else {
            None
        }
    }

    pub fn ax_dealloc(ptr: NonNull<u8>, layout: Layout) {
        axalloc::global_allocator().dealloc(ptr.as_ptr() as _, layout.size(), layout.align())
    }
}
