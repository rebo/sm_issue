use serde::{Deserialize, Serialize};
use slotmap::new_key_type;
use slotmap::SlotMap;
use wasm_bindgen::prelude::*;
new_key_type! { struct PlayerKey; }

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let mut sm: SlotMap<PlayerKey, u32> = SlotMap::with_key();
    sm.insert(3);
    let j = serde_json::to_string(&sm).unwrap();
    log(&j);
    let sm2: SlotMap<PlayerKey, u32> = serde_json::from_str(&j).unwrap();

    alert(&format!("Hello, {}!", name));
}
