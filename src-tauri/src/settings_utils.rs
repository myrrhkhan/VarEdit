use std::{path::Path, process::Command, fs};
use crate::errors::*;

/// Check if a file exists, and if not, make one
/// ### Arguments:
/// - path_to_dir: path to directory (do not include "/")
/// - filename: name of file
/// ### Returns
/// Either an empty string if the file exists or was successfully made, or an error message as a String
pub fn check_and_make_file(path_to_dir: &str, filename: &str) -> Result<String, String> {

  // check if directory exists, if not make the directory
  if !Path::new(&path_to_dir).exists() {
    Command::new("mkdir")
      .args([&path_to_dir])
      .output()
      .map_err(|err| construct_err_msg!(mkdir_err!(&path_to_dir), err.to_string()))?;
  }
  
  // Establish full path as a variable
  let full_path = [path_to_dir, filename].join("/");

  // if a file doesn't exist, make the file or return the error
  if !Path::new(&full_path).exists() {
    fs::File::create(&full_path).map_err(
      |err| 
      construct_err_msg!(
        make_file_err!(&full_path), err.to_string()
      )
    )?;
    return Err(String::from(empty_settings_err!()));
  }

  return Ok(String::from(""));  
}

/// Reads the JSON settings file, finds the value for a setting, and returns it
/// This program re-reads the JSON file every time a setting is needed in case the file is edited during runtime
/// ### Arguments
/// - settings_path: path to JSON file
/// - key: the setting that should be gathered
/// ### Returns:
/// Either a String with the setting or an error
pub fn gather_setting(settings_path: &str, key: &str) -> Result<String, String> {
  // read JSON to string, return error
  let settings_text: String = fs::read_to_string(&settings_path)
    .map_err(|err| construct_err_msg!(settings_read_error!(&settings_path), err.to_string()))?;
  // Parse JSON string, return the resulting string or return error string
  // TODO use key
  return (serde_json::from_str(&settings_text))
    .map_err(
      |err| 
      construct_err_msg!(
        json_parse_err!(), 
        err.to_string()
      ))?;
}