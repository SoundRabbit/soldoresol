use super::{
    common, modeless,
    state::{self, chat, table, Modeless},
};
use super::{Msg, State};
use crate::{
    block::{self, chat::item::Icon, BlockId},
    model::{self, PersonalData},
    Resource,
};
use kagura::prelude::*;
use wasm_bindgen::JsCast;

pub fn render(block_field: &block::Field, modeless: &model::Modeless<Modeless>) -> Html<Msg> {
    unimplemented!();
}
