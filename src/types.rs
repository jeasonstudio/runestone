use super::*;

#[wasm_bindgen(typescript_custom_section)]
const Network: &'static str = r#"
export type Network = "main" | "test" | "signet" | "regtest";
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Network")]
    pub type Network;
}
