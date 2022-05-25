mod bind;
use crate::libs::js_object::Object;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

pub struct DynamicLoader {
    instance: bind::DynamicLoader,
    available_game_systems: Vec<Object>,
}

pub struct GameSystemInfo {
    id: String,
    name: String,
    class_name: String,
    sort_key: String,
}

pub struct GameSystemClass {
    id: String,
    name: String,
    sort_key: String,
    help_message: String,
    command_pattern: js_sys::RegExp,
    eval: js_sys::Function,
    this: JsValue,
}

pub struct CommandResult {
    pub text: String,
    pub detailed_rands: Vec<Rand>,
    pub secret: bool,
    pub success: bool,
    pub failure: bool,
    pub critical: bool,
    pub fumble: bool,
}

pub struct Rand {
    pub kind: String,
    pub sides: i32,
    pub value: i32,
}

impl DynamicLoader {
    pub fn new() -> Self {
        let instance = bind::DynamicLoader::new();

        let available_game_systems = instance
            .list_available_game_systems()
            .to_vec()
            .into_iter()
            .filter_map(|game_system_info| game_system_info.dyn_into::<Object>().ok())
            .collect::<Vec<_>>();

        Self {
            instance,
            available_game_systems,
        }
    }

    pub fn available_game_systems(&self) -> &Vec<Object> {
        &self.available_game_systems
    }

    pub async fn dynamic_load(&self, id: &str) -> Option<GameSystemClass> {
        let game_system_class = JsFuture::from(self.instance.dynamic_load(id)).await;
        let game_system_class = unwrap!(game_system_class.ok(); None);
        let game_system_class = unwrap!(game_system_class.dyn_into::<Object>().ok(); None);

        game_system_class.try_as::<GameSystemClass>()
    }
}

impl GameSystemInfo {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn class_name(&self) -> &String {
        &self.class_name
    }

    pub fn sort_key(&self) -> &String {
        &self.sort_key
    }
}

impl TryFrom<&Object> for GameSystemInfo {
    type Error = ();

    fn try_from(data: &Object) -> Result<Self, Self::Error> {
        let id = unwrap!(data.get("id").and_then(|x| x.as_string()); Err(()));
        let name = unwrap!(data.get("name").and_then(|x| x.as_string()); Err(()));
        let class_name = unwrap!(data.get("className").and_then(|x| x.as_string()); Err(()));
        let sort_key = unwrap!(data.get("sortKey").and_then(|x| x.as_string()); Err(()));

        Ok(Self {
            id,
            name,
            class_name,
            sort_key,
        })
    }
}

impl GameSystemClass {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn sort_key(&self) -> &String {
        &self.sort_key
    }

    pub fn help_message(&self) -> &String {
        &self.help_message
    }

    pub fn command_pattern(&self) -> &js_sys::RegExp {
        &self.command_pattern
    }

    pub fn eval(&self, command: &str) -> Option<CommandResult> {
        crate::debug::log_2("command", command);
        let result = unwrap!(self.eval.call1(&self.this, &JsValue::from(command)).ok(); None);
        crate::debug::log_2("command", &result);
        let result = unwrap!(result.dyn_into::<Object>().ok(); None);
        crate::debug::log_2("command", &result);
        result.try_as::<CommandResult>()
    }
}

impl TryFrom<&Object> for GameSystemClass {
    type Error = ();

    fn try_from(data: &Object) -> Result<Self, Self::Error> {
        let id = unwrap!(data.get("ID").and_then(|x| x.as_string()); Err(()));
        let name = unwrap!(data.get("NAME").and_then(|x| x.as_string()); Err(()));
        let sort_key = unwrap!(data.get("SORT_KEY").and_then(|x| x.as_string()); Err(()));
        let help_message = unwrap!(data.get("HELP_MESSAGE").and_then(|x| x.as_string()); Err(()));
        let command_pattern = unwrap!(data.get("COMMAND_PATTERN").and_then(|x| x.dyn_into::<js_sys::RegExp>().ok()); Err(()));
        let eval =
            unwrap!(data.get("eval").and_then(|x| x.dyn_into::<js_sys::Function>().ok()); Err(()));

        Ok(Self {
            id,
            name,
            sort_key,
            help_message,
            command_pattern,
            eval,
            this: data.clone().into(),
        })
    }
}

impl TryFrom<&Object> for CommandResult {
    type Error = ();

    fn try_from(data: &Object) -> Result<Self, Self::Error> {
        let text = unwrap!(data.get("text").and_then(|x| x.as_string()); Err(()));
        let secret = unwrap!(data.get("secret").and_then(|x| x.as_bool()); Err(()));
        let success = unwrap!(data.get("success").and_then(|x| x.as_bool()); Err(()));
        let failure = unwrap!(data.get("failure").and_then(|x| x.as_bool()); Err(()));
        let critical = unwrap!(data.get("critical").and_then(|x| x.as_bool()); Err(()));
        let fumble = unwrap!(data.get("fumble").and_then(|x| x.as_bool()); Err(()));

        let detailed_rands =
            unwrap!(data.get("detailedRands").map(|x| js_sys::Array::from(&x)); Err(()))
                .to_vec()
                .into_iter()
                .filter_map(|x| x.dyn_into::<Object>().ok())
                .filter_map(|x| x.try_as::<Rand>())
                .collect::<Vec<_>>();

        Ok(Self {
            text,
            secret,
            success,
            failure,
            critical,
            fumble,
            detailed_rands,
        })
    }
}

impl TryFrom<&Object> for Rand {
    type Error = ();

    fn try_from(data: &Object) -> Result<Self, Self::Error> {
        let kind = unwrap!(data.get("kind").and_then(|x| x.as_string()); Err(()));
        let sides = unwrap!(data.get("kind").and_then(|x| x.as_f64()).map(|x| x as i32); Err(()));
        let value = unwrap!(data.get("value").and_then(|x| x.as_f64()).map(|x| x as i32); Err(()));

        Ok(Self { kind, sides, value })
    }
}
