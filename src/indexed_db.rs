use kagura::prelude::*;
use std::rc::Rc;
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
