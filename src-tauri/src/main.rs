#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use diesel_migrations::{embed_migrations, EmbedMigrations};


use tauri::App;
// use schema::todos;
use std::error::Error;
use std::{error, string, sync::Mutex};

// Start of DB example
// use super::db::{};
#[macro_use]
extern crate diesel;
#[macro_use] 
extern crate diesel_migrations;
embed_migrations!("./migrations/");

use diesel::prelude::*;
pub mod schema;
pub mod db;


#[tauri::command]
fn todos_list(state: tauri::State<AppState>) -> String{
    let con = state.conn.lock().unwrap();
    db::todos_list(&con)
}
#[tauri::command]
fn todos_create(title: String, body: String, state: tauri::State<AppState>) -> String{
    let conn = state.conn.lock().unwrap();
    db::todos_create(&conn, &title, &body).to_string()
}

#[tauri::command]
fn todos_toggle(id: i32, state: tauri::State<AppState>) -> String{
    let conn = state.conn.lock().unwrap();
    db::todos_toggle(&conn, id)
}
#[tauri::command]
fn todos_delete(id: i32, state: tauri::State<AppState>) -> String {
    let conn = state.conn.lock().unwrap();
    db::todos_delete(&conn, id);
    String::from("")
}
struct AppState {
    conn: Mutex<SqliteConnection>,
}

fn main() {
    let _conn = db::establish_connection();
    let state = AppState {
        conn: Mutex::new(db::establish_connection()),
    };

  tauri::Builder::default().manage(state)
    .invoke_handler(tauri::generate_handler![
      todos_create,
      todos_toggle,
      todos_delete,
      todos_list
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
