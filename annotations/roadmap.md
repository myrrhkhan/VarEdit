# Roadmap/code organization

## Adding Environment Variables

### Backend:

- Make an outer function to input environment variable and add -> string status
	- Function to check if already there (to prevent duplicates)
	- Function to check OS
	- Mac/Linux:
		- Helper function to check for config files
		- Function to make directory and config files if they don't exist
	- Check for settings file and give error if not and return "set a thing"
	- Set the thingy