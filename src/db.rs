use std::collections::HashMap;
use crate::db_file::DBFile;
use crate::entry::{Entry, ENTRYHEADERSIZE};
use crate::operations;
use crate::operations::{DELETE, INSERT};

pub struct DB<'path> {
    db_index: HashMap<String,u64>,
    db_file: DBFile,
    dir_path: &'path str,
}

impl<'path> DB<'path> {
    pub fn new(path: &'path str) -> Self {
        let mut db_file = DBFile::new(path.to_string()).expect("db file loaded error");
        let mut db_index: HashMap<String,u64> = DB::scan_index(&mut db_file);
        DB {
            db_index: db_index,
            db_file: db_file,
            dir_path: path,
        }
    }
    fn scan_index(db_file: &mut DBFile) -> HashMap<String,u64> {
        let mut offset: u64 = 0;
        let mut db_index: HashMap<String,u64> = HashMap::new();
        while offset < db_file.offset {
            println!("{}",offset);
            let entry = db_file.read(offset);
            println!("{:?}",entry);
            db_index.insert(entry.key,offset);
            offset = offset + entry.key_size as u64 + entry.value_size as u64 + ENTRYHEADERSIZE as u64;
        }
        println!("end");
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
        self.db_index.insert(key, self.db_file.offset);
        self.db_file.offset += write_len;
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
        let mut entry = Entry::new(&key, "nil", DELETE);
        let write_len = self.db_file.write(&mut entry);
        self.db_index.remove(&key);
        self.db_file.offset += write_len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // test read (after write)
    fn test_r() {
        let mut db = DB::new("test.data");
        let name = match db.get("name".to_string()) {
            Some(T) => T,
            None => "nil".to_string(),
        };
        let name1 = match db.get("name1".to_string()) {
            Some(T) => T,
            None => "nil".to_string(),
        };
        let age = match db.get("age".to_string()) {
            Some(T) => T,
            None => "nil".to_string(),
        };
        let nil = match db.get("name2".to_string()) {
            Some(T) => T,
            None => "nil".to_string(),
        };
        assert_eq!("Qimiaozi123",name1);
        assert_eq!("18",age);
        assert_eq!("Qimiaozi",name);
        assert_eq!("nil",nil);
    }

    // test read a entry(after write)
    #[test]
    fn test_r_entry() {
        let mut db = DB::new("test.data");
        let mut name = db.get_entry("name".to_string()).unwrap();
        assert_eq!(name.get_op_type(),INSERT);
    }

    #[test]
    // test write then read
    fn test_rw() {
        let mut db = DB::new("test.data");
        db.put("name1".to_string(),"Qimiaozi123".to_string());
        db.put("name".to_string(),"Qimiaozi".to_string());
        db.put("age".to_string(),"18".to_string());
        let name = db.get("name".to_string()).unwrap();
        let name1 = db.get("name1".to_string()).unwrap();
        let age = db.get("age".to_string()).unwrap();
        assert_eq!("Qimiaozi123",name1);
        assert_eq!("18",age);
        assert_eq!("Qimiaozi",name);
    }
}