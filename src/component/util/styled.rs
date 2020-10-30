use kagura::prelude::*;
use std::any;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use wasm_bindgen::JsCast;

thread_local! {static STYLED: RefCell<HashSet<u64>> = RefCell::new(set!{})}
thread_local! {static SHEET: RefCell<Option<web_sys::CssStyleSheet>> = RefCell::new(None)}

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

pub trait Styled: Constructor {
    fn style() -> Style;
    fn styled(node: Html) -> Html {
        STYLED.with(|styled| {
            let component_id = hash_of_type::<Self>();
            if styled.borrow().get(&component_id).is_none() {
                let style = Self::style();
                style.write::<Self>();
                styled.borrow_mut().insert(component_id);
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
}

impl Style {
    pub fn new() -> Self {
        Self { style: vec![] }
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

    fn write<C>(&self) {
        Self::add_style_element();

        for (selector, definition_block) in &self.style {
            let mut style_sheet = String::new();
            style_sheet += format!(".{}", styled_class::<C>(selector)).as_str();
            style_sheet += "{";
            for (property, value) in definition_block {
                style_sheet += format!("{}:{};", property, value).as_str();
            }
            style_sheet += "}";

            SHEET.with(move |sheet| {
                if let Some(sheet) = sheet.borrow().as_ref() {
                    let _ = sheet.insert_rule_with_index(
                        style_sheet.as_str(),
                        sheet.css_rules().unwrap().length(),
                    );
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
    { $( $selector:literal { $( $property:literal : $value:expr );*;} )* } => {{
        #[allow(unused_mut)]
        let mut style = Style::new();
        $(
            $(
                style.add(format!("{}", $selector), format!("{}", $property), format!("{}", $value));
            )*
        )*
        style
    }};
}
