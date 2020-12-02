use std::path::Path;

pub(crate) fn get_file(file_name : &str) -> String {
    Path::new(file!()).parent().unwrap()
        .join(format!("{}{}", "./input/", file_name))
        .to_str().unwrap().to_owned()
}