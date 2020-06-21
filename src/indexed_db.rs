use kagura::prelude::*;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

pub fn open_db<Msg: 'static, Sub>(
    name: &str,
    on_success: impl FnOnce(web_sys::IdbDatabase) -> Msg + 'static,
) -> Cmd<Msg, Sub> {
    Cmd::task({
        let request = web_sys::window().unwrap().indexed_db().unwrap().unwrap();
        let request = request.open(name).unwrap();
        let request = Rc::new(request);
        move |resolve| {
            let a = Closure::once(Box::new({
                let request = Rc::clone(&request);
                move || {
                    let database = request
                        .result()
                        .unwrap()
                        .dyn_into::<web_sys::IdbDatabase>()
                        .unwrap();
                    resolve(on_success(database));
                }
            }));
            request.set_onsuccess(Some(a.as_ref().unchecked_ref()));
            a.forget();
        }
    })
}

pub fn create_object_strage<Msg: 'static, Sub>(
    database: &web_sys::IdbDatabase,
    name: impl Into<String>,
    on_complete: impl FnOnce(web_sys::IdbDatabase) -> Msg + 'static,
) -> Cmd<Msg, Sub> {
    Cmd::task({
        let name = name.into();
        let (database_name, version) = (database.name(), database.version());
        database.close();
        let request = web_sys::window().unwrap().indexed_db().unwrap().unwrap();
        let request = request
            .open_with_f64(database_name.as_str(), version + 1.0)
            .unwrap();
        let request = Rc::new(request);
        move |resolve| {
            let a = Closure::once(Box::new({
                let request = Rc::clone(&request);
                move || {
                    let database = request
                        .result()
                        .unwrap()
                        .dyn_into::<web_sys::IdbDatabase>()
                        .unwrap();
                    let object_store = database.create_object_store(name.as_ref()).unwrap();
                    let a = Closure::once(Box::new(move || {
                        resolve(on_complete(database));
                    }));
                    object_store
                        .transaction()
                        .set_oncomplete(Some(a.as_ref().unchecked_ref()));
                    a.forget();
                }
            }));
            request.set_onupgradeneeded(Some(a.as_ref().unchecked_ref()));
            a.forget();
        }
    })
}

pub enum Query<'a> {
    Get(&'a JsValue),
    Add(&'a JsValue, &'a JsValue),
    Put(&'a JsValue, &'a JsValue),
}

pub fn query<Msg: 'static, Sub>(
    database: &web_sys::IdbDatabase,
    object_name: &str,
    query: Query,
    on_success: impl FnOnce(JsValue) -> Msg + 'static,
    on_error: impl FnOnce(JsValue) -> Msg + 'static,
) -> Cmd<Msg, Sub> {
    Cmd::task({
        let mode = match &query {
            Query::Get(..) => web_sys::IdbTransactionMode::Readonly,
            Query::Add(..) => web_sys::IdbTransactionMode::Readwrite,
            Query::Put(..) => web_sys::IdbTransactionMode::Readwrite,
        };
        let object_store = database
            .transaction_with_str_and_mode(object_name, mode)
            .unwrap()
            .object_store(object_name)
            .unwrap();
        let request = match query {
            Query::Get(key) => object_store.get(key).unwrap(),
            Query::Add(key, val) => object_store.add_with_key(val, key).unwrap(),
            Query::Put(key, val) => object_store.put_with_key(val, key).unwrap(),
        };
        let request = Rc::new(request);
        move |resolve| {
            let resolve = Rc::new(RefCell::new(Some(resolve)));

            let a = Closure::once(Box::new({
                let resolve = Rc::clone(&resolve);
                let request = Rc::clone(&request);
                move || {
                    let result = match request.result() {
                        Ok(x) => x,
                        Err(..) => JsValue::null(),
                    };
                    if let Some(resolve) = resolve.borrow_mut().take() {
                        resolve(on_success(result));
                    }
                }
            }));
            request.set_onsuccess(Some(a.as_ref().unchecked_ref()));
            a.forget();

            let a = Closure::once(Box::new({
                let resolve = Rc::clone(&resolve);
                let request = Rc::clone(&request);
                move || {
                    let result = match request.result() {
                        Ok(..) => JsValue::null(),
                        Err(x) => x,
                    };
                    if let Some(resolve) = resolve.borrow_mut().take() {
                        resolve(on_error(result));
                    }
                }
            }));
            request.set_onerror(Some(a.as_ref().unchecked_ref()));
            a.forget();
        }
    })
}
