use crate::config::ConfigPaths;
use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

// `pub` so `main.rs` can call it.
pub fn handle_prefs_get(paths: &ConfigPaths, key: &str) -> Result<()> {
    let json_data = read_or_create_prefs(&paths.user_pref_file)?;
    
    let json_pointer = format!("/{}", key.replace('.', "/"));
    
    match json_data.pointer(&json_pointer) {
        Some(value) => {
            if value.is_string() {
                println!("{}", value.as_str().unwrap());
            } else {
                println!("{}", serde_json::to_string_pretty(value)?);
            }
        }
        None => {
            println!("not found: {}", key);
        }
    }
    Ok(())
}

// `pub` so `main.rs` (and `wallpaper.rs`) can call it.
pub fn handle_prefs_set(paths: &ConfigPaths, key: &str, value_str: &str) -> Result<()> {
    let mut json_data = read_or_create_prefs(&paths.user_pref_file)?;
    let parsed_value = parse_value_str(value_str);
    
    let path_parts: Vec<&str> = key.split('.').collect();
    
    set_nested_value(&mut json_data, &path_parts, parsed_value)?;
    
    let new_content = serde_json::to_string_pretty(&json_data)?;
    fs::write(&paths.user_pref_file, new_content)?;
    
    println!("{}", value_str);
    Ok(())
}


// --- Helper Functions ---
// (These are NOT `pub`, they are private to this module)

fn read_or_create_prefs(path: &Path) -> Result<Value> {
    if !path.exists() {
        println!("Creating new preferences file at {:?}", path);
        fs::write(path, "{}")?;
        return Ok(json!({}));
    }
    let content = fs::read_to_string(path)
        .context(format!("Failed to read preferences file at {:?}", path))?;
    let json_data: Value = serde_json::from_str(&content)
        .context("Failed to parse preferences.json. Is it valid JSON?")?;
    Ok(json_data)
}

fn parse_value_str(value_str: &str) -> Value {
    if let Ok(b) = value_str.parse::<bool>() {
        return json!(b);
    }
    if let Ok(n) = value_str.parse::<i64>() {
        return json!(n);
    }
    if let Ok(f) = value_str.parse::<f64>() {
        return json!(f);
    }
    json!(value_str)
}

fn set_nested_value<'a>(
    current: &'a mut Value,
    path: &[&str],
    value: Value,
) -> Result<()> {
    if path.is_empty() {
        return Ok(());
    }
    
    if path.len() == 1 {
        let key = path[0];
        if let Some(obj) = current.as_object_mut() {
            obj.insert(key.to_string(), value);
            Ok(())
        } else {
            bail!("Invalid path: trying to set key '{}' on a non-object", key);
        }
    } else {
        let key = path[0];
        let next = current
            .as_object_mut()
            .context(format!("Invalid path: '{}' is not an object", key))?
            .entry(key.to_string())
            .or_insert_with(|| json!({}));
        
        set_nested_value(next, &path[1..], value)
    }
}