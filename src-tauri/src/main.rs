// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, env, process::Command, path::{Path, self}, fs::{File, self}, io::Write};
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
fn check_if_exist(key: String, var_submission: String) -> bool {
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
fn append(key: String, var_submission: String) {

}

#[cfg(target_os = "macos")]
fn append(key: String, var_submission: String) {

}

/// Checks if a file exists, and if not, creates directories and files
/// and adds argument to file.
/// Used for making config files or for appending to additional config files
/// Can also be used for appending to shell profile files
fn check_for_config_files(path_to_dir: String, filename: String, arg: String) -> String {
  if !Path::new(&path_to_dir).exists() {
    Command::new("mkdir")
      .args([&path_to_dir])
      .output()
      .expect("ERROR: failed to make directory");
  }
  
  let full_path = [path_to_dir, filename].join("/");

  if !Path::new(&full_path).exists() {
    let mut _file = File::create(&full_path);
  }
  let mut file = File::options()
    .read(true)
    .append(true)
    .open(&full_path)
    .expect("Error opening file.");

  if let Err(e) = writeln!(file, "{}", arg) {
    return String::from("Couldn't write to file");
  } else {
    return String::from("Write successful");
  }
  
}