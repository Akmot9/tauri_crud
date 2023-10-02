// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use tauri::Result;
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct PacketInfo {
    mac_source: String,
    mac_destination: String,
    ethertype: String,
    ip_source: String,
    ip_destination: String,
    protocol: String,
    port_source: String,
    port_destination: String,
    count: u32,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_table() -> Result<PacketInfo> {
    println!("mohamed t'es mort !");
    // Ouvrez une connexion à la base de données SQLite
    let conn = Connection::open("my_database.db").unwrap();

    // Créez une table pour stocker les données
    conn.execute(
        "CREATE TABLE IF NOT EXISTS packet_info (
            id INTEGER PRIMARY KEY,
            mac_source TEXT,
            mac_destination TEXT,
            ethertype TEXT,
            ip_source TEXT,
            ip_destination TEXT,
            protocol TEXT,
            port_source TEXT,
            port_destination TEXT,
            count INTEGER
        )",
        [],
    ).unwrap();

    let packet_info = PacketInfo {
        mac_source: "aa:bb:cc:dd:ee:f2".to_string(),
        mac_destination: "11:22:33:44:55:66".to_string(),
        ethertype: "IPv4".to_string(),
        ip_source: "192.0.2.1".to_string(),
        ip_destination: "192.0.2.2".to_string(),
        protocol: "TCP".to_string(),
        port_source: "80".to_string(),
        port_destination: "8080".to_string(),
        count: 1,
    };

    // Insérez les données dans la table
    conn.execute(
        "INSERT INTO packet_info (mac_source, mac_destination, ethertype, ip_source, ip_destination, protocol, port_source, port_destination, count)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            packet_info.mac_source,
            packet_info.mac_destination,
            packet_info.ethertype,
            packet_info.ip_source,
            packet_info.ip_destination,
            packet_info.protocol,
            packet_info.port_source,
            packet_info.port_destination,
            packet_info.count,
        ],
    ).unwrap();

    // Récupérez les données de la table
    let mut stmt = conn.prepare("SELECT * FROM packet_info").unwrap();
    let packet_info_iter = stmt.query_map([], |row| {
        Ok(PacketInfo {
            mac_source: row.get(1)?,
            mac_destination: row.get(2)?,
            ethertype: row.get(3)?,
            ip_source: row.get(4)?,
            ip_destination: row.get(5)?,
            protocol: row.get(6)?,
            port_source: row.get(7)?,
            port_destination: row.get(8)?,
            count: row.get(9)?,
        })
    }).unwrap();

    for packet_info in packet_info_iter {
        println!("{:#?}", packet_info.unwrap());
        
    }
    Ok(packet_info)

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,create_table])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
