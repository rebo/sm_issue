wasm_bindgen example that does not work when deserializing slotmaps
----

npm 
npm run serve

this panics when using serde deserialize serde_json::from_str on a serialized slotmap.