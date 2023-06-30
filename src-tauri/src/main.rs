// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, env, process::Command, path::{Path}, fs::{File, self}};
use serde_json::{Value, Error};
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

  println!("{}", err_msg);
  return err_msg;
}

/// Checks if a variable being submitted already exists, returns boolean
/// # Arguments
/// key: variable
/// var_submission: desired submissions
fn check_if_var_duplicate(key: String, var_submission: String) -> bool {
  let status: bool;

  let map: HashMap<String, Vec<String>> = get_vars();
  let entries_option: Option<&Vec<String>> = map.get(&key);

  match entries_option {
    None => status = false,
    Some(entries) => 
      if entries.contains(&var_submission) {
        status = true;
      } else {
        status = false;
      }
  }

  return status;
}

#[cfg(target_os = "windows")]
fn append(key: String, var_submission: String) -> String {
  let output = Command::new("SetX")
    .args([var_submission, key])
    .output()
    .expect("ERROR: command failed to start");

  return output.status;
}

#[cfg(target_os = "linux")]
fn append(key: String, var_submission: String) -> String {

}

#[cfg(target_os = "macos")]
fn append(key: String, var_submission: String) {

}

/// Template for making configuration files
fn check_for_file(path_to_dir: &str, filename: &str) {

  // check if directory exists, if not make the directory
  if !Path::new(&path_to_dir).exists() {
    Command::new("mkdir")
      .args([&path_to_dir])
      .output()
      .expect("ERROR: failed to make directory");
  }
  
  // Establish full path as a variable
  let full_path = [path_to_dir, filename].join("/");

  // if a file doesn't exist, make the file
  if !Path::new(&full_path).exists() {
    File::create(&full_path).expect("Couldn't create file");
  }  
}

fn gather_setting(settings_path: &str, key: &str) -> Option<String> {
  let settings_text: String = fs::read_to_string(settings_path).expect("Unable to read file");
  let json_result: Result<Value, Error> = serde_json::from_str(&settings_text);
  match json_result {
    Err(err) => return None,
    Ok(json) => return Some(json[key].to_string()),
  }
}