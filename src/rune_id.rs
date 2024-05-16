use super::*;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[wasm_bindgen(inspectable)]
pub struct RuneId {
    #[wasm_bindgen]
    pub block: u64,

    #[wasm_bindgen]
    pub tx: u32,
}

#[wasm_bindgen]
impl RuneId {
    #[wasm_bindgen(constructor)]
    pub fn new(block: u64, tx: u32) -> Self {
        Self { block, tx }
    }

    fn source(&self) -> ordinals::RuneId {
        ordinals::RuneId::from(self.clone())
    }

    #[wasm_bindgen]
    pub fn delta(&self, next: Self) -> Option<Range> {
        let next_rune_id = ordinals::RuneId {
            block: next.block,
            tx: next.tx,
        };
        let tuple = self.source().delta(next_rune_id);
        match tuple {
            Some((start, end)) => Some(Range {
                start: Some(start as u64),
                end: Some(end as u64),
            }),
            None => None,
        }
    }

    #[wasm_bindgen]
    pub fn next(&self, block: u64, tx: u32) -> Option<RuneId> {
        let next_rune_id = self.source().next(block as u128, tx as u128);
        match next_rune_id {
            Some(value) => Some(Self::from(value)),
            None => None,
        }
    }

    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(&self) -> String {
        self.source().to_string()
    }

    #[wasm_bindgen(js_name = "fromString")]
    pub fn from_string(s: &str) -> Result<RuneId, JsValue> {
        let source = ordinals::RuneId::from_str(s).unwrap();
        Ok(Self::from(source))
    }

    #[wasm_bindgen(js_name = "valueOf")]
    pub fn value_of(&self) -> Result<JsValue, Error> {
        let serializer = serde_wasm_bindgen::Serializer::json_compatible();
        self.serialize(&serializer)
    }
}

impl From<ordinals::RuneId> for RuneId {
    fn from(source: ordinals::RuneId) -> Self {
        Self {
            block: source.block,
            tx: source.tx,
        }
    }
}

impl From<RuneId> for ordinals::RuneId {
    fn from(source: RuneId) -> Self {
        ordinals::RuneId {
            block: source.block,
            tx: source.tx,
        }
    }
}
