mod config;
mod utils;
use std::{fs, path::Path};

use serde_json::{Map, Value};
use tauri_plugin_sql::{Migration, MigrationKind};

#[derive(serde::Serialize, serde::Deserialize)]
struct Project {
    name: String,
    path: String,
    directory_name: String,
    version: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Dependency {
    name: String,
    version: String,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn exist_directory(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

#[tauri::command]
fn get_locales(path: &str) -> Vec<String> {
    let path = std::path::Path::new(path);
    let locales = path.join(config::get_path_locales());

    let mut locales_rs = vec![];

    if !locales.exists() {
        return locales_rs;
    }

    match locales.read_dir() {
        Ok(dir) => {
            for entry in dir {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_dir() {
                    locales_rs.push(entry.file_name().into_string().unwrap());
                }
            }
        }
        Err(_) => {
            println!("Error reading directory");
        }
    }

    locales_rs
}

#[tauri::command]
fn get_info_project(path: &str) -> Option<Project> {
    let path = std::path::Path::new(path);
    let directory_name = path.file_name().unwrap().to_str().unwrap();

    if !path.join("package.json").exists() {
        return None;
    }

    let file_content = std::fs::read_to_string(path.join("package.json")).unwrap();
    let package: serde_json::Value = serde_json::from_str(&file_content).unwrap();
    let name = package["name"].as_str().unwrap();
    let version = package["version"].as_str().unwrap();

    Some(Project {
        name: name.to_string(),
        path: path.to_str().unwrap().to_string(),
        directory_name: directory_name.to_string(),
        version: version.to_string(),
    })
}

#[tauri::command]
fn get_dependencies(path: &str) -> Vec<Dependency> {
    let path = std::path::Path::new(path);
    let dependencies = path.join("package.json");

    if !dependencies.exists() {
        return vec![];
    }

    let file_content = std::fs::read_to_string(dependencies).unwrap();
    let package: serde_json::Value = serde_json::from_str(&file_content).unwrap();
    let dependencies = package["devDependencies"].as_object().unwrap();

    let mut dependencies_rs: Vec<Dependency> = vec![];

    for (key, value) in dependencies {
        dependencies_rs.push(Dependency {
            name: key.to_string(),
            version: value.as_str().unwrap().to_string(),
        });
    }

    dependencies_rs
}

#[tauri::command]
fn create_locale(path: &str, locale_rq: &str) -> bool {
    let binding = locale_rq.to_lowercase();
    let locale = binding.as_str();

    let path = std::path::Path::new(path);
    let locales = path.join(config::get_path_locales());

    if !locales.exists() {
        std::fs::create_dir(locales.clone()).unwrap();
    }

    let locale_path = locales.join(locale);

    if locale_path.exists() {
        return false;
    }

    let locales_files = utils::get_locales_files(path);

    fs::create_dir(locale_path.clone()).unwrap();

    match locales_files {
        Ok(files) => {
            for file in files {
                let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
                let file_path = locale_path.join(file_name);
                match fs::copy(file, file_path) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    match utils::generate_files(path.to_str().unwrap(), locale) {
        Ok(()) => {}
        Err(e) => {
            println!("Error al generar el archivo 'index.ts': {}", e);

            return false;
        }
    }

    return true;
}

#[tauri::command]
fn get_locales_files(project: &str, locale: &str) -> Option<Vec<String>> {
    let path = std::path::Path::new(project);

    let locales_files = utils::get_locales_files_directory(path, locale);

    if locales_files.is_err() {
        return None;
    }

    match locales_files {
        Ok(files) => {
            let mut files_rs = vec![];

            if files.is_empty() {
                return None;
            }

            for file in files {
                let path_file = Path::new(&file);
                let file_name = utils::get_name(&path_file.to_path_buf());

                files_rs.push(file_name.split('.').next().unwrap().to_string());
            }

            return Some(files_rs);
        }
        Err(_) => {
            println!("Error reading directory");
            return None;
        }
    }
}

#[tauri::command]
fn get_json_file(project: &str, locale: &str, file: &str) -> Option<Map<String, Value>> {
    let path = std::path::Path::new(project);
    let locales = path.join(config::get_path_locales());

    if !locales.exists() {
        return None;
    }

    let locale_path = locales.join(locale);

    if !locale_path.exists() {
        return None;
    }

    let file_path = locale_path.join(format!("{}.json", file.to_owned()).as_str());

    if !file_path.exists() {
        return None;
    }

    let file_content = std::fs::read_to_string(file_path).unwrap();
    let file_content_json: serde_json::Value = serde_json::from_str(&file_content).unwrap();

    return Some(file_content_json.as_object().unwrap().clone());
}

#[tauri::command]
fn save_json_file(project: &str, locale: &str, file: &str, data: Map<String, Value>) -> bool {
    let path = std::path::Path::new(project);
    let locales = path.join(config::get_path_locales());

    if !locales.exists() {
        return false;
    }

    let locale_path = locales.join(locale);

    if !locale_path.exists() {
        return false;
    }

    let file_path = locale_path.join(format!("{}.json", file.to_owned()).as_str());

    if !file_path.exists() {
        return false;
    }

    let result_json = serde_json::to_string_pretty(&data).unwrap();

    match fs::write(file_path, result_json) {
        Ok(_) => true,
        Err(e) => {
            println!("Error al guardar el archivo: {:?}", e);
            false
        }
    }
}

#[tauri::command]
fn create_json_file(project: &str, _locale: &str, file: &str) -> bool {
    let path = std::path::Path::new(project);
    let locales = path.join(config::get_path_locales());

    if !locales.exists() {
        return false;
    }

    for locale in get_locales(project) {
        let file_path = locales
            .join(locale.clone())
            .join(format!("{}.json", file.to_owned()).as_str());

        if file_path.exists() {
            continue;
        }

        match fs::write(file_path, "{}") {
            Ok(_) => {}
            Err(e) => {
                println!("Error al guardar el archivo: {:?}", e);
                continue;
            }
        }

        match utils::generate_files(project, &locale) {
            Ok(()) => {}
            Err(e) => {
                println!("Error al generar el archivo 'index.ts': {}", e);

                continue;
            }
        }

        println!("Se guardo el archivo '{}'", file);
    }

    return true;
}

#[tauri::command]
fn create_key_json_file(project: &str, _locale: &str, file: &str, name: &str) -> bool {
    let path = std::path::Path::new(project);
    let locales = path.join(config::get_path_locales());

    if !locales.exists() {
        return false;
    }

    for locale in get_locales(project) {
        let file_path = locales
            .join(locale.clone())
            .join(format!("{}.json", file.to_owned()).as_str());

        if !file_path.exists() {
            continue;
        }

        let file_content = std::fs::read_to_string(file_path.clone()).unwrap();
        let mut file_content_json: serde_json::Value = serde_json::from_str(&file_content).unwrap();

        let keys: Vec<&str> = name.split('.').collect();
        let value = serde_json::Value::String("".to_string());

        if keys.len() > 1 {
            let new_json = utils::build_deep_map(file_content_json, keys.as_slice());

            println!("{:?}", new_json);

            file_content_json = new_json;
        }

        let final_json_string = serde_json::to_string_pretty(&file_content_json).unwrap();

        match fs::write(file_path.clone(), final_json_string) {
            Ok(_) => {}
            Err(e) => {
                println!("Error al guardar el archivo: {:?}", e);
                continue;
            }
        }

        println!("Se guardo el archivo '{}'", file);

        match utils::generate_files(project, &locale) {
            Ok(()) => {}
            Err(e) => {
                println!("Error al generar el archivo 'index.ts': {}", e);

                continue;
            }
        }
    }

    return true;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations =
        vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql: "CREATE TABLE projects (id INTEGER PRIMARY KEY autoincrement, name TEXT, path TEXT)",
        kind: MigrationKind::Up,
    },
    Migration{
        version: 2,
        description: "add_directory_name",
        sql: "ALTER TABLE projects ADD directory_name TEXT",
        kind: MigrationKind::Up,
    }];
    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:mydatabase.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            exist_directory,
            get_info_project,
            get_locales,
            get_dependencies,
            create_locale,
            get_locales_files,
            get_json_file,
            save_json_file,
            create_json_file,
            create_key_json_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
