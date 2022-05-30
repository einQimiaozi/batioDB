use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use crate::entry::{Entry, ENTRY_HEADER_SIZE};
use crate::operations::{INSERT, QUERY};

/*
    DBfile is a separate DB file.
     Offset records the last write position of the current DB.
 */

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

    // When reading, scan forward from the end of the file.
    // See the notes in the entry.rs for the reason.
    pub fn read(&mut self, mut offset: u64) -> Entry {
        offset -= ENTRY_HEADER_SIZE as u64;
        self.file.seek(SeekFrom::Start(offset)).expect("error: offset invalid!");
        let mut buffer = [0u8; ENTRY_HEADER_SIZE];
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