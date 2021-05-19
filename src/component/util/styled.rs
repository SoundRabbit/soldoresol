use async_std::sync::Mutex;
use lazy_static::lazy_static;
use std::any;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use wasm_bindgen::JsCast;

lazy_static! {
    static ref STYLED: Mutex<HashSet<u64>> = Mutex::new(set! {});
}

thread_local! {
    static SHEET: RefCell<Option<web_sys::CssStyleSheet>> = RefCell::new(None);
}

fn hash_of_type<C>() -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(any::type_name::<C>().as_bytes());
    hasher.finish()
}

fn styled_class<C>(class_name: &str) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(any::type_name::<C>().as_bytes());
    format!("_{:X}__{}", hasher.finish(), class_name)
}

pub trait Styled: Sized {
    fn style() -> Style;
    fn styled<T>(node: T) -> T {
        wasm_bindgen_futures::spawn_local(async {
            let mut styled = STYLED.lock().await;
            let component_id = hash_of_type::<Self>();
            if styled.get(&component_id).is_none() {
                let style = Self::style();
                style.write::<Self>();
                styled.insert(component_id);
            }
        });

        node
    }
    fn class(class_name: &str) -> String {
        styled_class::<Self>(class_name)
    }
}

pub struct Style {
    style: Vec<(String, Vec<(String, String)>)>,
    media: Vec<(String, Self)>,
}

impl Style {
    pub fn new() -> Self {
        Self {
            style: vec![],
            media: vec![],
        }
    }

    pub fn add(
        &mut self,
        selector: impl Into<String>,
        property: impl Into<String>,
        value: impl Into<String>,
    ) {
        let selector = selector.into();
        let property = property.into();
        let value = value.into();

        if let Some(class_idx) = self.style.iter().position(|s| s.0 == selector) {
            if let Some(property_idx) = self.style[class_idx].1.iter().position(|p| p.0 == property)
            {
                self.style[class_idx].1[property_idx].1 = value;
            } else {
                self.style[class_idx].1.push((property, value));
            }
        } else {
            self.style.push((selector, vec![(property, value)]));
        }
    }

    pub fn add_media(&mut self, query: impl Into<String>, style: Style) {
        let query = query.into();
        self.media.push((query, style));
    }

    fn rules<C>(&self) -> Vec<String> {
        let mut res = vec![];

        for (selector, definition_block) in &self.style {
            let mut rule = String::new();
            rule += format!(".{}", styled_class::<C>(selector)).as_str();
            rule += "{";
            for (property, value) in definition_block {
                rule += format!("{}:{};", property, value).as_str();
            }
            rule += "}";

            res.push(rule);
        }

        for (query, media_style) in &self.media {
            let mut rule = String::from("@media ");
            rule += query;
            rule += "{";
            for child_rule in &media_style.rules::<C>() {
                rule += child_rule;
            }
            rule += "}";
            res.push(rule);
        }

        res
    }

    fn write<C>(&self) {
        Self::add_style_element();

        for rule in &self.rules::<C>() {
            SHEET.with(|sheet| {
                if let Some(sheet) = sheet.borrow().as_ref() {
                    if let Err(err) = sheet
                        .insert_rule_with_index(rule.as_str(), sheet.css_rules().unwrap().length())
                    {
                        crate::debug::log_1(err);
                    }
                }
            });
        }
    }

    fn add_style_element() {
        SHEET.with(|sheet| {
            if sheet.borrow().is_none() {
                let style_element = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("style")
                    .unwrap()
                    .dyn_into::<web_sys::HtmlStyleElement>()
                    .unwrap();

                let head = web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_elements_by_tag_name("head")
                    .item(0)
                    .unwrap();

                let _ = head.append_child(&style_element);

                *sheet.borrow_mut() = Some(
                    style_element
                        .sheet()
                        .unwrap()
                        .dyn_into::<web_sys::CssStyleSheet>()
                        .unwrap(),
                );
            }
        });
    }
}

macro_rules! style {
    {
        $(
            $selector:literal {$(
                $property:literal : $value:expr
            );*;}
        )*
        $(
            @media $query:tt {$($media_style:tt)*}
        )*
    } => {{
        #[allow(unused_mut)]
        let mut style = Style::new();
        $(
            $(
                style.add(format!("{}", $selector), format!("{}", $property), format!("{}", $value));
            )*
        )*
        $({
            style.add_media($query, style!{$($media_style)*});
        })*
        style
    }};
}
