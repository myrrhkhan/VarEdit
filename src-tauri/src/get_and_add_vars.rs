use std::fs::{self, File};
use std::io::Write;
use std::{collections::HashMap, env};
use crate::errors::ModificationError;

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
pub fn add_var(key: String, var_submission: String) {
  let mut err_msg: String = String::from(""); // error message for frontend to process
  if !(var_submission.contains("\0") || var_submission.is_empty()) {
    let duplicate = check_if_var_duplicate(&key, &var_submission);
    if duplicate {
      return;
    }
    let append_status = append(&key, &var_submission);
    match append_status {
      Some(_) => return,
      None => return
    }
  } else {
    err_msg.push_str("Invalid input, contains null character or is empty.");
  }
}


/// Checks if a variable being submitted already exists, returns boolean
/// # Arguments
/// key: variable
/// var_submission: desired submissions
fn check_if_var_duplicate(key: &String, var_submission: &String) -> bool {
	let status: bool;
  
	let map: HashMap<String, Vec<String>> = get_vars();
	let entries_option: Option<&Vec<String>> = map.get(key);
  
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
fn append(key: &String, var_submission: &String) -> Option<ModificationError> {
  // make settings file if not already made

use crate::settings_utils::{check_and_make_file, gather_setting};
  let file_status = check_and_make_file(
    "~/.config/varedit", 
    "settings.json"
  );

  // check if settings file could be made
  match file_status {
    Some(error) => return Some(error), // if not return
    // if so, get the setting and add
    None => ()
  }

  let shell_string_status = gather_setting(
    "~/.config/varedit/settings.json", 
    "shell_profile"
  );

  match shell_string_status {
    Err(error) => return Some(error),
    Ok(shell_string) => {
      write_to_file(shell_string, &key, &var_submission);
      return None;
    }
  }
}

fn write_to_file (shell_string: String, key: &String, var_submission: &String) -> Option<ModificationError> {
  let file_status = fs::OpenOptions::new()
    .append(true)
    .open(shell_string);

  match file_status {
    Err(_) => return Some(ModificationError::BashFileOpenError),
    Ok(_) => ()
  }

  // get file
  let mut file: File = file_status.unwrap();
  // make string to add to end of file
  let export_cmd = format!(
    "export {}=${}:{}", 
    key, 
    key, 
    var_submission
  );

  let write_status = file.write(export_cmd.as_bytes());
  
  match write_status {
    Err(_) => return Some(ModificationError::WriteToFileError),
    Ok(_) => return None
  }
}