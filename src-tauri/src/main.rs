#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Manager};

#[derive(Default)]
struct SharedState {
    stop_listen: bool,
}
// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Define a type for your window map.
// Lazy init static variables
lazy_static! {
    // This is a static variable that can be accessed from anywhere in the program
    static ref APP: OnceCell<AppHandle> = OnceCell::new();
}
// In your main function, initialize the window map.

async fn copied(window: tauri::Window) {
    println!("ctrl c pressed");

    window
        .emit(
            "copied",
            Payload {
                message: "Tauri is awesome!".into(),
            },
        ) // Emit an event that JavaScript side will listen to
        .expect("failed to emit");
}
async fn pasted(window: tauri::Window) {
    println!("ctrl v pressed");

    window
        .emit(
            "pasted",
            Payload {
                message: "Tauri is awesome!".into(),
            },
        ) // Emit an event that JavaScript side will listen to
        .expect("failed to emit");
}

#[tauri::command]
fn start_listen(state: tauri::State<Arc<Mutex<SharedState>>>) {
    let state = state.inner().clone(); // Clone the Arc
    println!("start called ");
    thread::spawn(move || {
        let device_state: DeviceState = DeviceState::new();

        let _guard = device_state.on_key_up(move |key| {
            // Capture the cloned Arc in this closure

            if *key == Keycode::C {
                let device_state1: DeviceState = DeviceState::new();
                let keys: Vec<Keycode> = device_state1.get_keys();
                if keys.contains(&Keycode::Meta) {
                    let app = APP.wait();
                    let main_window = app.get_window("main").unwrap();
                    tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap()
                        .block_on(copied(main_window)); // Await the async function
                }
            }

            if *key == Keycode::V {
                let device_state1: DeviceState = DeviceState::new();
                let keys: Vec<Keycode> = device_state1.get_keys();
                if keys.contains(&Keycode::Meta) {
                    let app = APP.wait();
                    let main_window = app.get_window("main").unwrap();
                    tokio::runtime::Builder::new_current_thread()
                        .enable_all()
                        .build()
                        .unwrap()
                        .block_on(pasted(main_window)); // Await the async function
                }
            }
        });

        loop {
            if state.lock().unwrap().stop_listen {
                println!("Received stop signal, stopping loop...");
                state.lock().unwrap().stop_listen = false; // Reset stop_listen
                break;
            }
        }
    });
}

#[tauri::command]
fn stop_listen(state: tauri::State<Arc<Mutex<SharedState>>>) {
    state.inner().lock().unwrap().stop_listen = true;
}

fn main() {
    let state = Arc::new(Mutex::new(SharedState::default()));

    tauri::Builder::default()
        .setup(move |app| {
            APP.get_or_init(|| app.app_handle());

            Ok(())
        })
        .manage(state)
        .invoke_handler(tauri::generate_handler![start_listen, stop_listen,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
