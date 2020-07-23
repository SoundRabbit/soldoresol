use super::super::{btn, text};
use crate::{block, idb, model, random_id::U128Id, JsObject, Promise, Timestamp};
use kagura::prelude::*;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

pub type TableDb = Component<Msg, Props, State, Sub>;

pub struct Props {
    pub common_db: Rc<web_sys::IdbDatabase>,
}

pub struct State {
    common_db: Rc<web_sys::IdbDatabase>,
    tables: Vec<(U128Id, Table)>,
    selecting_table: Option<U128Id>,
    cmd_queue: model::CmdQueue<Msg, Sub>,
}

struct Table {
    name: String,
    timestamp: Timestamp,
}

pub enum Msg {
    Close,
    SetTables(Vec<(U128Id, Table)>),
    SelectTable(U128Id),
}

pub enum Sub {
    Close,
    Open(U128Id),
    Clone(U128Id),
}

impl Table {
    fn from_jsvalue(val: &JsValue) -> Option<Self> {
        val.dyn_ref::<JsObject>().and_then(|val| {
            let name = val.get("name").and_then(|x| x.as_string());
            let timestamp = val.get("timestamp").and_then(|x| x.as_f64());
            if let (Some(name), Some(timestamp)) = (name, timestamp) {
                Some(Self {
                    name,
                    timestamp: Timestamp::from(timestamp),
                })
            } else {
                None
            }
        })
    }
}

pub fn new() -> TableDb {
    Component::new(init, update, render)
}

fn init(_: &mut TableDb, state: Option<State>, props: Props) -> (State, Cmd<Msg, Sub>) {
    if let Some(state) = state {
        (
            State {
                common_db: props.common_db,
                tables: state.tables,
                selecting_table: state.selecting_table,
                cmd_queue: state.cmd_queue,
            },
            Cmd::none(),
        )
    } else {
        let state = State {
            common_db: props.common_db,
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
                        Promise::some(promises)
                    } else {
                        Promise::new(|resolve| resolve(None))
                    }
                })
                .then(|tables| {
                    if let Some(tables) = tables {
                        let tables: Vec<_> = tables.into_iter().filter_map(|x| x).collect();
                        resolve(Msg::SetTables(tables));
                    }
                });
        });
        (state, cmd)
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
    }
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
                                    Events::new(),
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
