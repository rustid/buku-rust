var ffi = require("ffi");
var assert = require("assert");

var ref = require("ref");
var ArrayType = require("ref-array");

// Define a new array type.
var UInt32Array = ArrayType(ref.types.uint32);

var lib = ffi.Library("target/debug/libcount", {
  "createCounter": [ "pointer", [ "uint32" ] ],
  "destroyCounter": [ "void", [ "pointer" ] ],
  "getCounterValue": [ "uint32", [ "pointer" ] ],
  "incrementCounterBy": [ "uint32", [ "pointer", UInt32Array, "size_t" ] ],
  "decrementCounterBy": [ "uint32", [ "pointer", UInt32Array, "size_t" ] ]
});

var ptr = lib.createCounter(4);

var val = lib.getCounterValue(ptr);
console.log("created counter", val);
assert.equal(val, 4, "unexpected initial counter value");

var bys = new UInt32Array([1, 1, 2]);
val = lib.incrementCounterBy(ptr, bys, bys.length);
console.log("val", val);
assert.equal(val, 8, "unexpected increment result");

bys = new UInt32Array([2, 2, 1]);
val = lib.incrementCounterBy(ptr, bys, bys.length);
console.log("val", val);
assert.equal(val, 13, "unexpected increment result");

bys = new UInt32Array([0, 3, 5]);
val = lib.decrementCounterBy(ptr, bys, bys.length);
console.log("val", val);
assert.equal(val, 5, "unexpected decrement result");

lib.destroyCounter(ptr);
console.log("destroyed counter");
