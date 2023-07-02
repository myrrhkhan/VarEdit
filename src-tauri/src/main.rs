// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, env, process::Command, path::{Path}, fs::{File, self}, fmt};
use set_env;

#[derive(Debug)]
pub enum ModificationError {
  JSONParseError,
  MakeDirError,
  MakeFileError,
  EmptySettingsError
}
impl std::error::Error for ModificationError {}
impl fmt::Display for ModificationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::JSONParseError => write!(f, "Value not found in JSON file"),
      Self::MakeDirError => write!(f, "Could not make a directory. Please see help page and make the directory."),
      Self::MakeFileError => write!(f, "Could not make desired file. Please see help page and make the file."),
      Self::EmptySettingsError => write!(f, "Empty settings file. Please fill out settings first.")
    }
  }
}

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
fn append(key: String, var_submission: String) -> Result<String, ModificationError> {
  // make settings file if not already made
  let file_status = check_and_make_file(
    "~/.config/varedit", 
    "settings.json"
  );

  match file_status {
    Some(error) => return Err(error),
    None => return gather_setting("~/.config/varedit", "shell_profile")
  }


}

/// Check if a file exists, and if not, make one
/// ### Arguments:
/// - path_to_dir: path to directory (do not include "/")
/// - filename: name of file
/// ### Returns
/// Option<ModificationError>, where it'll either return an error or None if everything worked smoothly
/// ### Panics:
/// - whenever directory could not be made
/// - whenever file could not be made
fn check_and_make_file(path_to_dir: &str, filename: &str) -> Option<ModificationError> {

  // check if directory exists, if not make the directory
  if !Path::new(&path_to_dir).exists() {
    let mkdir_status = Command::new("mkdir")
      .args([&path_to_dir])
      .output()
      .map_err(|_| ModificationError::MakeDirError);
    match mkdir_status {
      // return error if could not make directory
      Err(error) => return Some(error),
      // if ok, ignore output and continue function
      Ok(_) => ()
    }
  }
  
  // Establish full path as a variable
  let full_path = [path_to_dir, filename].join("/");

  // if a file doesn't exist, make the file
  if !Path::new(&full_path).exists() {
    File::create(&full_path).expect("Couldn't create file");
    return Some(ModificationError::EmptySettingsError);
  }

  return None;  
}

/// Reads the JSON settings file, finds the value for a setting, and returns it
/// This program re-reads the JSON file every time a setting is needed in case the file is edited during runtime
/// ### Arguments
/// - settings_path: path to JSON file
/// - key: the setting that should be gathered
/// ### Returns:
/// The setting value as a String, or an Error
/// ### Panics:
/// - If the settings file could not be found or read into a JSON
fn gather_setting(settings_path: &str, key: &str) -> Result<String, ModificationError> {
  // read JSON to string
  let settings_text: String = fs::read_to_string(settings_path).expect("Could not find file or read from file");
  // Parse JSON string, return the resulting string or return a JSONParseError
  let json_result: Result<String, ModificationError> = 
    (serde_json::from_str(&settings_text)).map_err(|_| ModificationError::JSONParseError);
  return json_result;
}