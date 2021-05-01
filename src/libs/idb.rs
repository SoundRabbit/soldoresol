use js_sys::Promise;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;

pub async fn open_db(name: &str) -> Option<web_sys::IdbDatabase> {
    let request = web_sys::window().unwrap().indexed_db().unwrap().unwrap();
    let request = request.open(name).unwrap();
    crate::debug::log_1(format!("open db: {}", name));
    let request = Rc::new(request);
    JsFuture::from(Promise::new(&mut move |resolve, _| {
        let a = Closure::once(Box::new({
            let request = Rc::clone(&request);
            move || {
                let database = request.result().unwrap();
                let _ = resolve.call1(&js_sys::global(), &database);
            }
        }));
        request.set_onsuccess(Some(a.as_ref().unchecked_ref()));
        a.forget();
    }))
    .await
    .ok()
    .and_then(|x| x.dyn_into::<web_sys::IdbDatabase>().ok())
}

pub async fn create_object_strage(
    database: &web_sys::IdbDatabase,
    name: impl Into<String>,
) -> Option<web_sys::IdbDatabase> {
    let name = Rc::new(name.into());
    let (database_name, version) = (database.name(), database.version());
    database.close();
    let request = web_sys::window().unwrap().indexed_db().unwrap().unwrap();
    let request = request
        .open_with_f64(database_name.as_str(), version + 1.0)
        .unwrap();
    let request = Rc::new(request);
    crate::debug::log_1(format!("upgrade to {}", version + 1.0));
    JsFuture::from(Promise::new(&mut move |resolve, _| {
        let resolve = Rc::new(resolve);
        let a = Closure::wrap(Box::new({
            let request = Rc::clone(&request);
            let resolve = Rc::clone(&resolve);
            let name = Rc::clone(&name);
            move || {
                crate::debug::log_1("onupgradeneeded");
                let database = request
                    .result()
                    .unwrap()
                    .dyn_into::<web_sys::IdbDatabase>()
                    .unwrap();
                let object_store = database.create_object_store(name.as_ref()).unwrap();
                let a = Closure::wrap(Box::new({
                    let resolve = Rc::clone(&resolve);
                    move || {
                        crate::debug::log_1("oncomplete");
                        let _ = resolve.call1(&js_sys::global(), &database);
                    }
                }) as Box<dyn FnMut()>);
                object_store
                    .transaction()
                    .set_oncomplete(Some(a.as_ref().unchecked_ref()));
                a.forget();
            }
        }) as Box<dyn FnMut()>);
        request.set_onupgradeneeded(Some(a.as_ref().unchecked_ref()));
        a.forget();
    }))
    .await
    .ok()
    .and_then(|x| x.dyn_into::<web_sys::IdbDatabase>().ok())
}

pub enum Query<'a> {
    Get(&'a JsValue),
    Add(&'a JsValue, &'a JsValue),
    Put(&'a JsValue, &'a JsValue),
    GetAll,
    GetAllKeys,
}

pub async fn query<'a>(
    database: &web_sys::IdbDatabase,
    object_name: &str,
    query: Query<'a>,
) -> Option<JsValue> {
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
    JsFuture::from(Promise::new(&mut move |resolve, reject| {
        let a = Closure::wrap(Box::new({
            let request = Rc::clone(&request);
            move || {
                let result = match request.result() {
                    Ok(x) => x,
                    Err(..) => JsValue::null(),
                };
                crate::debug::log_1(&result);
                let _ = resolve.call1(&js_sys::global(), &result);
            }
        }) as Box<dyn FnMut()>);
        request.set_onsuccess(Some(a.as_ref().unchecked_ref()));
        a.forget();

        let a = Closure::wrap(Box::new(move || {
            let _ = reject.call1(&js_sys::global(), &JsValue::null());
        }) as Box<dyn FnMut()>);
        request.set_onerror(Some(a.as_ref().unchecked_ref()));
        a.forget();
    }))
    .await
    .ok()
}

pub async fn assign(
    database: &web_sys::IdbDatabase,
    object_name: &str,
    key: &JsValue,
    value: &JsValue,
) -> Option<JsValue> {
    if let Some(keys) = query(&database, &object_name, Query::GetAllKeys).await {
        let keys = js_sys::Array::from(&keys).to_vec();
        if keys.into_iter().position(|x| x.eq(key)).is_some() {
            query(&database, &object_name, Query::Put(&key, &value)).await
        } else {
            query(&database, &object_name, Query::Add(&key, &value)).await
        }
    } else {
        None
    }
}
