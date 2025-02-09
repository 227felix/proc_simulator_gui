// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_dialog::FilePath;
mod proc;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use color_backtrace::install;

use proc::proc::*;
mod my_def;
use my_def::constants::*;

struct ProcessorState {
    processor: Mutex<Processor>,
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn file_test(file: &str) -> String {
    println!("file: {}", file);
    file.to_string()
}

#[tauri::command]
fn clock_processor(state: State<'_, Arc<ProcessorState>>) -> String {
    let mut processor = state.processor.lock().unwrap();
    processor.clock();
    let new_state = processor.get_state_serialized();
    new_state
}

#[tauri::command]
fn get_state(state: State<'_, Arc<ProcessorState>>) -> String {
    let processor = state.processor.lock().unwrap();
    processor.get_state_serialized()
}

#[tauri::command]
fn set_num_representation(state: State<'_, Arc<ProcessorState>>, representation: String) {
    let mut processor = state.processor.lock().unwrap();
    processor.set_num_rep(representation);
}

#[tauri::command]
fn reset_processor(state: State<'_, Arc<ProcessorState>>) -> String {
    let mut processor = state.processor.lock().unwrap();
    let new_processor = processor.reset();
    *processor = new_processor;
    processor.get_state_serialized()
}

#[tauri::command]
fn reload_program(state: State<'_, Arc<ProcessorState>>) -> String {
    let mut processor = state.processor.lock().unwrap();
    let new_processor = processor.reload_program();
    *processor = new_processor;
    processor.get_state_serialized()
}

#[tauri::command]
fn load_program(app: tauri::AppHandle, state: State<'_, Arc<ProcessorState>>) -> String {
    let mut processor = state.processor.lock().unwrap();
    let current_path = processor.get_rom_path();
    // open file dialog to get file path
    let rom_file = app
        .dialog()
        .file()
        .blocking_pick_file()
        .unwrap_or(FilePath::from(current_path))
        .into_path()
        .unwrap();
    let new_proc = processor.load_program(rom_file);
    *processor = new_proc;
    processor.get_state_serialized()
}

fn main() {
    install();
    tauri::Builder::default()
        .setup(|app| {
            let args: Vec<String> = env::args().collect();
            let proc = if args.len() > 1 {
                let rom_path = PathBuf::from(&args[1]);
                Processor::new(rom_path, "hex".to_string())
            } else {
                let current_dir = env::current_dir().unwrap();
                let dat_file = current_dir.join("rom.dat");
                println!("Looking for rom.dat in: {:?}", dat_file);
                if dat_file.exists() {
                    Processor::new(dat_file, "hex".to_string())
                } else {
                    println!("rom.dat not found, starting with empty rom");
                    Processor::new_empty_rom()
                }
            };

            let processor_state = ProcessorState {
                processor: Mutex::new(proc),
            };

            app.manage(Arc::new(processor_state));

            app.get_webview_window("main").unwrap().open_devtools(); // FIXME: remove this line

            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            file_test,
            clock_processor,
            get_state,
            set_num_representation,
            reset_processor,
            load_program,
            reload_program,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
