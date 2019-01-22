import * as bindings from './wasm_bindgen_option'

console.log(`Some(Foo::Foo) = ${bindings.foo()}`)
console.log(`Foo::Bar = ${bindings.bar()}`)
console.log(`3 = ${bindings.baz()}`)
