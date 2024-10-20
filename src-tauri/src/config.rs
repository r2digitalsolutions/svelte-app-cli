const PATH_SRC: &'static str = "src";
const PATH_LIB: &'static str = "lib";
const PATH_LOCALES: &'static str = "locales";

pub fn get_path_src() -> String {
    PATH_SRC.to_string()
}

pub fn get_path_lib() -> String {
    format!("{}/{}", PATH_SRC, PATH_LIB)
}

pub fn get_path_locales() -> String {
    format!("{}/{}/{}", PATH_SRC, PATH_LIB, PATH_LOCALES)
}
