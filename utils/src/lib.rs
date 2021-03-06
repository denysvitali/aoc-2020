use std::path::Path;

pub fn get_file(file: &str, file_name : &str) -> String {
    Path::new(file).parent().unwrap()
        .join(format!("{}{}", "./input/", file_name))
        .to_str().unwrap().to_owned()
}