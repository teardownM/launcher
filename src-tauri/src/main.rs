#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::path::Path;
use std::fs;
use directories::{BaseDirs};

fn install_sledge(sledge_path: &Path) {
  fs::create_dir_all(sledge_path).unwrap_or_else(|e| panic!("Error creating dir: {}", e));

  // TODO:
  // 1. Download latest Sledge release 
  // 2. Unzip the folder
}

#[tauri::command]
fn check_teardownm() {
  if let Some(base_dirs) = BaseDirs::new() {
    println!("Checking TeardownM install");

    let appdata_path = base_dirs.data_local_dir();
    let teardownm_buf = appdata_path.join("TeardownM").join("mods").join("TeardownM");
    let teardownm_path = teardownm_buf.as_path();

    if teardownm_path.exists() {
      let required_files: [&str; 5] = ["TeardownM.dll", "TeardowmM.info", "assets", "dependencies", "lua"];

      for file in &required_files {
        match teardownm_path.join(file).as_path().exists() {
          true => println!("{} found", file),
          false => eprintln!("{} not found", file)
        }
      }
     } else {
      println!("TeardownM folder inside Sledge mod not found");

      // TODO:
      // 1. Download TeardownM release
      // 2. Unzip folder
    }
  }
}

#[tauri::command]
fn check_sledge() {
  println!("Checking Sledge install");

  if let Some(base_dirs) = BaseDirs::new() {
    let appdata_path = base_dirs.data_local_dir();
    let sledge_buf = appdata_path.join("TeardownM");
    let sledge_path = sledge_buf.as_path();

    if sledge_path.exists() {
      // TODO: Check hash of Sledge to ensure they match latest release
      let required_files: [&str; 5] = ["sledge.exe", "sledge.dll", "ultralight", "ui", "mods"];

      for file in &required_files {
        match sledge_path.join(file).as_path().exists() {
          true => println!("{} found", file),
          false => eprintln!("{} not found", file)
        }
      }
     } else {
      println!("Sledge folder not found");
      install_sledge(sledge_path);
    }
  }
} 

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![check_sledge, check_teardownm])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
