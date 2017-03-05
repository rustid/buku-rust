var ffi = require("ffi");
var assert = require("assert");

// Describe our API's functions, their return types and arguments.
var lib = ffi.Library("target/debug/libcount", {
  "createCounter": [ "pointer", [ "uint32" ] ],
  "destroyCounter": [ "void", [ "pointer" ] ],
  "getCounterValue": [ "uint32", [ "pointer" ] ],
  "incrementCounterBy": [ "uint32", [ "pointer", "uint32" ] ],
  "decrementCounterBy": [ "uint32", [ "pointer", "uint32" ] ]
});

var ptr = lib.createCounter(7);

var val = lib.getCounterValue(ptr);
console.log("created counter", val);
assert.equal(val, 7, "unexpected initial counter value");

val = lib.incrementCounterBy(ptr, 3);
console.log("val", val);
assert.equal(val, 10, "unexpected increment result");

val = lib.incrementCounterBy(ptr, 2);
console.log("val", val);
assert.equal(val, 12, "unexpected increment result");

val = lib.decrementCounterBy(ptr, 1);
console.log("val", val);
assert.equal(val, 11, "unexpected decrement result");

lib.destroyCounter(ptr);
console.log("destroyed counter");
