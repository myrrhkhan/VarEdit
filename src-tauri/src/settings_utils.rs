use std::{path::Path, process::Command, fs};
use crate::errors::ModificationError;

/// Check if a file exists, and if not, make one
/// ### Arguments:
/// - path_to_dir: path to directory (do not include "/")
/// - filename: name of file
/// ### Returns
/// Option<ModificationError>, where it'll either return an error or None if everything worked smoothly
/// ### Panics:
/// - whenever directory could not be made
/// - whenever file could not be made
pub fn check_and_make_file(path_to_dir: &str, filename: &str) -> Option<ModificationError> {

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
    fs::File::create(&full_path).expect("Couldn't create file");
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
pub fn gather_setting(settings_path: &str, key: &str) -> Result<String, ModificationError> {
  // read JSON to string
  let settings_text: String = fs::read_to_string(settings_path).expect("Could not find file or read from file");
  // Parse JSON string, return the resulting string or return a JSONParseError
  let json_result: Result<String, ModificationError> = 
    (serde_json::from_str(&settings_text)).map_err(|_| ModificationError::JSONParseError);
  return json_result;
}