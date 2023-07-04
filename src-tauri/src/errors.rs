use std::fmt::{self};

pub const JSON_PARSE_ERROR: &str = "Value not found in settings file. Please open the settings page and ensure that all settings are set. View help for more info.";
pub const MAKE_DIR_ERROR: &str = "Could not make a directory at {}. Please see the help page and follow instructions on making a settings directory.";
pub const MAKE_FILE_ERROR: &str = "Could not make desired file {}. Please see the help page and follow the steps to diagnose.";
pub const EMPTY_SETTINGS_ERROR: &str = "Settings file is empty. Please fill out all settings before trying again.";
pub const PROFILE_OPEN_ERROR: &str = "Could not open shell profile ({}). Please check that the shell profile setting points to the right file and try again.";
pub const WRITE_TO_FILE_ERROR: &str = "Could not write to file {}. Please write manually";

#[derive(Debug)]
pub enum ModificationError {
	JSONParseError,
	MakeDirError,
	MakeFileError,
	EmptySettingsError,
  ProfileOpenError,
  WriteToFileError
}


impl std::error::Error for ModificationError {}
impl fmt::Display for ModificationError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::JSONParseError => write!(f, "Value not found in JSON file"),
      Self::MakeDirError => write!(f, "Could not make a directory. Please see help page and make the directory."),
      Self::MakeFileError => write!(f, "Could not make desired file. Please see help page and make the file."),
      Self::EmptySettingsError => write!(f, "Empty settings file. Please fill out settings first."),
      Self::ProfileOpenError => write!(f, "Could not open the shell profile. Check settings and try again."),
      Self::WriteToFileError => write!(f, "Could not write to file")
    }
  }
}