extern "C" {
    pub fn woops_local_array() -> *const u8;
}

#[test]
fn test_oops() {
    unsafe {
        let array_ptr = woops_local_array();
        let array: [u8; 6] = std::ptr::read(array_ptr as _);
        assert_eq!(array, [3, 1, 4, 1, 5, 9]);
    }
}
