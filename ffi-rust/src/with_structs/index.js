var ffi = require("ffi");
var assert = require("assert");

var ref = require("ref");
var StructType = require("ref-struct");

// Define a struct type with properties matching the order and type of the
// receiving struct type.
var Args = StructType({
  init: ref.types.uint32,
  by: ref.types.uint32
});

var lib = ffi.Library("target/debug/libcount", {
  "createCounter": [ "pointer", [ Args ] ],
  "destroyCounter": [ "void", [ "pointer" ] ],
  "getCounterValue": [ "uint32", [ "pointer" ] ],
  "incrementCounterBy": [ "uint32", [ "pointer" ] ],
  "decrementCounterBy": [ "uint32", [ "pointer" ] ]
});

var ptr = lib.createCounter(new Args({ init: 4, by: 2 }));

var val = lib.getCounterValue(ptr);
console.log("created counter", val);
assert.equal(val, 4, "unexpected initial counter value");

val = lib.incrementCounterBy(ptr);
console.log("val", val);
assert.equal(val, 6, "unexpected increment result");

val = lib.incrementCounterBy(ptr);
console.log("val", val);
assert.equal(val, 8, "unexpected increment result");

val = lib.decrementCounterBy(ptr);
console.log("val", val);
assert.equal(val, 6, "unexpected decrement result");

lib.destroyCounter(ptr);
console.log("destroyed counter");
