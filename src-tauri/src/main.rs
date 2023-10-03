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


#[tauri::command]
fn create_table() -> Result<()> {
    
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
    println!("table created.");
    Ok(())

}

#[tauri::command]
fn get_packet_infos() -> Result<Vec<PacketInfo>> {
    // Open a connection to the SQLite database
    let conn = Connection::open("my_database.db").unwrap();

    // Retrieve data from the table
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

    let mut packet_infos = Vec::new();

    for packet_info in packet_info_iter {
        packet_infos.push(packet_info.unwrap());
    }

    Ok(packet_infos)
}

#[tauri::command(rename_all = "snake_case")]
fn insert_packet_info(packet_info: PacketInfo) -> Result<()> {
    println!("I was invoked from JS, with this message: {:?}", &packet_info);
    // Open a connection to the SQLite database
    let conn = Connection::open("my_database.db").unwrap();

    // Insert the provided `PacketInfo` into the table
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

    Ok(())
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_table,insert_packet_info, get_packet_infos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
