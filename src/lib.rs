#[allow(dead_code)]
fn main() {
    test_sum();
}

fn test_sum() {
    let input = vec![1 as u8, 2, 3, 4, 5];
    let ptr = alloc(input.len());
    let res: u8;
    unsafe {
        std::ptr::copy(input.as_ptr(), ptr, input.len());
        res = array_sum(ptr, input.len());
        // no need to call dealloc, since the array_sum
        // function already cleaned up the array data
        // dealloc(ptr, input.len());
    }
    println!("Result: {:#?}", res);
}

/// Allocate memory into the module's linear memory
/// and return the offset to the start of the block.
#[no_mangle]
pub fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    let data = Vec::from_raw_parts(ptr, size, size);

    std::mem::drop(data);
}

/// Given a pointer to the start of a byte array and
/// its length, return the sum of its elements.
#[no_mangle]
pub unsafe fn array_sum(ptr: *mut u8, len: usize) -> u8 {
    let data = Vec::from_raw_parts(ptr, len, len);
    data.iter().sum()
}

/// The Node.js WASI runtime requires a `_start` function
/// for instantiating the module.
#[no_mangle]
pub fn _start() {}
