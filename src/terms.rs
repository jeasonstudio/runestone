use super::types::*;
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize, Default, Copy, Clone)]
#[wasm_bindgen]
pub struct Terms {
    amount: Option<u128>,
    cap: Option<u128>,

    #[wasm_bindgen(readonly)]
    pub height: Option<Range>,

    #[wasm_bindgen(readonly)]
    pub offset: Option<Range>,

    #[wasm_bindgen(skip)]
    #[serde(skip)]
    pub source: ordinals::Terms,
}

#[wasm_bindgen]
impl Terms {
    #[wasm_bindgen(constructor)]
    pub fn new(amount: Option<js_sys::BigInt>, cap: Option<js_sys::BigInt>) -> Self {
        let default_range = Range::default();
        let source = ordinals::Terms {
            amount: amount.map(|amount| amount.try_into().unwrap()),
            cap: cap.map(|cap| cap.try_into().unwrap()),
            height: (default_range.start, default_range.end),
            offset: (default_range.start, default_range.end),
        };
        Self::from(source)
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> Option<js_sys::BigInt> {
        match self.source.amount {
            Some(amount) => Some(js_sys::BigInt::from(amount)),
            None => None,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn cap(&self) -> Option<js_sys::BigInt> {
        match self.source.cap {
            Some(cap) => Some(js_sys::BigInt::from(cap)),
            None => None,
        }
    }

    #[wasm_bindgen(js_name = "height", setter)]
    pub fn set_height(&mut self, values: Range) {
        let height_source = (values.start, values.end);
        self.source.height = height_source;
        self.height = Some(values);
    }

    #[wasm_bindgen(js_name = "offset", setter)]
    pub fn set_offset(&mut self, values: Range) {
        let offset_source = (values.start, values.end);
        self.source.offset = offset_source;
        self.offset = Some(values);
    }

    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json_value(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
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
            source,
        }
    }
}
