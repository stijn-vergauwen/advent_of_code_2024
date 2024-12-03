use std::{fs, io};

// IO
#[derive(Clone, Copy)]
pub enum FileType {
    Input,
    Result,
}

pub fn save_data_to_asset_folder(data: String, day: u8, challenge: u8) -> Result<(), io::Error> {
    let path_to_dir = get_path_to_dir(day);

    if !fs::exists(&path_to_dir).unwrap_or(false) {
        fs::create_dir_all(path_to_dir).expect("Should be able to create missing dir");
    }

    fs::write(get_path_to_file(FileType::Result, day, challenge), data)
}

pub fn load_data_from_asset_folder(day: u8, challenge: u8) -> Result<String, io::Error> {
    fs::read_to_string(get_path_to_file(FileType::Input, day, challenge))
}

fn get_path_to_file(file_type: FileType, day: u8, challenge: u8) -> String {
    let path_to_dir = get_path_to_dir(day);
    let file_type = match file_type {
        FileType::Input => "input",
        FileType::Result => "result",
    };

    format!("{path_to_dir}/{file_type}_{challenge}.txt")
}

fn get_path_to_dir(day: u8) -> String {
    format!("assets/day_{day}")
}
