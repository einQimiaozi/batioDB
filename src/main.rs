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
    // let's go!!!
    let mut db = DB::new("test.data");
    DBService::listen(db,"127.0.0.1:8765",1024).await;
}
