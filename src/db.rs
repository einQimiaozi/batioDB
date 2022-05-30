use std::collections::{BTreeMap, HashSet};
use crate::db_file::DBFile;
use crate::entry::{Entry, ENTRY_HEADER_SIZE};
use crate::operations::{DELETE, INSERT, QUERY};

/*
    DB is a lockless database, and all requests are processed through a single thread.
    Concurrency security is controlled by db_service.
    db_index is a B-tree structure that stores the mapping of each key to the offset of the data line in its file.
 */
pub struct DB<'path> {
    db_index: BTreeMap<String,u64>,
    db_file: DBFile,
    file_path: &'path str,
}

impl<'path> DB<'path> {
    // When starting dB, DB will scan disk data and create db_index.
    pub fn new(path: &'path str) -> Self {
        let mut db_file = DBFile::new(path.to_string()).expect("db file loaded error");
        let mut db_index: BTreeMap<String,u64> = DB::scan_index(&mut db_file);
        DB {
            db_index: db_index,
            db_file: db_file,
            file_path: path,
        }
    }

    fn scan_index(db_file: &mut DBFile) -> BTreeMap<String,u64> {
        let mut db_index: BTreeMap<String,u64> = BTreeMap::new();
        let mut offset = db_file.offset;
        let mut delete_list: HashSet<String> = HashSet::new();
        while offset > 0 {
            let mut entry = db_file.read(offset);
            if entry.get_op_type() == DELETE {
                delete_list.insert(entry.key);
                offset = offset - entry.key_size as u64 - entry.value_size as u64 - ENTRY_HEADER_SIZE as u64;
                continue;
            }
            if delete_list.contains(&entry.key) {
                offset = offset - entry.key_size as u64 - entry.value_size as u64 - ENTRY_HEADER_SIZE as u64;
                continue;
            }
            db_index.entry(entry.key).or_insert(offset);
            offset = offset - entry.key_size as u64 - entry.value_size as u64 - ENTRY_HEADER_SIZE as u64;
        }
        db_index
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        let mut offset: u64;
        match self.db_index.get(&key) {
            Some(T) => offset = *T,
            None => return None
        };
        let entry = self.db_file.read(offset);
        Some(entry.value)
    }

    pub fn put(&mut self, key: String, value: String) {
        let mut entry = Entry::new(&key, &value, INSERT);
        let write_len = self.db_file.write(&mut entry);
        self.db_file.offset += write_len;
        self.db_index.insert(key, self.db_file.offset);
    }

    pub fn get_entry(&mut self, key: String) -> Option<Entry> {
        let mut offset: u64;
        match self.db_index.get(&key) {
            Some(T) => offset = *T,
            None => return None
        };
        Some(self.db_file.read(offset))
    }

    pub fn remove(&mut self, key: String) {
        if !self.db_index.contains_key(&key) {
            return;
        }
        let mut entry = Entry::new(&key, "nil", DELETE);
        let write_len = self.db_file.write(&mut entry);
        self.db_index.remove(&key);
        self.db_file.offset += write_len;
    }

    /*
        After the garbage collector is called, it will scan the entire disk,
        delete the old version data of each key, and only keep the latest version data. Each time,
        it takes a long time. It is recommended to use a separate spawn for execution.
     */
    pub fn garbage_collection(&mut self) {
        let mut new_db_file = DBFile::new("gc_tmp_file.data".to_string()).unwrap();
        let mut new_offset: u64 = 0;
        let mut new_index: BTreeMap<String,u64> = self.db_index.clone();

        for (key,_) in new_index {
            let mut entry = self.get_entry(key.to_string()).unwrap();
            let write_len = new_db_file.write(&mut entry);
            new_offset += write_len;
            self.db_index.insert(key.to_string(), new_offset);
        }
        std::fs::remove_file(self.file_path);
        std::fs::rename("gc_tmp_file.data",self.file_path)
            .expect("gc failed,you may need to manually rename the gc_tmp_file.data file to the your db file");
        std::fs::remove_file("gc_tmp_file.data".to_string());
    }
}