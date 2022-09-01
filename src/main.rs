use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

const WASM: &str = "rust.wasm";
// const WASM: &str = "as.wasm";
const ALLOC_FN: &str = "alloc";
const MEMORY: &str = "memory";
const ARRAY_SUM_FN: &str = "array_sum";
const UPPER_FN: &str = "upper";
const DEALLOC_FN: &str = "dealloc";

fn main() {
    test_array_sum();
    test_upper();
}

fn test_array_sum() {
    let input = vec![1_u8, 2, 3, 4, 5];
    let res = array_sum(input).unwrap();
    println!("Result from running {}: {:#?}", WASM, res);
}

fn test_upper() {
    let input = "this should be uppercase";
    let res = upper(input.to_string()).unwrap();
    println!("Result from running {}: {:#?}", WASM, res);
}

fn array_sum(input: Vec<u8>) -> Result<i32, anyhow::Error> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, WASM)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdin()
        .inherit_stdout()
        .inherit_stderr()
        .build();
    let mut store = Store::new(&engine, wasi_ctx);

    let instance = linker.instantiate(&mut store, &module)?;

    let memory = instance
        .get_memory(&mut store, MEMORY)
        .ok_or_else(|| anyhow::format_err!("failed to find `memory` export"))?;
    let alloc_fn = instance.get_typed_func::<i32, i32, _>(&mut store, ALLOC_FN)?;

    let guest_ptr_offset = alloc_fn.call(&mut store, input.len() as i32)?;
    unsafe {
        let raw = memory.data_ptr(&store).offset(guest_ptr_offset as isize);
        raw.copy_from(input.as_ptr(), input.len());
    }

    let array_sum_fn = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, ARRAY_SUM_FN)?;
    let results = array_sum_fn.call(&mut store, (guest_ptr_offset, input.len() as i32))?;
    Ok(results)
}

fn upper(input: String) -> Result<String, anyhow::Error> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, WASM)?;
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)?;

    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdin()
        .inherit_stdout()
        .inherit_stderr()
        .build();
    let mut store = Store::new(&engine, wasi_ctx);

    let instance = linker.instantiate(&mut store, &module)?;
    let memory = instance
        .get_memory(&mut store, MEMORY)
        .ok_or_else(|| anyhow::format_err!("failed to find `memory` export"))?;
    let alloc_fn = instance.get_typed_func::<i32, i32, _>(&mut store, ALLOC_FN)?;

    let len = input.as_bytes().len() as i32;
    let guest_ptr_offset = alloc_fn.call(&mut store, len)?;
    unsafe {
        let raw = memory.data_ptr(&store).offset(guest_ptr_offset as isize);
        raw.copy_from(input.as_ptr(), input.len());
    }

    let upper_fn = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, UPPER_FN)?;
    let res_ptr = upper_fn.call(&mut store, (guest_ptr_offset, len))?;

    let data = memory
        .data(&store)
        .get(res_ptr as u32 as usize..)
        .and_then(|arr| arr.get(..len as u32 as usize));
    let str = match data {
        Some(data) => match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(_) => return Err(anyhow::Error::msg("invalid utf-8")),
        },
        None => return Err(anyhow::Error::msg("pointer/length out of bounds")),
    };
    let res = String::from(str);

    let dealloc_fn = instance.get_typed_func::<(i32, i32), (), _>(&mut store, DEALLOC_FN)?;
    dealloc_fn.call(&mut store, (guest_ptr_offset, len))?;

    Ok(res)
}
