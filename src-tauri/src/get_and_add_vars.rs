use std::{collections::HashMap, env};
use crate::errors::ModificationError;
use crate::settings_utils;


#[tauri::command]
pub fn get_vars() -> HashMap<String, Vec<String>> {

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
pub fn add_var(key: String, var_submission: String) -> String {
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
  let file_status = settings_utils::check_and_make_file(
    "~/.config/varedit", 
    "settings.json"
  );

  match file_status {
    Some(error) => return Err(error),
    None => return settings_utils::gather_setting("~/.config/varedit", "shell_profile")
  }
}