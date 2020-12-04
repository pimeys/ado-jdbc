use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct JdbcString {
    inner: connection_string::JdbcString,
}

#[wasm_bindgen]
impl JdbcString {
    #[wasm_bindgen(constructor)]
    pub fn new(s: &str) -> Result<JdbcString, JsValue> {
        let inner = if s.starts_with("jdbc") {
            s.parse()
        } else {
            format!("jdbc:{}", s).parse()
        }
        .map_err(|err| JsValue::from_str(&format!("{}", err)))?;

        Ok(Self { inner })
    }

    pub fn sub_protocol(&self) -> String {
        self.inner.sub_protocol().to_string()
    }

    pub fn server_name(&self) -> Option<String> {
        self.inner.server_name().map(|s| s.to_string())
    }

    pub fn instance_name(&self) -> Option<String> {
        self.inner.instance_name().map(|s| s.to_string())
    }

    pub fn port(&self) -> Option<u16> {
        self.inner.port()
    }

    pub fn get_param(&self, key: &str) -> Option<String> {
        self.inner.properties().get(key).map(|s| s.to_string())
    }

    pub fn set_param(&mut self, key: &str, value: &str) -> Option<String> {
        self.inner.properties_mut().insert(key.into(), value.into())
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.inner)
    }
}
