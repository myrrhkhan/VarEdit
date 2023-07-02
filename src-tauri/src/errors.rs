use std::fmt;

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