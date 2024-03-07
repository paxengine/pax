use actix::Addr;
use actix_web::middleware::Logger;

use actix_web::web::Data;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use actix_web_actors::ws;
use async_openai::config::OpenAIConfig;
use colored::Colorize;

use notify::{Error, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use pax_compiler::helpers::PAX_BADGE;
use pax_compiler::RunContext;
use pax_designtime::messages::LLMHelpResponse;
use pax_designtime::orm::template::NodeAction;
use pax_manifest::PaxManifest;
use tera::Value;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use websocket::PrivilegedAgentWebSocket;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use std::env;
use async_openai::{
    types::{ResponseFormat},
    Client,
};

pub mod code_serialization;
pub mod websocket;
mod llm;
const PORT: u16 = 8252;

pub struct AppState {
    active_websocket_client: Mutex<Option<Addr<PrivilegedAgentWebSocket>>>,
    request_id_counter: Mutex<usize>,
    manifest: Option<PaxManifest>,
    last_written_timestamp: Mutex<SystemTime>,
    open_ai_client: Client<OpenAIConfig>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            active_websocket_client: Mutex::new(None),
            request_id_counter: Mutex::new(0),
            manifest: None,
            last_written_timestamp: Mutex::new(UNIX_EPOCH),
            open_ai_client: Client::new(),
        }
    }

    pub fn new_with_manifest(manifest: PaxManifest) -> Self {
        AppState {
            active_websocket_client: Mutex::new(None),
            request_id_counter: Mutex::new(0),
            manifest: Some(manifest),
            last_written_timestamp: Mutex::new(UNIX_EPOCH),
            open_ai_client: Client::new(),
        }
    }

    fn generate_request_id(&self) -> usize {
        let mut counter = self.request_id_counter.lock().unwrap();
        *counter += 1;
        *counter
    }

    pub fn update_last_written_timestamp(&self) {
        let mut last_written = self.last_written_timestamp.lock().unwrap();
        *last_written = SystemTime::now();
    }
}

#[get("/ws")]
pub async fn web_socket(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> impl Responder {
    ws::start(PrivilegedAgentWebSocket::new(state), &req, stream)
}

#[allow(unused_assignments)]
pub async fn start_server(folder_to_watch: &str) -> std::io::Result<()> {
    //env_logger::init_from_env(Env::default().default_filter_or("info"));


    std::env::set_var("PAX_WORKSPACE_ROOT", "../pax");
    let ctx = RunContext {
        target: pax_compiler::RunTarget::Web,
        path: "../pax-designer".to_string(),
        verbose: false,
        should_also_run: false,
        is_libdev_mode: true,
        process_child_ids: Arc::new(Mutex::new(vec![])),
        is_release: false,
    };

    let (manifest, fs_path) = pax_compiler::perform_build(&ctx).unwrap();

    let state = Data::new(AppState::new_with_manifest(manifest));
    let _watcher =
        setup_file_watcher(state.clone(), folder_to_watch).expect("Failed to setup file watcher");

    let _watcher =
        setup_file_watcher(state.clone(), "../pax-designer").expect("Failed to setup file watcher");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(state.clone())
            .service(web_socket)
            .service(
                actix_files::Files::new("/*", fs_path.clone().unwrap()).index_file("index.html"),
            )
    })
    .bind(("127.0.0.1", PORT))?;

    let address_msg = format!("http://127.0.0.1:{}", PORT).blue();
    let server_running_at_msg = format!("Server running at {}", address_msg).bold();
    println!("{} 📠 {}", *PAX_BADGE, server_running_at_msg);

    server.run().await
}

#[derive(Default)]
pub enum FileContent {
    Pax(String),
    Rust(String),
    #[default]
    Unknown,
}
#[derive(Default)]
struct WatcherFileChanged {
    pub contents: FileContent,
    pub path: String,
}

impl actix::Message for WatcherFileChanged {
    type Result = ();
}

struct LLMHelpResponseMessage {
    pub request_id: usize,
    pub actions: Vec<NodeAction>,
}

impl actix::Message for LLMHelpResponseMessage {
    type Result = ();
}

impl From<LLMHelpResponseMessage> for LLMHelpResponse {
    fn from(value: LLMHelpResponseMessage) -> Self {
        LLMHelpResponse {
            request_id: value.request_id,
            response: value.actions,
        }
    }
}


pub fn setup_file_watcher(state: Data<AppState>, path: &str) -> Result<RecommendedWatcher, Error> {
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, Error>| match res {
            Ok(e) => {
                if let Some(addr) = &*state.active_websocket_client.lock().unwrap() {
                    let now = SystemTime::now();
                    // check last written time so we don't spam file changes when we serialize
                    let last_written = *state.last_written_timestamp.lock().unwrap();
                    if now
                        .duration_since(last_written)
                        .unwrap_or_default()
                        .as_millis()
                        > 100
                    {
                        match e.kind {
                            EventKind::Modify(_) => {
                                if let Some(path) = e.paths.first() {
                                    match fs::read_to_string(path) {
                                        Ok(contents) => {
                                            let extension = path.extension();
                                            let msg = WatcherFileChanged {
                                                contents: match extension.and_then(|e| e.to_str()) {
                                                    Some("pax") => FileContent::Pax(contents),
                                                    Some("rs") => FileContent::Rust(contents),
                                                    _ => FileContent::Unknown,
                                                },
                                                path: path.to_str().unwrap().to_string(),
                                            };
                                            addr.do_send(msg);
                                        }
                                        Err(e) => println!("Error reading file: {:?}", e),
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Err(e) => {
                println!("File system watch error: {:?}", e);
            }
        },
        Default::default(),
    )?;
    watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
    Ok(watcher)
}



// async fn query_openai_api(prompt: &str) -> Result<Vec<SimpleNodeAction>, String> {
//     let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
//     let system_prompt = fs::read_to_string("../pax-designtime/src/orm/llm/system_prompt.txt").expect("Error reading system prompt");
//     let client = reqwest::Client::new();
//     let mut headers = HeaderMap::new();
//     headers.insert(AUTHORIZATION, format!("Bearer {}", api_key).parse().unwrap());
//     headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

//     let response = client.post("https://api.openai.com/v1/chat/completions")
//         .headers(headers)
//         .json(&serde_json::json!({
//             "model": "gpt-4-turbo-preview",
//             "messages": [
//                 {
//                     "role": "system",
//                     "content": system_prompt
//                 },
//                 {
//                     "role": "user",
//                     "content": prompt
//                 }
//             ]
//         }))
//         .send()
//         .await.map_err(|e| format!("Error querying OpenAI API: {:?}", e))?;

//     let text = response.text().await.unwrap();
//     let value : Value = serde_json::from_str(&text).unwrap();
    
//     if let Some(first_choice) = value["choices"].as_array().and_then(|arr| arr.get(0)) {
//         let content = &first_choice["message"]["content"];

//         // Assuming the content is a string that represents JSON, parse it further if needed
//         if let Some(content_str) = content.as_str() {
//             // Further processing here, e.g., trim code block markers if necessary
//             let trimmed_content = content_str.trim_start_matches("```json\n").trim_end_matches("\n```");
//             println!("trimmed_content {:?}", trimmed_content);
//             // Optionally, parse the inner JSON if it's structured
//             let inner_value: Vec<SimpleNodeAction> = serde_json::from_str(trimmed_content).unwrap();
//             println!("actions {:?}", inner_value);
//             return Ok(inner_value)
//         }
//     }
//     Err("Invalid response from OpenAI API".to_string())

// }
