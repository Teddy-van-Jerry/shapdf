use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_script(script: &str) -> Result<Box<[u8]>, JsValue> {
    let bytes = shapdf::render_script_to_bytes(script)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;
    Ok(bytes.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::render_script;

    #[test]
    fn generates_pdf_bytes() {
        let script = "page default\nline 10mm 10mm 50mm 50mm";
        let bytes = render_script(script).expect("render succeeds");
        assert!(bytes.len() > 0);
    }
}
