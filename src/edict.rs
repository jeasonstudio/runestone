use super::*;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[wasm_bindgen(inspectable)]
pub struct Edict {
    #[wasm_bindgen]
    pub id: RuneId,

    amount: u128,

    #[wasm_bindgen]
    pub output: u32,
}

#[wasm_bindgen]
impl Edict {
    #[wasm_bindgen(constructor)]
    pub fn new(id: RuneId, amount: js_sys::BigInt, output: Option<u32>) -> Self {
        let amount: u128 = amount.try_into().unwrap();
        let output = match output {
            Some(output) => output,
            None => 0,
        };
        Self { id, output, amount }
    }

    // fn source(&self) -> ordinals::Edict {
    //     ordinals::Edict::from(self.clone())
    // }

    #[wasm_bindgen(getter, js_name = "amount")]
    pub fn get_amount(&self) -> js_sys::BigInt {
        js_sys::BigInt::from(self.amount)
    }

    #[wasm_bindgen(setter, js_name = "amount")]
    pub fn set_amount(&mut self, new_amount: js_sys::BigInt) {
        let new_amount: u128 = new_amount.try_into().unwrap();
        self.amount = new_amount;
    }

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> Result<JsValue, Error> {
        let serializer = serde_wasm_bindgen::Serializer::json_compatible();
        self.serialize(&serializer)
    }
}

impl From<ordinals::Edict> for Edict {
    fn from(source: ordinals::Edict) -> Self {
        Self {
            id: RuneId::from(source.id),
            output: source.output,
            amount: source.amount,
        }
    }
}

impl From<Edict> for ordinals::Edict {
    fn from(source: Edict) -> Self {
        ordinals::Edict {
            id: ordinals::RuneId::from(source.id),
            output: source.output,
            amount: source.amount,
        }
    }
}
