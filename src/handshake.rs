use arrayvec::ArrayString;
use std::ffi::CString;

// based on: <https://dev.mysql.com/doc/dev/mysql-server/latest/page_protocol_connection_phase_packets_protocol_handshake_v10.html>
#[derive(Debug, Clone)]
struct MySQLHandshake {
    // Always 10
    protocol_version: u8,
    // Human readable status information
    server_version: CString,
    // connection id
    thread_id: u32,
    // first 8 bytes of auth plugin provided data
    auth_plugin_data_part_1: ArrayString<8>,
    // 0x00 byte, terminates the previous field
    filler: u8,
    // lower 2 bytes of capabilities flags
    capability_flags_1: u16,
    // lower 8 bits of the server character set
    character_set: u8,
    // server status
    status_flags: u16,
    // upper 2 bytes of the capabilities flags
    capability_flags_2: u16,
    // length of the combined auth_plugin_data
    auth_plugin_data_len: u8,
    // reserved
    reserved: ArrayString<10>,
    // TODO: Define length encoded string for auth-plugin-data-part-2

    // name of the auth_method that the auth_plugin_data belongs to
    auth_plugin_name: CString,
}
