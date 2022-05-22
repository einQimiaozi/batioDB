use crate::{operations,utils};
use std::mem;
use crc_any::CRC;
use crate::utils::{u32_to_vec_u8, vec_u8_to_u32};

pub const ENTRYHEADERSIZE: usize = 12;
pub const STATUSBITS: usize = 4;

#[derive(Debug)]
pub struct Entry {
    pub status: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub key: String,
    pub value: String,
}

impl Entry {
    pub fn new(key: &str,value: &str,op_type: u8) -> Self {
        let mut crc_num = CRC::crc24();
        crc_num.digest((key.to_string()+value).as_bytes());
        let mut status_bytes = crc_num.get_crc_vec_be();
        status_bytes.push(op_type);
        let status = unsafe { vec_u8_to_u32(status_bytes) };
        Entry {
            status: status,
            key_size: key.len() as u32,
            value_size: value.len() as u32,
            key: key.to_string(),
            value: value.to_string(),
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        let bytes_key = self.key.clone().into_bytes();
        let bytes_value = self.value.clone().into_bytes();
        let key_size_u8 = unsafe {
            u32_to_vec_u8(self.key_size, 4)
        };
        let value_size_u8 = unsafe {
            u32_to_vec_u8(self.value_size, 4)
        };
        let status_u8 = unsafe {
            u32_to_vec_u8(self.status,4)
        };
        [key_size_u8,value_size_u8,status_u8,bytes_key,bytes_value].concat()
    }
    pub fn decode(&mut self,buffer: Vec<u8>) {
        let key:String = String::from_utf8_lossy(&buffer[ .. (self.key_size) as usize]).to_string();
        let value: String = String::from_utf8_lossy(&buffer[(self.key_size) as usize .. (self.value_size+self.key_size) as usize]).to_string();

        self.key = key;
        self.value = value;
    }
    pub fn get_header(&mut self,header: Vec<u8>) {
        let key_size_u8 = header[..4].to_vec();
        let value_size_u8 = header[4..8].to_vec();
        let status = unsafe { vec_u8_to_u32(header[8..ENTRYHEADERSIZE].to_vec()) };
        let key_size:u32 = unsafe {
            vec_u8_to_u32(key_size_u8)
        };
        let value_size:u32 = unsafe {
            vec_u8_to_u32(value_size_u8)
        };
        self.key_size = key_size;
        self.value_size = value_size;
        self.status = status;
    }
    pub fn get_op_type(&mut self) -> u8 {
        let mut status_bytes = unsafe {
            u32_to_vec_u8(self.status,STATUSBITS)
        };
        status_bytes[STATUSBITS-1]
    }
    pub fn set_op_type(&mut self, op_type: u8) {
        let mut status_bytes = unsafe {
            u32_to_vec_u8(self.status,STATUSBITS)
        };
        status_bytes[STATUSBITS-1] = op_type;
        self.status = unsafe {
            vec_u8_to_u32(status_bytes)
        };
    }
}