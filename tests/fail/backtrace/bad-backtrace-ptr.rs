extern "Rust" {
    fn miri_resolve_frame(ptr: *mut (), flags: u64);
}

fn main() {
    unsafe {
        miri_resolve_frame(std::ptr::null_mut(), 0); //~ ERROR null pointer is not a valid pointer for this operation
    }
}
