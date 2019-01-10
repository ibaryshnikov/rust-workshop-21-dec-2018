use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen(js_name = doubleList)]
pub fn double_list(list: &[i32]) -> Vec<i32> {
    list.iter().map(|x| x * 2).collect()
}

#[wasm_bindgen(js_name = addElement)]
pub fn add_element(name: String) -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window");
    let document = window.document().expect("should have a document");
    let body = document.body().expect("should have a body");

    let p = document.create_element("p")?;
    p.set_inner_html(&format!("New element with name: {}", name));

    body.append_child(&p)?;

    Ok(())
}
