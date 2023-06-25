// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, env};
use set_env;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_vars, add_var])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn get_vars() -> HashMap<String, Vec<String>> {

  // create map for variables and entries
  let mut names_and_vars: HashMap<String, Vec<String>> = HashMap::new();
  
  // procedure to save all keys and vals into map
  for (key, vals) in env::vars() {
    // convert string into vector by splitting, then map &str to String
    let entries: Vec<String> = vals.split(":").map(str::to_string).collect();
    names_and_vars.insert(key, entries);
  }

  return names_and_vars;
  
}

#[tauri::command]
fn add_var(key: String, var_submission: String) -> String {
  let mut err_msg: String = String::from(""); // error message for frontend to process
  if !(var_submission.contains("\0") || var_submission.is_empty()) {
    set_env::append(key, var_submission).expect("error adding variable.");
  } else {
    err_msg.push_str("Invalid input, contains null character or is empty.");
  }

fn find_default_shell() {
  let vars: HashMap<String, Vec<String>> = get_vars();
  let mut shell: String;
  match vars.get("SHELL") {
    None => panic!("could not get shell"),
    Some(shellVar) => {shell = format!("{}", (*shellVar)[0])},
  }
}

  println!("{}", err_msg);
  return err_msg;
}