const fs = require("fs");

const filename = "./rust.wasm";
// const filename = "./as.wasm";

const module_bytes = fs.readFileSync(filename);

(async () => {
    const mod = new WebAssembly.Module(module_bytes);
    const instance = await WebAssembly.instantiate(mod, {});

    arraySum([1, 2, 3, 4, 5], instance);
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

