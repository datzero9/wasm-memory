const fs = require("fs");

const filename = "./rust.wasm";
// const filename = "./as.wasm";

const module_bytes = fs.readFileSync(filename);

(async () => {
    const mod = new WebAssembly.Module(module_bytes);
    const instance = await WebAssembly.instantiate(mod, {});

    arraySum([1, 2, 3, 4, 5], instance);
    upper("this should be uppercase", instance);
})();

// Invoke the `array_sum` exported method and log the result to the console
function arraySum(array, instance) {
    var ptr = copyMemory(array, instance);
    var res = instance.exports.array_sum(ptr, array.length);
    console.log(`Result running ${filename}: ${res}`);

    // if running the AssemblyScript module, this should also
    // be executed, particularly for long-running modules
    //instance.exports.__release(ptr);
}

// Copy `data` into the `instance` exported memory buffer.
function copyMemory(data, instance) {
    var ptr = instance.exports.alloc(data.length);
    var mem = new Uint8Array(instance.exports.memory.buffer, ptr, data.length);
    mem.set(new Uint8Array(data));
    return ptr;
}

// Invoke the `upper` function from the module and log the result to the console.
function upper(input, instance) {
    var bytes = new TextEncoder("utf-8").encode(input);
    var ptr = copyMemory(bytes, instance);
    var res_ptr = instance.exports.upper(ptr, bytes.length);
    var result = readString(res_ptr, bytes.length, instance);
    console.log(result);
    deallocGuestMemory(res_ptr, bytes.length, instance);
}

// Read a string from the instance's memory.
function readString(ptr, len, instance) {
    var m = new Uint8Array(instance.exports.memory.buffer, ptr, len);
    var decoder = new TextDecoder("utf-8");
    return decoder.decode(m.slice(0, len));
}

function deallocGuestMemory(ptr, len, instance) {
    instance.exports.dealloc(ptr, len);
}