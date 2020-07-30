use crate::Promise;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast};

pub fn open_db(name: &str) -> Promise<web_sys::IdbDatabase> {
    let request = web_sys::window().unwrap().indexed_db().unwrap().unwrap();
    let request = request.open(name).unwrap();
    let request = Rc::new(request);
    Promise::new(move |resolve| {
        let a = Closure::once(Box::new({
            let request = Rc::clone(&request);
            move || {
                let database = request
                    .result()
                    .unwrap()
                    .dyn_into::<web_sys::IdbDatabase>()
                    .unwrap();
                resolve(Some(database));
            }
        }));
        request.set_onsuccess(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}

pub fn create_object_strage(
    database: &web_sys::IdbDatabase,
    name: impl Into<String>,
) -> Promise<web_sys::IdbDatabase> {
    let name = name.into();
    let (database_name, version) = (database.name(), database.version());
    database.close();
    let request = web_sys::window().unwrap().indexed_db().unwrap().unwrap();
    let request = request
        .open_with_f64(database_name.as_str(), version + 1.0)
        .unwrap();
    let request = Rc::new(request);
    Promise::new(move |resolve| {
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
                    resolve(Some(database));
                }));
                object_store
                    .transaction()
                    .set_oncomplete(Some(a.as_ref().unchecked_ref()));
                a.forget();
            }
        }));
        request.set_onupgradeneeded(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}

pub enum Query<'a> {
    Get(&'a JsValue),
    Add(&'a JsValue, &'a JsValue),
    Put(&'a JsValue, &'a JsValue),
    GetAll,
    GetAllKeys,
}

pub fn query(database: &web_sys::IdbDatabase, object_name: &str, query: Query) -> Promise<JsValue> {
    let mode = match &query {
        Query::Get(..) => web_sys::IdbTransactionMode::Readonly,
        Query::Add(..) => web_sys::IdbTransactionMode::Readwrite,
        Query::Put(..) => web_sys::IdbTransactionMode::Readwrite,
        Query::GetAll => web_sys::IdbTransactionMode::Readonly,
        Query::GetAllKeys => web_sys::IdbTransactionMode::Readonly,
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
        Query::GetAll => object_store.get_all().unwrap(),
        Query::GetAllKeys => object_store.get_all_keys().unwrap(),
    };
    let request = Rc::new(request);
    Promise::new(move |resolve| {
        let resolve = Rc::new(RefCell::new(Some(resolve)));

        let a = Closure::wrap(Box::new({
            let resolve = Rc::clone(&resolve);
            let request = Rc::clone(&request);
            move || {
                let result = match request.result() {
                    Ok(x) => x,
                    Err(..) => JsValue::null(),
                };
                crate::debug::log_1(&result);
                if let Some(resolve) = resolve.borrow_mut().take() {
                    resolve(Some(result));
                }
            }
        }) as Box<dyn FnMut()>);
        request.set_onsuccess(Some(a.as_ref().unchecked_ref()));
        a.forget();

        let a = Closure::wrap(Box::new({
            let resolve = Rc::clone(&resolve);
            move || {
                if let Some(resolve) = resolve.borrow_mut().take() {
                    resolve(None);
                }
            }
        }) as Box<dyn FnMut()>);
        request.set_onerror(Some(a.as_ref().unchecked_ref()));
        a.forget();
    })
}

pub fn assign(
    database: Rc<web_sys::IdbDatabase>,
    object_name: Rc<String>,
    key: JsValue,
    value: JsValue,
) -> Promise<JsValue> {
    query(&database, &object_name, Query::Add(&key, &value)).and_then(move |x| {
        if let Some(x) = x {
            Promise::new(move |resolve| resolve(Some(x)))
        } else {
            query(&database, &object_name, Query::Put(&key, &value))
        }
    })
}
