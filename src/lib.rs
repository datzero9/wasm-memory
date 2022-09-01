#[allow(dead_code)]
fn main() {
    test_sum();
    test_upper();
}

fn test_sum() {
    let input = vec![1_u8, 2, 3, 4, 5];
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

fn test_upper() {
    let input = "this should be uppercase";
    let ptr = alloc(input.as_bytes().len());
    unsafe {
        let len = input.as_bytes().len();
        std::ptr::copy(input.as_bytes().as_ptr(), ptr, len);
        let res_ptr = upper(ptr, len);
        let data = Vec::from_raw_parts(res_ptr, len, len);
        let output = String::from_utf8(data).unwrap();
        println!("{}", output);
        // no need to call dealloc, since `Vec::from_raw_parts`
        // takes ownership of the underlying data, which goes
        // out of scope when this unsafe block returns
        // dealloc(res_ptr, input.as_bytes().len());
    }
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

/// Given a pointer to the start of a byte array and
/// its length, read a string, create its uppercase
/// representation, then return the pointer in
/// memory to it.
#[no_mangle]
pub unsafe fn upper(ptr: *mut u8, len: usize) -> *mut u8 {
    let data = Vec::from_raw_parts(ptr, len, len);
    let input_str = String::from_utf8(data).unwrap();
    let mut upper = input_str.to_ascii_uppercase().as_bytes().to_owned();
    let ptr = upper.as_mut_ptr();
    std::mem::forget(upper);
    ptr
}

/// The Node.js WASI runtime requires a `_start` function
/// for instantiating the module.
#[no_mangle]
pub fn _start() {}
