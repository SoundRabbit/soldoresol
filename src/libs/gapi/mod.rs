use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type GoogleAPI;
    pub static gapi: GoogleAPI;

    #[wasm_bindgen(method, getter)]
    pub fn client(this: &GoogleAPI) -> GoogleAPIClient;

    #[wasm_bindgen(method, getter)]
    pub fn auth2(this: &GoogleAPI) -> GoogleAPIAuth2;
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleAPIClient;

    #[wasm_bindgen(method)]
    pub fn init(this: &GoogleAPIClient, args: &JsValue) -> GoogleThenalbe;
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleAPIAuth2;

    #[wasm_bindgen(method, js_name = "getAuthInstance")]
    pub fn get_auth_instamce(this: &GoogleAPIAuth2) -> GoogleAPIGoogleAuth;
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleAPIGoogleAuth;

    #[wasm_bindgen(method, js_name = "signIn")]
    pub fn sign_in(this: &GoogleAPIGoogleAuth);
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleThenalbe;

    #[wasm_bindgen(method)]
    pub fn then(this: &GoogleThenalbe, resolve: &js_sys::Function, reject: &js_sys::Function);
}
