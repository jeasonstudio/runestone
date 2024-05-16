use super::*;
use serde::Serializer;

#[derive(Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Rune {
    value: u128,
}

#[wasm_bindgen]
impl Rune {
    #[wasm_bindgen(constructor)]
    pub fn new(value: js_sys::BigInt) -> Self {
        Self::from(value)
    }

    fn source(&self) -> ordinals::Rune {
        ordinals::Rune::from(self.clone())
    }

    #[wasm_bindgen(getter, js_name = "value")]
    pub fn get_value(&self) -> js_sys::BigInt {
        js_sys::BigInt::from(self.value)
    }

    #[wasm_bindgen(setter, js_name = "value")]
    pub fn set_value(&mut self, new_value: js_sys::BigInt) {
        let rune_value: u128 = new_value.try_into().unwrap();
        self.value = rune_value;
    }

    #[wasm_bindgen(js_name = "isReserved")]
    pub fn is_reserved(&self) -> bool {
        self.source().is_reserved()
    }

    #[wasm_bindgen]
    pub fn commitment(&self) -> String {
        let commit = self.source().commitment();
        encode_bytes_to_hex(commit)
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.source().to_string()
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> Result<JsValue, Error> {
        serde_wasm_bindgen::to_value(&self)
    }

    // static
    #[wasm_bindgen(js_name = "firstRuneHeight")]
    pub fn first_rune_height(network: Network) -> u32 {
        let network: bitcoin::Network =
            bitcoin::Network::from_core_arg(&network.as_string().unwrap()).unwrap();
        ordinals::Rune::first_rune_height(network)
    }

    #[wasm_bindgen(js_name = "minimumAtHeight")]
    pub fn minimum_at_height(network: Network, height: u32) -> Self {
        let network: bitcoin::Network =
            bitcoin::Network::from_core_arg(&network.as_string().unwrap()).unwrap();
        let source: ordinals::Rune =
            ordinals::Rune::minimum_at_height(network, ordinals::Height(height));
        Self::from(source)
    }

    #[wasm_bindgen]
    pub fn reserved(block: u64, tx: u32) -> Self {
        let source: ordinals::Rune = ordinals::Rune::reserved(block, tx);
        Self::from(source)
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(s: &str) -> Result<Rune, JsValue> {
        let source: ordinals::Rune = ordinals::Rune::from_str(s).unwrap();
        Ok(Self::from(source))
    }
}

impl From<u128> for Rune {
    fn from(value: u128) -> Self {
        Self { value }
    }
}

impl From<Rune> for u128 {
    fn from(rune: Rune) -> u128 {
        rune.value
    }
}

impl From<js_sys::BigInt> for Rune {
    fn from(value: js_sys::BigInt) -> Self {
        let rune_result: Result<u128, _> = value.try_into();
        match rune_result {
            Ok(value) => Rune::from(value),
            Err(_) => throw_str("Cannot convert BigInt to Rune"),
        }
    }
}

impl From<SpacedRune> for Rune {
    fn from(source: SpacedRune) -> Self {
        source.rune.clone()
    }
}

impl From<ordinals::Rune> for Rune {
    fn from(source: ordinals::Rune) -> Self {
        Self { value: source.0 }
    }
}

impl From<Rune> for ordinals::Rune {
    fn from(source: Rune) -> Self {
        ordinals::Rune(source.value)
    }
}

impl Serialize for Rune {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u128(self.value)
    }
}
