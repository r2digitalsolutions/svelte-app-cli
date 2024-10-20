use serde_json::{Map, Value};

use crate::config::{self, get_path_locales};
use std::{
    fs::{self},
    io::{self},
    path::{Path, PathBuf},
    vec,
};

pub fn get_locales_files(project_directory: &Path) -> Result<Vec<String>, std::io::Error> {
    let locales = project_directory.join(get_path_locales());

    if !locales.exists() {
        return Ok(vec![]);
    }

    match locales.read_dir() {
        Ok(dir) => {
            let mut locales_files = vec![];
            let mut locale: String = String::new();

            for entry in dir {
                if let Ok(entry) = entry {
                    if entry.file_type().unwrap().is_dir() {
                        locale = entry.file_name().into_string().unwrap();

                        break;
                    }
                }
            }

            if locale.is_empty() {
                return Ok(vec![]);
            }

            let locale_directory_path = locales.join(locale);

            for entry in locale_directory_path.read_dir().unwrap() {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name().into_string().unwrap();
                    if file_name != "_index.json" {
                        locales_files.push(
                            locale_directory_path
                                .join(file_name)
                                .to_str()
                                .unwrap()
                                .to_string(),
                        );
                    }
                }
            }

            return Ok(locales_files);
        }
        Err(_) => {
            println!("Error reading directory");
        }
    }

    return Ok(vec![]);
}

pub fn generate_index(project: &str, locale: &str) -> Result<(), std::io::Error> {
    let path = std::path::Path::new(project);
    let locales = path.join(config::get_path_locales());

    if !locales.exists() {
        std::fs::create_dir(locales.clone()).unwrap();
    }

    let locale_path = locales.join(locale);

    if !locale_path.exists() {
        return Err(io::Error::new(io::ErrorKind::Other, "Ocurrió un error"));
    }

    let mut content_keys = Map::new();

    for entry in locale_path.read_dir().unwrap() {
        match entry {
            Ok(entry) => {
                let file_name = entry.file_name().into_string().unwrap();

                if file_name == "_index.json" {
                    continue;
                }

                let file_path = locale_path.join(file_name);
                let file_content = std::fs::read_to_string(file_path).unwrap();
                let file_content_json: serde_json::Value =
                    serde_json::from_str(&file_content).unwrap();

                for (key, value) in file_content_json.as_object().unwrap() {
                    if value.is_object() {
                        let nested = flatten_json(value.clone());

                        for (nested_key, nested_value) in nested.as_object().unwrap() {
                            content_keys
                                .insert(format!("{}.{}", key, nested_key), nested_value.clone());
                        }

                        continue;
                    }

                    content_keys.insert(key.to_string(), value.clone());
                }
            }
            Err(_) => {
                println!("Error reading directory");
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Error reading directoru",
                ));
            }
        }
    }

    match fs::write(
        locale_path.join("_index.json"),
        serde_json::to_string_pretty(&content_keys).unwrap(),
    ) {
        Ok(_) => {
            println!("Se guardo el archivo '_index.json'");
        }
        Err(e) => eprintln!("Error al guardar el archivo '_index.json': {}", e),
    }

    Ok(())
}

pub fn flatten_json(obj: Value) -> Value {
    let mut flattened = Map::new();

    if let Value::Object(map) = obj {
        for (key, value) in map {
            if value.is_object() {
                let nested = flatten_json(value);
                for (nested_key, nested_value) in nested.as_object().unwrap() {
                    flattened.insert(format!("{}.{}", key, nested_key), nested_value.clone());
                }
            } else {
                flattened.insert(key, value);
            }
        }
    }

    Value::Object(flattened)
}

pub fn generate_files(project: &str, locale: &str) -> Result<(), std::io::Error> {
    if let Err(e) = generate_index(project, locale) {
        return Err(e);
    }

    if let Err(e) = generate_translations_ts(project) {
        return Err(e);
    }

    if let Err(e) = generate_translation_constants(project) {
        return Err(e);
    }

    Ok(())
}

pub fn generate_translations_ts(project: &str) -> Result<(), std::io::Error> {
    let path = std::path::Path::new(project);
    let locales_path = path.join(config::get_path_locales());

    if !locales_path.exists() {
        println!("No hay directorio locales");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "No hay directorio locales",
        ));
    }

    let locales = locales_path.read_dir().unwrap();
    let file_index = locales_path.join("index.ts");

    let mut locales_data: Vec<String> = vec![];

    for locale in locales {
        if let Ok(locale) = locale {
            let locale_path = locale.path();
            if locale_path.is_dir() {
                let locale_name = get_name(&locale_path);
                locales_data.push(locale_name.clone());
            }
        }
    }

    locales_data.sort();
    println!("{:?}", locales_data);

    let comment_autogenerated = format!("// Autogenerated by {}", env!("CARGO_PKG_NAME"));
    let locales_import_export = format!(
        "export const translations = {{ {} }};",
        locales_data.join(", ")
    );
    let import_locales = locales_data
        .into_iter()
        .map(|locale| format!("import {} from './{}/{}';", locale, locale, "_index.json"))
        .collect::<Vec<String>>()
        .join("\n");

    let file = fs::write(
        file_index,
        format!(
            "{}\n{}\n{}",
            comment_autogenerated, import_locales, locales_import_export
        ),
    );

    match file {
        Ok(_) => println!("File written successfully"),
        Err(e) => {
            println!("Error writing file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn get_name(locale_path: &PathBuf) -> String {
    locale_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

pub fn generate_translation_constants(project: &str) -> Result<(), std::io::Error> {
    let path = std::path::Path::new(project);
    let locales_path = path.join(config::get_path_locales());

    if !locales_path.exists() {
        println!("No hay directorio locales");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "No hay directorio locales",
        ));
    }

    let locales = locales_path.read_dir().unwrap();

    // get one locale _index.json
    let mut locale_key = String::new();

    for locale in locales {
        if let Ok(locale) = locale {
            let locale_path = locale.path();
            if locale_path.is_dir() {
                let locale_name = get_name(&locale_path);
                locale_key = locale_name.clone();
                break;
            }
        }
    }

    let index_translations = locales_path.join(locale_key).join("_index.json");

    if !index_translations.exists() {
        println!("No hay archivo '_index.json'");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "No hay archivo '_index.json'",
        ));
    }

    let file_content = std::fs::read_to_string(index_translations).unwrap();
    let file_content_json: serde_json::Value = serde_json::from_str(&file_content).unwrap();

    return generate_keys_file(project, file_content_json.to_string());
}

pub(crate) fn generate_keys_file(project: &str, obj_keys: String) -> Result<(), std::io::Error> {
    let path = std::path::Path::new(project);
    let ruta_locales = path.join(config::get_path_locales());

    if !ruta_locales.exists() {
        eprintln!("El directorio 'locales' no existe o no es un directorio.");
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "El directorio 'locales' no existe o no es un directorio.",
        ));
    }

    let parsed: Value = serde_json::from_str(&obj_keys).expect("Error al parsear el JSON");

    let result = recursive_keys(parsed, "".to_string());

    let comment_autogenerated = format!("// Autogenerated by {}\n", env!("CARGO_PKG_NAME"));
    let mut template = comment_autogenerated;

    template.push_str("export const I18N_TRANSLATIONS = {");

    let template_end = "\n} as const;";

    let result_keys: Vec<String> = result
        .keys()
        .map(|k| k.to_string())
        .collect::<Vec<String>>();

    for key in result_keys {
        let key_object = key.to_uppercase().replace(".", "_").replace("-", "_");
        template.push_str(&format!("\n\t{}: \"{}\",", key_object, key));
    }

    template.push_str(&template_end);

    fs::write(ruta_locales.join("translations.ts"), template)
        .expect("Error al escribir el archivo");

    println!("Se guardo el archivo 'translations.ts'");

    Ok(())
}

fn recursive_keys(obj: Value, prefix: String) -> Map<String, Value> {
    let mut keys = Map::new();

    if let Value::Object(map) = obj {
        for (key, value) in map {
            let full_key = if prefix.is_empty() {
                key.clone() // Si no hay prefijo, usamos la clave tal cual
            } else {
                format!("{}.{}", prefix, key) // Si hay prefijo, lo concatenamos
            };

            if value.is_object() {
                // Si el valor es un objeto, llamamos recursivamente
                let nested_keys = recursive_keys(value, full_key.clone());
                keys.extend(nested_keys); // Extendemos el mapa actual con los resultados
            } else {
                // Si es un valor simple, lo agregamos al mapa
                keys.insert(full_key.clone(), Value::String(full_key.clone()));
            }
        }
    }

    keys
}
