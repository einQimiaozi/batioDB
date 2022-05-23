use crate::config::DBConfig;
use crate::db::DB;

mod entry;
mod operations;
mod db;
mod db_file;
mod utils;
mod config;

fn main() {
    println!("baby trun it on!");
    // test
    let db_config = DBConfig::default();
    let mut db = DB::new(&db_config.db_path);

    db.put("name1".to_string(),"xiaoxixi".to_string());
    db.put("name1".to_string(),"xiaoxixi123".to_string());
    db.garbage_collection();
    let name1 = match db.get("name1".to_string()) {
        Some(T) => T,
        None => "nil".to_string(),
    };
    let name = db.get("name".to_string()).unwrap();
    let age = db.get("age".to_string()).unwrap();
    println!("{} {} {}",name,name1,age);
}
