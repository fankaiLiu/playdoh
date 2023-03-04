
use deno_core::{FsModuleLoader};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
//use tokio::task::LocalSet;
use std::rc::Rc;
use std::{collections::HashMap, sync::Mutex};
use tokio::io::AsyncWriteExt;
use tokio::runtime::Builder;
use uuid::Uuid;
use deno_core::error::AnyError;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::ops::io::Stdio;
use deno_runtime::ops::io::StdioPipe;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;

use std::path::Path;
use std::sync::Arc;

fn get_error_class_name(e: &AnyError) -> &'static str {
deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}


//pub mod ops;

static JS_DATA: Lazy<Mutex<HashMap<String, Value>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn run(code: &str, args: &str) -> Result<String, AnyError> {
  //   let rt = Builder::new_current_thread().enable_all().build().unwrap();
  //   let (id, file_name) = save_code(args, code).await;
  //   {
  //       let mut result = JS_DATA.lock().unwrap();
  //       let json = json!({
  //  "code": "200"});
  //       result.insert(id.to_string(), json);
  //   }

  //   let handler = std::thread::spawn(move || {
  //       let local = LocalSet::new();
  //       local.spawn_local(async move {
  //         let module_loader = Rc::new(FsModuleLoader);
  //         let create_web_worker_cb = Arc::new(|_| {
  //           todo!("Web workers are not supported in the example");
  //         });
  //         let web_worker_event_cb = Arc::new(|_| {
  //           todo!("Web workers are not supported in the example");
  //         });
  //         //let file=File::open("log.txt").unwrap();
  //         let stdio=Stdio {
  //           stdin: StdioPipe::Inherit,
  //           stdout: StdioPipe::Inherit,
  //           stderr: StdioPipe::Inherit,
  //           };
  //         let options = WorkerOptions {
  //           bootstrap: BootstrapOptions {
  //             args: vec![],
  //             cpu_count: 1,
  //             debug_flag: false,
  //             enable_testing_features: false,
  //             locale: deno_core::v8::icu::get_language_tag(),
  //             location: None,
  //             no_color: false,
  //             is_tty: false,
  //             runtime_version: "x".to_string(),
  //             ts_version: "x".to_string(),
  //             unstable: false,
  //             user_agent: "hello_runtime".to_string(),
  //             inspect: false,
  //           },
  //           extensions: vec![],
  //           extensions_with_js: vec![],
  //           startup_snapshot: None,
  //           unsafely_ignore_certificate_errors: None,
  //           root_cert_store: None,
  //           seed: None,
  //           source_map_getter: None,
  //           format_js_error_fn: None,
  //           web_worker_preload_module_cb: web_worker_event_cb.clone(),
  //           web_worker_pre_execute_module_cb: web_worker_event_cb,
  //           create_web_worker_cb,
  //           maybe_inspector_server: None,
  //           should_break_on_first_statement: false,
  //           should_wait_for_inspector_session: false,
  //           module_loader,
  //           npm_resolver: None,
  //           get_error_class_fn: Some(&get_error_class_name),
  //           cache_storage_dir: None,
  //           origin_storage_dir: None,
  //           blob_store: BlobStore::default(),
  //           broadcast_channel: InMemoryBroadcastChannel::default(),
  //           shared_array_buffer_store: None,
  //           compiled_wasm_module_store: None,
  //           stdio,
  //         };
  //          let main_module = deno_core::resolve_path(&file_name).unwrap();
  //         let permissions = PermissionsContainer::allow_all();
        
  //         let mut worker = MainWorker::bootstrap_from_options(
  //           main_module.clone(),
  //           permissions,
  //           options,
  //         );
  //         worker.execute_main_module(&main_module).await.unwrap();
  //         worker.run_event_loop(false).await.unwrap();
          
  //       });
  //       rt.block_on(local);
  //   });
  //   //tokio::fs::remove_file(&file_name).await.unwrap();
  //   //等待线程完成
  //   handler.join().unwrap();
  //   let result = JS_DATA
  //   .lock()
  //   .unwrap()
  //   .get(&id.to_string()) 
  //   .unwrap()
  //   .clone();
   
  //   JS_DATA.lock().unwrap().remove(&id.to_string());
  //   dbg!(&result);
  //   Ok(result.to_string())
  todo!()
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
        //let res=run("","{}").await;
    }
}