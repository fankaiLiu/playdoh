use deno_core::anyhow::Result;
use deno_core::anyhow::anyhow;
use deno_core::{resolve_url_or_path, FsModuleLoader, JsRuntime, RuntimeOptions};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::rc::Rc;
use std::time::Duration;
use std::{collections::HashMap, sync::Mutex};
use tokio::io::AsyncWriteExt;
use tokio::runtime::Builder;
use tokio::task::LocalSet;
use uuid::Uuid;

pub mod ops;

static JS_DATA: Lazy<Mutex<HashMap<String, Value>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn run(code: &str, args: &str) -> String {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();
    let (id, file_name) = save_code(args, code).await;
    {
        let mut result = JS_DATA.lock().unwrap();
        let json = json!({
   "code": "200"});
        result.insert(id.to_string(), json);
    }
    let handler = std::thread::spawn(move || {
        let local = LocalSet::new();
        local.spawn_local(async move {
            let options = RuntimeOptions {
                extensions: vec![ops::init()],
                module_loader: Some(Rc::new(FsModuleLoader)),
                ..Default::default()
            };
            let mut rt = JsRuntime::new(options);
            execute_main_module(&mut rt, &file_name).await.unwrap();
        });
        rt.block_on(local);
    });
    //tokio::fs::remove_file(&file_name).await.unwrap();
    //等待线程完成
    handler.join().unwrap();
    let result = JS_DATA
        .lock()
        .unwrap()
        .get(&id.to_string()) 
        .unwrap()
        .clone();
    JS_DATA.lock().unwrap().remove(&id.to_string());
    result.to_string()
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

async fn execute_main_module(rt: &mut JsRuntime, path: impl AsRef<str>) -> Result<()> {
    let url = resolve_url_or_path(path.as_ref())?;
    let id = rt.load_main_module(&url, None).await?;
    let mut resolver = rt.mod_evaluate(id);
    let timer = tokio::time::sleep(Duration::from_millis(5000));
    let fut = async move {
        loop {
            tokio::select! {
            resolved=&mut resolver=>{
                return resolved.expect("faild to eval");
            }
            _=rt.run_event_loop(false)=>{
               return  resolver.await.expect("faild to eval");
            }
            }
        }
    };
    tokio::select! {
        ret = fut => ret,
        _ = timer => {
            Err(anyhow!("time out"))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
 
    #[tokio::test]
    async fn test_main_router() {
        let res=run("","{}").await;
        dbg!(res);
    }
}