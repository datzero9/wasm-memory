use wasmtime::*;
use wasmtime_wasi::WasiCtxBuilder;

const WASM: &str = "rust.wasm";
// const WASM: &str = "as.wasm";
const ALLOC_FN: &str = "alloc";
const MEMORY: &str = "memory";
const ARRAY_SUM_FN: &str = "array_sum";

fn main() {
    test_array_sum();
}

fn test_array_sum() {
    let input = vec![1_u8, 2, 3, 4, 5];
    let res = array_sum(input).unwrap();
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
    let array_sum_fn = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, ARRAY_SUM_FN)?;

    let guest_ptr_offset = alloc_fn.call(&mut store, input.len() as i32)?;
    unsafe {
        let raw = memory.data_ptr(&store).offset(guest_ptr_offset as isize);
        raw.copy_from(input.as_ptr(), input.len());
    }

    let results = array_sum_fn.call(&mut store, (guest_ptr_offset, input.len() as i32))?;
    Ok(results)
}
