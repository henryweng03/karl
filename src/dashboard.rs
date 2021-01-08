//! Controller dashboard.
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

use rocket;
use rocket::State;
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use tokio::runtime::Runtime;
use serde::Serialize;

use handlebars::{Helper, Handlebars, Context, RenderContext, Output, HelperResult};
use crate::controller::{Request, Host, Client};

#[derive(Serialize)]
struct MainContext {
    title: &'static str,
    hosts: Vec<Host>,
    clients: Vec<Client>,
}

#[derive(Serialize)]
struct AppContext {
    client_id: String,
    client_ip: String,
    files: Vec<PathBuf>,
}

#[get("/")]
fn index(
    hosts: State<Arc<Mutex<HashMap<String, Host>>>>,
    clients: State<Arc<Mutex<HashMap<String, Client>>>>,
) -> Template {
    Template::render("index", &MainContext {
        title: "Hello",
        hosts: hosts.lock().unwrap().values().map(|host| host.clone()).collect(),
        clients: clients.lock().unwrap().values().map(|client| client.clone()).collect(),
    })
}

#[get("/app/<client_id>")]
fn app(
    client_id: String,
    karl_path: State<PathBuf>,
    clients: State<Arc<Mutex<HashMap<String, Client>>>>,
) -> Option<Template> {
    let client_ip = if let Some(client) = clients.lock().unwrap().get(&client_id) {
        format!("{}", client.addr)
    } else {
        return None;
    };
    let files = {
        let storage_path = karl_path.join("storage").join(&client_id);
        std::fs::read_dir(&storage_path).unwrap()
            .map(|res| res.unwrap().path())
            .map(|path| path.strip_prefix(&storage_path).unwrap().to_path_buf())
            .collect()
    };
    debug!("files for client_id={}: {:?}", &client_id, files);
    Some(Template::render(
        client_id.clone(),
        &AppContext { client_id, client_ip, files },
    ))
}

#[get("/app/<client_id>/storage/<file..>")]
pub fn files(
    karl_path: State<PathBuf>,
    client_id: String,
    file: PathBuf,
) -> Option<NamedFile> {
    let path = karl_path
        .join("storage")
        .join(client_id)
        .join(file);
    NamedFile::open(path).ok()
}

fn request_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(description) = param.value().get("description") {
            if let Some(description) = description.as_str() {
                out.write(description)?;
            }
        }
        if let Some(start) = param.value().get("start") {
            if let Some(start) = start.as_u64() {
                let mut end = Request::time_since_epoch_s();
                if let Some(end_param) = param.value().get("end") {
                    if let Some(end_param) = end_param.as_u64() {
                        end = end_param;
                    }
                };
                let elapsed = end - start;
                out.write(" (")?;
                out.write(&elapsed.to_string())?;
                out.write("s)")?;
            }
        }
    }

    Ok(())
}

pub fn start(
    rt: &mut Runtime,
    karl_path: PathBuf,
    hosts: Arc<Mutex<HashMap<String, Host>>>,
    clients: Arc<Mutex<HashMap<String, Client>>>,
) {
    rt.spawn(async move {
        rocket::ignite()
        .manage(karl_path)
        .manage(hosts)
        .manage(clients)
        .mount("/", routes![index, app, files])
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("request", Box::new(request_helper));
        }))
        .launch();
    });
}
