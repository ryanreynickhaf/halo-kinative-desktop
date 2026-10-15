// Sembunyikan jendela konsol di Windows saat rilis.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("gagal menjalankan aplikasi");
}
