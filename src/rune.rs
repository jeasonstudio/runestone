use super::*;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Rune(u128);

#[wasm_bindgen]
impl Rune {
    #[wasm_bindgen(constructor)]
    pub fn new(value: js_sys::BigInt) -> Self {
        let rune_value: u128 = value.try_into().unwrap();
        Self(rune_value)
    }

    fn source(&self) -> ordinals::Rune {
        ordinals::Rune::from(self.clone())
    }

    #[wasm_bindgen(getter, js_name = "value")]
    pub fn get_value(&self) -> js_sys::BigInt {
        js_sys::BigInt::from(self.0)
    }

    #[wasm_bindgen(setter, js_name = "value")]
    pub fn set_value(&mut self, new_value: js_sys::BigInt) {
        let rune_value: u128 = new_value.try_into().unwrap();
        self.0 = rune_value;
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

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        self.source().to_string()
    }

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> js_sys::BigInt {
        js_sys::BigInt::from(self.0)
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
        Self(value)
    }
}

impl From<js_sys::BigInt> for Rune {
    fn from(value: js_sys::BigInt) -> Self {
        Self(value.try_into().unwrap())
    }
}

impl From<SpacedRune> for Rune {
    fn from(source: SpacedRune) -> Self {
        Self(source.rune.0)
    }
}

impl From<ordinals::Rune> for Rune {
    fn from(source: ordinals::Rune) -> Self {
        Self(source.0)
    }
}

impl From<Rune> for ordinals::Rune {
    fn from(source: Rune) -> Self {
        ordinals::Rune(source.0)
    }
}
