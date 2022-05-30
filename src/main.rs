use crate::config::DBConfig;
use crate::db::DB;
use crate::parse::Parse;
use crate::service::*;

mod entry;
mod operations;
mod db;
mod db_file;
mod utils;
mod config;
mod service;
mod parse;

#[tokio::main]
async fn main() {
    println!("baby turn it on!");
    let config = DBConfig::new("config.yaml");
    // let's go!!!
    let mut db = DB::new(config.db_path.clone());
    println!("bind: {}",config.port);
    println!("db in: {}",config.db_path.clone());
    DBService::listen(db,config.port,config.channel_cap).await;
}
