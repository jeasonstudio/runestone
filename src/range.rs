use super::*;

#[derive(Default, Serialize, Deserialize, Tsify, Clone)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RangeParams {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Default, Serialize, Deserialize, Copy, Clone)]
#[wasm_bindgen]
pub struct Range {
    #[wasm_bindgen]
    pub start: Option<u64>,

    #[wasm_bindgen]
    pub end: Option<u64>,
}

#[wasm_bindgen]
impl Range {
    #[wasm_bindgen(constructor)]
    pub fn new(start: Option<u64>, end: Option<u64>) -> Self {
        Self { start, end }
    }
}

impl From<(Option<u64>, Option<u64>)> for Range {
    fn from(source: (Option<u64>, Option<u64>)) -> Self {
        Self {
            start: source.0,
            end: source.1,
        }
    }
}

impl From<Range> for (Option<u64>, Option<u64>) {
    fn from(source: Range) -> (Option<u64>, Option<u64>) {
        (source.start, source.end)
    }
}
