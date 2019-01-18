use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo() -> Option<Foo> {
    Some(Foo::Foo)
}

#[wasm_bindgen]
pub fn bar() -> Foo {
    Foo::Bar
}

pub enum Foo {
    Foo = 1,
    Bar = 2,
}

impl wasm_bindgen::describe::WasmDescribe for Foo {
    fn describe() {
        wasm_bindgen::describe::inform(wasm_bindgen::describe::I32)
    }
}

impl wasm_bindgen::convert::IntoWasmAbi for Foo {
    type Abi = i32;

    fn into_abi(self, _extra: &mut dyn wasm_bindgen::convert::Stack) -> Self::Abi {
        self as Self::Abi
    }
}

impl wasm_bindgen::convert::OptionIntoWasmAbi for Foo {
    fn none() -> Self::Abi {
        0
    }
}
