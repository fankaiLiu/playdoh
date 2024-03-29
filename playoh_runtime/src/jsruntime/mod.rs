use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::ops::io::Stdio;
use deno_runtime::ops::io::StdioPipe;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::fs::File;
use std::rc::Rc;
use std::{collections::HashMap, sync::Mutex};
use tokio::io::AsyncWriteExt;
use tokio::runtime::Builder;
use tokio::task::LocalSet;
use uuid::Uuid;

use std::path::Path;
use std::sync::Arc;

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

//pub mod ops;

static JS_DATA: Lazy<Mutex<HashMap<String, Value>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub struct ExecutionResult {
    pub result: String,
    pub console_log: Option<String>,
    pub console_error: Option<String>,
}
impl ExecutionResult {
    pub fn new(result: String, console_log: Option<String>, console_error: Option<String>) -> Self {
        Self {
            result,
            console_log,
            console_error,
        }
    }
}

pub async fn run(code: &str, args: &str) -> Result<ExecutionResult, AnyError> {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let (id, file_name) = save_code(args, code).await;
    {
        let mut result = JS_DATA.lock().unwrap();
        let json = json!({"code": "200"});
        result.insert(id.to_string(), json);
    }
    let inner_file_name = file_name.clone();
    let inner_id = id.clone();
    let handler = std::thread::spawn(move || {
        // IF console_log is not created，create it
        if !Path::new("console_log").exists() {
            std::fs::create_dir("console_log").unwrap();
        }
        if !Path::new("console_error").exists() {
            std::fs::create_dir("console_error").unwrap();
        }

        let file = inner_id.to_string() + ".txt";
        let path = Path::new("console_log").join(&file);
        let log_file = File::create(path).unwrap();
        let error_file = File::create(Path::new("console_error").join(&file)).unwrap();

        let local = LocalSet::new();
        local.spawn_local(async move {
            let module_loader = Rc::new(FsModuleLoader);
            let create_web_worker_cb = Arc::new(|_| {
                todo!("Web workers are not supported in the example");
            });
            let web_worker_event_cb = Arc::new(|_| {
                todo!("Web workers are not supported in the example");
            });
            //let file=File::open("log.txt").unwrap();
            let stdio = Stdio {
                stdin: StdioPipe::Inherit,
                // stdout: StdioPipe::Inherit,
                // stderr: StdioPipe::Inherit,
                stdout: StdioPipe::File(log_file),
                stderr: StdioPipe::File(error_file),
            };
            let options = WorkerOptions {
                bootstrap: BootstrapOptions {
                    args: vec![],
                    cpu_count: 1,
                    debug_flag: false,
                    enable_testing_features: false,
                    locale: deno_core::v8::icu::get_language_tag(),
                    location: None,
                    no_color: false,
                    is_tty: false,
                    runtime_version: "x".to_string(),
                    ts_version: "x".to_string(),
                    unstable: false,
                    user_agent: "hello_runtime".to_string(),
                    inspect: false,
                },
                extensions: vec![],
                extensions_with_js: vec![],
                startup_snapshot: None,
                unsafely_ignore_certificate_errors: None,
                root_cert_store: None,
                seed: None,
                source_map_getter: None,
                format_js_error_fn: None,
                web_worker_preload_module_cb: web_worker_event_cb.clone(),
                web_worker_pre_execute_module_cb: web_worker_event_cb,
                create_web_worker_cb,
                maybe_inspector_server: None,
                should_break_on_first_statement: false,
                should_wait_for_inspector_session: false,
                module_loader,
                npm_resolver: None,
                get_error_class_fn: Some(&get_error_class_name),
                cache_storage_dir: None,
                origin_storage_dir: None,
                blob_store: BlobStore::default(),
                broadcast_channel: InMemoryBroadcastChannel::default(),
                shared_array_buffer_store: None,
                compiled_wasm_module_store: None,
                stdio,
            };
            let main_module = deno_core::resolve_path(&inner_file_name.clone()).unwrap();
            let permissions = PermissionsContainer::allow_all();

            let mut worker =
                MainWorker::bootstrap_from_options(main_module.clone(), permissions, options);
            worker.execute_main_module(&main_module).await.unwrap();
            worker.run_event_loop(false).await.unwrap();
        });
        rt.block_on(local);
    });
    handler.join().unwrap();
    let result = JS_DATA
        .lock()
        .unwrap()
        .get(&id.to_string())
        .unwrap()
        .clone();
    JS_DATA.lock().unwrap().remove(&id.to_string());
    dbg!(&result);
    let log_path = Path::new("console_log").join(Path::new(&(id.to_string() + ".txt")));
    let error_path = Path::new("console_error").join(Path::new(&(id.to_string() + ".txt")));
    let log_msg = std::fs::read_to_string(&log_path)?;
    let error_msg = std::fs::read_to_string(&error_path)?;
    std::fs::remove_file(&log_path).unwrap();
    std::fs::remove_file(&error_path).unwrap();
    let result = ExecutionResult::new(result.to_string(), Some(log_msg), Some(error_msg));
    Ok(result)
}

async fn save_code(args: &str, code: &str) -> (Uuid, String) {
    let id = Uuid::new_v4();
    let file_name = format!("{}.js", id);
    if !std::path::Path::new("js").exists() {
        std::fs::create_dir("js").unwrap();
    }
    let path = std::path::Path::new("js");
    let path = path.join(file_name);
    let file_name = path.clone().into_os_string().into_string().unwrap();
    let mut file = tokio::fs::File::create(path).await.unwrap();
    let init_code = format!(
        r#"const inner_id="{}";
        const args = JSON.parse('{}');
    {}
    "#,
        &id,
        args,
        " function add_msg(key,msg){
        result(inner_id,key,msg);
    }"
    );
    file.write_all(init_code.as_bytes()).await.unwrap();
    file.write_all(code.as_bytes()).await.unwrap();
    file.flush().await.unwrap();
    (id, file_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_main_router() {
        let res = run("", "{}").await;
    }
}
