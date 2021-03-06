use super::super::{btn, text};
use crate::{
    block::{self, BlockId},
    idb, model,
    random_id::U128Id,
    resource::Data,
    JsObject, Promise, Timestamp,
};
use kagura::prelude::*;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

pub type LoadTable = Component<Props, Sub>;

pub struct Props {
    pub common_db: Rc<web_sys::IdbDatabase>,
    pub table_db: Rc<web_sys::IdbDatabase>,
    pub block_field: block::Field,
}

pub struct State {
    block_field: block::Field,
    common_db: Rc<web_sys::IdbDatabase>,
    table_db: Rc<web_sys::IdbDatabase>,
    tables: Vec<(U128Id, Table)>,
    selecting_table: Option<U128Id>,
    cmd_queue: model::CmdQueue<Msg, Sub>,
}

pub struct Table {
    name: String,
    timestamp: Timestamp,
    resources: Vec<U128Id>,
}

pub enum LoadMode {
    Open,
    Clone,
}

pub enum Msg {
    Close,
    SetTables(Vec<(U128Id, Table)>),
    SelectTable(U128Id),
    LoadSelectingTable(LoadMode),
    LoadTable(
        LoadMode,
        U128Id,
        HashMap<U128Id, JsValue>,
        HashMap<U128Id, Data>,
    ),
    OpenTable(
        U128Id,
        HashMap<BlockId, block::FieldBlock>,
        HashMap<U128Id, Data>,
    ),
}

pub enum Sub {
    Close,
    Open(
        BlockId,
        HashMap<BlockId, block::FieldBlock>,
        HashMap<U128Id, Data>,
    ),
    Clone(U128Id),
}

impl Table {
    fn from_jsvalue(val: &JsValue) -> Option<Self> {
        val.dyn_ref::<JsObject>().and_then(|val| {
            let name = val.get("name").and_then(|x| x.as_string());
            let timestamp = val.get("timestamp").and_then(|x| x.as_f64());
            let resources = val.get("resources").map(|x| {
                js_sys::Array::from(&x)
                    .to_vec()
                    .into_iter()
                    .filter_map(|x| U128Id::from_jsvalue(&x))
                    .collect::<Vec<_>>()
            });
            if let (Some(name), Some(timestamp), Some(resources)) = (name, timestamp, resources) {
                Some(Self {
                    name,
                    timestamp: Timestamp::from(timestamp),
                    resources: resources,
                })
            } else {
                None
            }
        })
    }
}

pub fn new() -> LoadTable {
    Component::new(init, update, render)
}

fn init(state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>, Vec<Batch<Msg>>) {
    if let Some(state) = state {
        (
            State {
                block_field: props.block_field,
                common_db: props.common_db,
                table_db: props.table_db,
                tables: state.tables,
                selecting_table: state.selecting_table,
                cmd_queue: state.cmd_queue,
            },
            Cmd::none(),
            vec![],
        )
    } else {
        let state = State {
            block_field: props.block_field,
            common_db: props.common_db,
            table_db: props.table_db,
            tables: vec![],
            selecting_table: None,
            cmd_queue: model::CmdQueue::new(),
        };

        let common_db = Rc::clone(&state.common_db);
        let promise = idb::query(&common_db, "tables", idb::Query::GetAllKeys);
        let cmd = Cmd::task(move |resolve| {
            promise
                .and_then(move |keys| {
                    if let Some(keys) = keys {
                        let keys = js_sys::Array::from(&keys).to_vec();
                        let mut promises = vec![];
                        for key in keys {
                            if let Some(table_id) = U128Id::from_jsvalue(&key) {
                                promises.push(
                                    idb::query(&common_db, "tables", idb::Query::Get(&key)).map(
                                        |x| {
                                            x.and_then(|x| Table::from_jsvalue(&x))
                                                .map(|x| (table_id, x))
                                        },
                                    ),
                                )
                            }
                        }
                        Promise::all(promises)
                    } else {
                        Promise::new(|resolve| resolve(None))
                    }
                })
                .then(|tables| {
                    if let Some(tables) = tables {
                        let tables: Vec<_> = tables.into_iter().collect();
                        resolve(Msg::SetTables(tables));
                    }
                });
        });
        (state, cmd, vec![])
    }
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    match msg {
        Msg::Close => {
            state.cmd_queue.enqueue(Cmd::sub(Sub::Close));
            state.cmd_queue.dequeue()
        }
        Msg::SetTables(tables) => {
            state.tables = tables;
            state.cmd_queue.dequeue()
        }
        Msg::SelectTable(table_id) => {
            state.selecting_table = Some(table_id);
            state.cmd_queue.dequeue()
        }
        Msg::LoadSelectingTable(load_mode) => {
            if let Some(table_id) = state.selecting_table.as_ref() {
                if let Some(i) = state.tables.iter().position(|(t_id, _)| t_id == table_id) {
                    let promise = load_table(
                        Rc::clone(&state.common_db),
                        Rc::clone(&state.table_db),
                        table_id.clone(),
                        &state.tables[i].1.resources,
                    );

                    let cmd = Cmd::task({
                        let table_id = table_id.clone();
                        move |resolve| {
                            promise.then(|x| {
                                if let Some((blocks, resources)) = x {
                                    resolve(Msg::LoadTable(load_mode, table_id, blocks, resources));
                                }
                            });
                        }
                    });
                    state.cmd_queue.enqueue(cmd);
                }
            }
            state.cmd_queue.dequeue()
        }
        Msg::LoadTable(load_mode, table_id, blocks, resources) => {
            let blocks = state.block_field.unpack_listed(blocks.into_iter());

            let cmd = Cmd::task(move |resolve| {
                blocks.then(move |blocks| {
                    if let Some(blocks) = blocks {
                        match load_mode {
                            LoadMode::Open => resolve(Msg::OpenTable(table_id, blocks, resources)),
                            LoadMode::Clone => {}
                        }
                    }
                })
            });

            state.cmd_queue.enqueue(cmd);
            state.cmd_queue.dequeue()
        }
        Msg::OpenTable(table_id, blocks, resources) => {
            state.cmd_queue.enqueue(Cmd::sub(Sub::Open(
                state.block_field.block_id(table_id),
                blocks,
                resources,
            )));
            state.cmd_queue.dequeue()
        }
    }
}

fn load_table(
    common_db: Rc<web_sys::IdbDatabase>,
    table_db: Rc<web_sys::IdbDatabase>,
    table_id: U128Id,
    resources: &Vec<U128Id>,
) -> Promise<(HashMap<U128Id, JsValue>, HashMap<U128Id, Data>)> {
    let mut promises = Promise::new(|resolve| resolve(Some(vec![])));
    for r_id in resources {
        crate::debug::log_1(r_id.to_jsvalue());
        promises = promises.and_then({
            let r_id = r_id.clone();
            let common_db = Rc::clone(&common_db);
            move |resources| match resources {
                Some(mut resources) => {
                    idb::query(&common_db, "resources", idb::Query::Get(&r_id.to_jsvalue()))
                        .and_then(|x| match x {
                            Some(x) => Data::unpack(x),
                            None => Promise::new(|resolve| resolve(None)),
                        })
                        .map({
                            let r_id = r_id.clone();
                            move |x| {
                                x.map(|x| {
                                    resources.push((r_id, x));
                                    resources
                                })
                            }
                        })
                }
                None => Promise::new(|resolve| resolve(None)),
            }
        });
    }
    let resources = promises;

    idb::query(&table_db, &table_id.to_string(), idb::Query::GetAllKeys)
        .and_then(move |keys| {
            if let Some(keys) = keys {
                let keys = js_sys::Array::from(&keys).to_vec();
                let mut promises = vec![];
                for key in keys {
                    if let Some(block_id) = U128Id::from_jsvalue(&key) {
                        promises.push(
                            idb::query(&table_db, &table_id.to_string(), idb::Query::Get(&key))
                                .map(|x| x.map(|x| (block_id, x))),
                        )
                    } else if key.as_string().map(|key| key == "data").unwrap_or(false) {
                        promises.push(
                            idb::query(
                                &table_db,
                                &table_id.to_string(),
                                idb::Query::Get(&JsValue::from("data")),
                            )
                            .map({
                                let table_id = table_id.clone();
                                move |x| x.map(|x| (table_id, x))
                            }),
                        )
                    }
                }
                Promise::all(promises)
            } else {
                Promise::new(|resolve| resolve(None))
            }
        })
        .and_then(move |tables| {
            if let Some(tables) = tables {
                resources.map(move |resources| resources.map(|x| (tables, x)))
            } else {
                Promise::new(|resolve| resolve(None))
            }
        })
        .map(|x| {
            x.map(|(tables, resources)| {
                (
                    tables.into_iter().collect::<HashMap<_, _>>(),
                    resources.into_iter().collect::<HashMap<_, _>>(),
                )
            })
        })
}

fn render(state: &State, _: Vec<Html>) -> Html {
    super::container(
        Attributes::new(),
        Events::new(),
        vec![super::frame(
            12,
            Attributes::new(),
            Events::new(),
            vec![
                super::header(
                    Attributes::new().class("keyvalue").class("keyvalue-rev"),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new().class("text-label"),
                            Events::new(),
                            vec![Html::text("保存済みのテーブル")],
                        ),
                        Html::div(
                            Attributes::new().class("linear-h"),
                            Events::new(),
                            vec![btn::close(
                                Attributes::new(),
                                Events::new().on_click(move |_| Msg::Close),
                            )],
                        ),
                    ],
                ),
                super::body(
                    Attributes::new()
                        .class("keyvalue")
                        .class("keyvalue-rev")
                        .class("keyvalue-align-stretch"),
                    Events::new(),
                    vec![
                        Html::div(
                            Attributes::new()
                                .class("linear-v")
                                .class("container-a")
                                .class("scroll-y"),
                            Events::new(),
                            state
                                .tables
                                .iter()
                                .map(|(table_id, table)| {
                                    btn::selectable(
                                        state
                                            .selecting_table
                                            .as_ref()
                                            .map(|t_id| *table_id == *t_id)
                                            .unwrap_or(false),
                                        Attributes::new().class("pure-button-list"),
                                        Events::new().on_click({
                                            let table_id = table_id.clone();
                                            move |_| Msg::SelectTable(table_id)
                                        }),
                                        vec![
                                            text::div(&table.name),
                                            text::div(format!(
                                                "最終更新日時：{}",
                                                table.timestamp.to_string()
                                            )),
                                        ],
                                    )
                                })
                                .collect(),
                        ),
                        Html::div(
                            Attributes::new()
                                .class("vkeyvalue")
                                .class("vkeyvalue-rev")
                                .class("container-a"),
                            Events::new(),
                            vec![
                                Html::div(Attributes::new(), Events::new(), vec![]),
                                btn::primary(
                                    Attributes::new(),
                                    Events::new()
                                        .on_click(|_| Msg::LoadSelectingTable(LoadMode::Open)),
                                    vec![Html::text("読み込み")],
                                ),
                            ],
                        ),
                    ],
                ),
                super::footer(Attributes::new(), Events::new(), vec![]),
            ],
        )],
    )
}
