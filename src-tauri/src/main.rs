// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, env};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_vars, add_var])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn get_vars() -> HashMap<String, Vec<String>> {

  // create map for variables and entries
  let mut names_and_vars = HashMap::new();
  
  // procedure to save all keys and vals into map
  for (key, val) in env::vars_os() {
    // save variable entries as a string
    let entries_string: String = val.into_string().expect("error could not convert to string");
    let entries = entries_string.split(":"); // split variables
    let entries_vector: Vec<&str> = entries.collect::<Vec<&str>>(); // convert split into vector of &str
    // convert vector of &str to vector of String
    let mut final_entries: Vec<String> = Vec::new();
    for i in 0..(entries_vector.len()) {
      let element: String = entries_vector[i].to_string();
      final_entries.push(element);
    }
    
    // add to map
    names_and_vars.insert(key.into_string().expect(""), final_entries);
  }

  return names_and_vars;
  
}

#[tauri::command]
fn add_var(key: String, var: String) -> String {
  let mut err_msg: String = String::from(""); // returns error message for frontend to process
  if !(var.contains("\0") || var.is_empty()) {
    env::set_var(key, var);
  } else {
    err_msg.push_str("Invalid input, contains null character.");
  }

  return err_msg;
}