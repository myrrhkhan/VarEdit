// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, env};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("hi {}", name)
}

#[tauri::command]
fn getPath() -> HashMap<String, Vec<str>> {
  let mut names_and_vars = HashMap::new();
  for (n,v) in env::vars_os() {
    let entries_string: String = v.into_string();
    let entries = entries_string.split(":");
    let entries_vector: Vec<&str> = entries.collect();
    names_and_vars.insert(n, entries_vector);
  }
  return names_and_vars;
  
}
