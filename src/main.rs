/* 
 * This program is free software. It comes without any warranty, to
 * the extent permitted by applicable law. You can redistribute it
 * and/or modify it under the terms of the Do What The Fuck You Want
 * To Public License, Version 2, as published by Sam Hocevar. See
 * http://www.wtfpl.net/ for more details. 
 */

use std::fs::File;
use std::io::{Read};

fn calc_checksum(bytes : &Vec<u8>) -> u8 {
    let mut sum : u32 = 0;
    for x in bytes {
        sum += *x as u32;
    }
    let sum_as_byte : u8 = (sum & 0xFF) as u8;
    !(sum_as_byte) + 0x01
}

#[test]
fn test_calc_checksum(){
    let bytes = vec![0x10, 0x01, 0x00, 0x00, 0x21, 0x46, 0x01, 0x36, 0x01, 0x21, 0x47, 0x01, 0x36, 0x00, 0x7E, 0xFE, 0x09, 0xD2, 0x19, 0x01];
    assert!(calc_checksum(&bytes) == 0x40);
}

fn convert_to_line(bytes : &Vec<u8>, address : u16, type_byte : u8) -> String {
    if bytes.len() > 0x10 {
        String::from("");
    }
    let mut checksum_bytes = Vec::new();
    checksum_bytes.push(bytes.len() as u8);
    checksum_bytes.push(((address >> 8) & 0xFF) as u8);
    checksum_bytes.push((address & 0xFF) as u8);
    checksum_bytes.push(type_byte);
    for x in bytes {
        checksum_bytes.push(*x);
    }
    let checksum = calc_checksum(&checksum_bytes);
    let mut line = String::from(":");
    line.push_str(&format!("{:02X}", bytes.len()));
    line.push_str(&format!("{:04X}", address));
    line.push_str(&format!("{:02X}", type_byte));
    let byte_strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    line.push_str(&byte_strs.join(""));
    line.push_str(&format!("{:02X}", checksum));
    line
}

#[test]
fn test_convert_to_line(){
    let bytes = vec![0x21, 0x46, 0x01, 0x36, 0x01, 0x21, 0x47, 0x01, 0x36, 0x00, 0x7E, 0xFE, 0x09, 0xD2, 0x19, 0x01];
    let address : u16 = 0x0100;
    let type_byte : u8 = 0x00;
    assert_eq!(convert_to_line(&bytes, address, type_byte), String::from(":10010000214601360121470136007EFE09D2190140"));
}

fn read_file(filename : &String) -> Vec<String>{
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read the file");
    let split_string = contents.split("\n");
    let mut lines : Vec<String> = Vec::new();
    for string in split_string {
        let trimmed_string : String = String::from(string.trim());
        if !trimmed_string.is_empty() {
            lines.push(trimmed_string);
        }
    }
    lines
}

#[test]
fn test_read_file(){
    let lines : Vec<String> = read_file(&String::from("zeKnirf.txt"));
    assert_eq!(lines.len(), 8);
    assert_eq!(lines[0], "02AB1D00");
    assert_eq!(lines[1], "DEADBEEF");
    assert_eq!(lines[2], "DEADC0DE");
    assert_eq!(lines[3], "C0FFEE00");
    assert_eq!(lines[4], "DEADCAFF");
    assert_eq!(lines[5], "C0DEBEEF");
    assert_eq!(lines[6], "BEEFCAFF");
    assert_eq!(lines[7], "C0DECAFF");
}

fn parse_hex_string(hex_string : &String) -> Vec<u8> {
    if hex_string.len() % 2 != 0 {
        return Vec::new()
    }
    let mut bytes : Vec<u8> = Vec::new();
    for i in 0..(hex_string.len() / 2){
        let pos = i * 2;
        let byte_string = hex_string.get(pos..(pos + 2)).expect("Could not extract partial string for parsing");
        let byte : u8 = u8::from_str_radix(byte_string, 16).expect("Could not parse byte");
        bytes.push (byte);
    }
    bytes
}

#[test]
fn test_parse_hex_string(){
    let hex_string = String::from("02AB1D00");
    let bytes : Vec<u8> = parse_hex_string(&hex_string);
    assert_eq!(bytes.len(), 4);
    assert_eq!(bytes[0], 0x02);
    assert_eq!(bytes[1], 0xAB);
    assert_eq!(bytes[2], 0x1D);
    assert_eq!(bytes[3], 0x00);
}

fn main() {
    let path : String = String::from("zeKnirf.txt");
    let lines : Vec<String> = read_file(&path);
    let mut bytes : Vec<u8> = Vec::new();
    for mut line in lines {
        bytes.append(&mut parse_hex_string(&mut line));
    }
    let mut bytes_left : usize = bytes.len();
    let mut address : usize = 0x0000;
    while bytes_left > 0 {
       let mut size_to_write : usize = 0x10;
       if bytes_left < 0x10 {
           size_to_write = bytes_left; 
       }
       let mut bytes_to_write : Vec <u8> = Vec::new();
       for i in 0..size_to_write {
           bytes_to_write.push (bytes[address + i]);
       }
       let address_lower = (address & 0xFFFF) as u16;
       println!("{}", convert_to_line(&bytes_to_write, address_lower, 0x00));
       address += size_to_write;
       bytes_left -= size_to_write;
       //TODO: Continue here
    }
}
