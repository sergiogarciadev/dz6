use std::error::Error;
use std::fs;

use crate::app::App;

impl App {
    pub fn save_database(&self) -> Result<(), Box<dyn Error>> {
        // do not save a database if there's nothing to be saved
        if self.hex_view.bookmarks.is_empty() && self.hex_view.comment_name_list.is_empty() {
            return Ok(());
        }

        let toml_string = toml::to_string_pretty(&self.hex_view)?;
        let dbname = self.file_info.name.clone() + ".dz6";
        fs::write(dbname, toml_string)?;
        Ok(())
    }
    pub fn load_database(&mut self) -> Result<(), Box<dyn Error>> {
        let dbname = self.file_info.name.clone() + ".dz6";
        let data = fs::read_to_string(dbname)?;
        self.hex_view = toml::from_str(&data)?;
        Ok(())
    }
}
