use crate::libs::js_object::Object;
use regex::Regex;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub struct GameSystem {
    pub id: String,
    pub name: String,
    pub sort_key: String,
}

pub struct GameSystemInfo {
    pub id: String,
    pub name: String,
    pub sort_key: String,
    pub command_pattern: Regex,
    pub help_message: String,
}

async fn get_json(url: &str) -> Option<Object> {
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");
    opts.mode(web_sys::RequestMode::Cors);

    let request = unwrap!(web_sys::Request::new_with_str_and_init(&url, &opts).ok(); None);

    let response = JsFuture::from(web_sys::window().unwrap().fetch_with_request(&request))
        .await
        .ok()
        .and_then(|response| response.dyn_into::<web_sys::Response>().ok());
    let response = unwrap!(response; None);

    let response = unwrap!(response.json().ok(); None);
    let response = unwrap!(JsFuture::from(response).await.ok(); None);

    response.dyn_into::<Object>().ok()
}

pub async fn game_system(root: &str) -> Vec<GameSystem> {
    let response = unwrap!(get_json(&format!("{}/v2/game_system", root)).await; vec![]);

    let game_systems = unwrap!(response.get("game_system"); vec![]);
    let game_systems = unwrap!(game_systems.dyn_into::<js_sys::Array>().ok(); vec![]);

    game_systems
        .to_vec()
        .into_iter()
        .filter_map(|game_system| {
            let game_system = unwrap!(game_system.dyn_into::<Object>().ok(); None);
            let id = unwrap!(game_system.get("id").and_then(|x| x.as_string()); None);
            let name = unwrap!(game_system.get("name").and_then(|x| x.as_string()); None);
            let sort_key = unwrap!(game_system.get("sort_key").and_then(|x| x.as_string()); None);

            Some(GameSystem { id, name, sort_key })
        })
        .collect()
}

pub async fn game_system_info(root: &str, id: &str) -> Option<GameSystemInfo> {
    let response = unwrap!(get_json(&format!("{}/v2/game_system/{}", root, id)).await; None);

    let id = unwrap!(response.get("id").and_then(|x| x.as_string()); None);
    let name = unwrap!(response.get("name").and_then(|x| x.as_string()); None);
    let sort_key = unwrap!(response.get("sort_key").and_then(|x| x.as_string()); None);
    let command_pattern =
        unwrap!(response.get("command_pattern").and_then(|x| x.as_string()); None);
    let help_message = unwrap!(response.get("help_message").and_then(|x| x.as_string()); None);

    let command_pattern = format!("(?i){}", command_pattern);
    let command_pattern = unwrap!(Regex::new(&command_pattern).ok(); None);

    Some(GameSystemInfo {
        id,
        name,
        sort_key,
        command_pattern,
        help_message,
    })
}
