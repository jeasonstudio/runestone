use super::*;

#[derive(Default, Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TermsParams {
    pub amount: Option<u128>,
    pub cap: Option<u128>,
    pub height: Option<RangeParams>,
    pub offset: Option<RangeParams>,
}

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen(inspectable)]
pub struct Terms {
    amount: Option<u128>,
    cap: Option<u128>,

    #[wasm_bindgen]
    pub height: Option<Range>,

    #[wasm_bindgen]
    pub offset: Option<Range>,
}

#[wasm_bindgen]
impl Terms {
    #[wasm_bindgen(constructor)]
    pub fn new(params: Option<TermsParams>) -> Self {
        match params {
            None => Self::default(),
            Some(params) => Self {
                amount: params.amount,
                cap: params.cap,
                height: match params.height {
                    Some(height) => Some(Range::new(height.start, height.end)),
                    None => None,
                },
                offset: match params.offset {
                    Some(offset) => Some(Range::new(offset.start, offset.end)),
                    None => None,
                },
            },
        }
    }

    // fn source(&self) -> ordinals::Terms {
    //     ordinals::Terms::from(self.clone())
    // }

    #[wasm_bindgen(getter, js_name = "amount")]
    pub fn get_amount(&self) -> Option<js_sys::BigInt> {
        match self.amount {
            Some(amount) => Some(js_sys::BigInt::from(amount)),
            None => None,
        }
    }

    #[wasm_bindgen(setter, js_name = "amount")]
    pub fn set_amount(&mut self, new_amount: Option<js_sys::BigInt>) {
        self.amount = match new_amount {
            Some(amount) => Some(amount.try_into().unwrap()),
            None => None,
        };
    }

    #[wasm_bindgen(getter, js_name = "cap")]
    pub fn get_cap(&self) -> Option<js_sys::BigInt> {
        match self.cap {
            Some(cap) => Some(js_sys::BigInt::from(cap)),
            None => None,
        }
    }

    #[wasm_bindgen(setter, js_name = "cap")]
    pub fn set_cap(&mut self, new_cap: Option<js_sys::BigInt>) {
        self.cap = match new_cap {
            Some(cap) => Some(cap.try_into().unwrap()),
            None => None,
        };
    }

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> Result<JsValue, Error> {
        let serializer = serde_wasm_bindgen::Serializer::json_compatible();
        self.serialize(&serializer)
    }
}

impl From<ordinals::Terms> for Terms {
    fn from(source: ordinals::Terms) -> Self {
        let height = Some(Range::from(source.height));
        let offset = Some(Range::from(source.offset));
        Self {
            amount: source.amount,
            cap: source.cap,
            height,
            offset,
        }
    }
}

impl From<Terms> for ordinals::Terms {
    fn from(source: Terms) -> Self {
        let height = match source.height {
            Some(height) => (height.start, height.end),
            None => (None, None),
        };
        let offset = match source.offset {
            Some(offset) => (offset.start, offset.end),
            None => (None, None),
        };
        ordinals::Terms {
            amount: source.amount,
            cap: source.cap,
            height,
            offset,
        }
    }
}
