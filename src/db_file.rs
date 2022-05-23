use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use crate::entry::{Entry, ENTRYHEADERSIZE};
use crate::operations;
use crate::operations::{INSERT, QUERY};

pub struct DBFile {
    file: File,
    pub offset: u64,
}

impl DBFile {
    pub fn new(path: String) -> Result<Self,&'static str> {
        let db_file = OpenOptions::new().create(true).append(true).read(true).open(path).expect("DBFile Error");
        let offset = db_file.metadata().unwrap().len();
        Ok(DBFile{
            file: db_file,
            offset: offset,
        })
    }
    pub fn read(&mut self, mut offset: u64) -> Entry {
        offset -= ENTRYHEADERSIZE as u64;
        self.file.seek(SeekFrom::Start(offset)).expect("error: offset invalid!");
        let mut buffer = [0u8; ENTRYHEADERSIZE];
        self.file.read(&mut buffer);
        let header = buffer.to_vec();
        let mut entry = Entry::new("nil","nil",0);
        entry.get_header(header);

        self.file.seek(SeekFrom::Start(offset - entry.key_size as u64 - entry.value_size as u64)).expect("error: offset invalid!");

        let mut buffer = Vec::new();
        buffer.resize((entry.key_size + entry.value_size) as usize,0u8);
        self.file.read(&mut buffer);

        entry.decode(buffer);
        entry
    }
    pub fn write(&mut self, entry: &mut Entry) -> u64 {
        entry.set_op_type(INSERT);
        self.file.write(&(entry.encode())).unwrap() as u64
    }
}