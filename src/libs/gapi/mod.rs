use super::js_object::Object;
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

    #[wasm_bindgen(method, getter)]
    pub fn drive(this: &GoogleAPIClient) -> GoogleAPIClientDrive;
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleAPIClientDrive;

    #[wasm_bindgen(method, getter)]
    pub fn files(this: &GoogleAPIClientDrive) -> GoogleAPIClientDriveFiles;
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleAPIClientDriveFiles;

    #[wasm_bindgen(method)]
    pub fn create(this: &GoogleAPIClientDriveFiles, args: &JsValue) -> GoogleThenalbe;

    #[wasm_bindgen(method)]
    pub fn list(this: &GoogleAPIClientDriveFiles, args: &JsValue) -> GoogleThenalbe;
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleAPIAuth2;

    #[wasm_bindgen(method, js_name = "getAuthInstance")]
    pub fn get_auth_instance(this: &GoogleAPIAuth2) -> GoogleAPIGoogleAuth;
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleAPIGoogleAuth;

    #[wasm_bindgen(method, js_name = "signIn")]
    pub fn sign_in(this: &GoogleAPIGoogleAuth);

    #[wasm_bindgen(method, js_name = "signOut")]
    pub fn sign_out(this: &GoogleAPIGoogleAuth);

    #[wasm_bindgen(method, getter, js_name = "isSignedIn")]
    pub fn is_signed_in(this: &GoogleAPIGoogleAuth) -> GoogleAPIGoogleAuthIsSignedIn;
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleAPIGoogleAuthIsSignedIn;

    #[wasm_bindgen(method)]
    pub fn get(this: &GoogleAPIGoogleAuthIsSignedIn) -> bool;

    #[wasm_bindgen(method)]
    pub fn listen(this: &GoogleAPIGoogleAuthIsSignedIn, callback: &js_sys::Function);
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleThenalbe;

    #[wasm_bindgen(method)]
    pub fn then(
        this: &GoogleThenalbe,
        resolve: Option<&js_sys::Function>,
        reject: Option<&js_sys::Function>,
    );
}

#[wasm_bindgen]
extern "C" {
    pub type GoogleResponse;

    #[wasm_bindgen(method, getter)]
    pub fn result(this: &GoogleResponse) -> Object;
}
