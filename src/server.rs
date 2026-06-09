use tiny_http::{Server, Response, Header};
use std::sync::{Arc, Mutex};
use std::path::Path;
use std::fs;
use std::thread;
use std::time::{Duration, SystemTime};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::compiler::Compiler;

pub fn start_server(file_path: &str, port: u16) {
    let path_to_watch = Path::new(file_path);
    if !path_to_watch.exists() {
        eprintln!("\x1b[31mError: File '{}' does not exist.\x1b[0m", file_path);
        return;
    }

    println!("\x1b[34m[Slate] Initial compile of '{}'...\x1b[0m", file_path);
    
    // Compile initially
    let html_content = match compile_file(file_path) {
        Ok(html) => html,
        Err(e) => {
            eprintln!("\x1b[31mInitial compile error: {}\x1b[0m", e);
            "<h1>Slate Compilation Error</h1><pre>Check terminal for logs</pre>".to_string()
        }
    };

    let compiled_html = Arc::new(Mutex::new(html_content));
    let reload_flag = Arc::new(Mutex::new(false));

    // Watcher thread using metadata polling (polling every 250ms)
    let compiled_html_watcher = Arc::clone(&compiled_html);
    let reload_flag_watcher = Arc::clone(&reload_flag);
    let watched_file_str = file_path.to_string();

    thread::spawn(move || {
        let path = Path::new(&watched_file_str);
        let mut last_modified = get_modified_time(path).unwrap_or_else(|_| SystemTime::now());

        loop {
            thread::sleep(Duration::from_millis(250));
            
            if let Ok(current_modified) = get_modified_time(path) {
                if current_modified != last_modified {
                    last_modified = current_modified;
                    
                    println!("\x1b[32m[Slate] File modified, recompiling...\x1b[0m");
                    match compile_file(&watched_file_str) {
                        Ok(new_html) => {
                            if let Ok(mut html_guard) = compiled_html_watcher.lock() {
                                *html_guard = new_html;
                            }
                            if let Ok(mut reload_guard) = reload_flag_watcher.lock() {
                                *reload_guard = true;
                            }
                            println!("\x1b[32m[Slate] Recompiled successfully!\x1b[0m");
                        }
                        Err(e) => {
                            eprintln!("\x1b[31m[Slate] Compilation error: {}\x1b[0m", e);
                        }
                    }
                }
            }
        }
    });

    // Start HTTP Server
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).expect("Failed to start HTTP server");

    println!("\x1b[35m[Slate] Server listening at http://localhost:{}\x1b[0m", port);
    println!("\x1b[35m[Slate] Watching for changes in '{}'. Press Ctrl+C to stop.\x1b[0m", file_path);

    for request in server.incoming_requests() {
        let url = request.url();
        match url {
            "/" | "/index.html" => {
                let html_guard = compiled_html.lock().unwrap();
                let mut response = Response::from_string(html_guard.clone());
                response.add_header(Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap());
                let _ = request.respond(response);
            }
            "/status" => {
                let mut reload_guard = reload_flag.lock().unwrap();
                let status_json = if *reload_guard {
                    *reload_guard = false;
                    r#"{"reload": true}"#
                } else {
                    r#"{"reload": false}"#
                };
                let mut response = Response::from_string(status_json.to_string());
                response.add_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
                response.add_header(Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap());
                let _ = request.respond(response);
            }
            _ => {
                let response = Response::from_string("404 Not Found").with_status_code(404);
                let _ = request.respond(response);
            }
        }
    }
}

fn get_modified_time(path: &Path) -> Result<SystemTime, std::io::Error> {
    fs::metadata(path).and_then(|m| m.modified())
}

fn compile_file(file_path: &str) -> Result<String, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    let lexer = Lexer::new(&content);
    let mut parser = Parser::new(lexer);
    let ast_nodes = parser.parse();

    if !parser.errors.is_empty() {
        let mut err_msg = String::new();
        for err in &parser.errors {
            err_msg.push_str(&format!("{}\n", err));
        }
        return Err(err_msg);
    }

    let compiler = Compiler::new(true); // Enable live reload script
    let html = compiler.compile(&ast_nodes);
    Ok(html)
}
