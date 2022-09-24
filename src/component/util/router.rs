use wasm_bindgen::prelude::*;

macro_rules! router {
    {$path:ident then _ => $default:expr} => {{
        $default
    }};

    {$path:ident then $pattern:tt $(($capture:ident))? => $html:expr, $($patterns:tt $(($captures:ident))? => $htmls:expr),+} => {{
        let pattern = format!("^{}$", $pattern);
        let pattern = regex::Regex::new(pattern.as_str()).unwrap();
        if let Some(_capture) = pattern.captures(&$path) {
            $(
                let $capture = _capture;
            )?
            $html
        } else {
            router! {$path then $($patterns $(($captures))? => $htmls),+}
        }
    }};

    {$($patterns:tt $(($captures:ident))? => $htmls:expr),+} => {{
        let path = web_sys::window().unwrap().location().pathname().unwrap();
        let path = path.as_str();

        router! {path then $($patterns $(($captures))? => $htmls),+}
    }}
}

pub fn jump_to(path: &str) {
    if crate::is_dev_mode() {
        let _ = web_sys::window()
            .unwrap()
            .history()
            .unwrap()
            .push_state_with_url(&JsValue::null(), "", Some(path));
    } else {
        let _ = web_sys::window()
            .unwrap()
            .history()
            .unwrap()
            .replace_state_with_url(&JsValue::null(), "", Some(path));
    }
}
