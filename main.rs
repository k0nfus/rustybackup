use chrono::{Local, Datelike};
use std::fs;

fn main() {
    let desktop_path = get_desktop_path();
    let source_file = "C:\\Users\\Jan\\Desktop\\test.txt";
    let backup_file = create_backup_file_name(&desktop_path);

    match fs::copy(source_file, &backup_file) {
        Ok(_) => {
            println!("Backup erfolgreich erstellt: {}", backup_file);
        }
        Err(err) => {
            eprintln!("Fehler beim Erstellen des Backups: {}", err);
        }
    }
}

fn get_desktop_path() -> String {
    let desktop = if cfg!(target_os = "windows") {
        match std::env::var("USERPROFILE") {
            Ok(path) => path + "\\Desktop\\",
            Err(_) => String::from(""),
        }
    } else {
        String::from("")
    };
    desktop
}

fn create_backup_file_name(desktop_path: &str) -> String {
    let local_time = Local::now();
    let year = local_time.year();
    let month = local_time.month();
    let day = local_time.day();

    let backup_file_name = format!(
        "{}{}_{}_{}_Backup_test.txt",
        desktop_path, year, month, day
    );

    backup_file_name
}
