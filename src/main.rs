use serde::{Serialize,Deserialize };
use std::fs;
use std::path::Path;
use std::time;

#[derive(Serialize, Deserialize)]
struct Config {
    ignore_file: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Database {
    time: u64,
    data: Vec<FileInfo>,
}

#[derive(Serialize, Deserialize)]
struct FileInfo {
    path: String,
    time: u64,
}

const DEFAULT_MAJOR: u8 = 0;
const DEFAULT_MINOR: u8 = 1;
const DEFAULT_PATCH: u8 = 0;
const CONFIG_FILE_NAME:&str = "config.toml";
const DATA_FILE_NAME:&str = "db.json";

// const SUPPORT_EXT:Vec<&str>= vec!["doc"];
fn main() {
    let mut config = Config {
        ignore_file: vec![]
    };
    let mut db = Database {
        time: time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        data: Vec::new(),
    };
    let db_path = Path::new(DATA_FILE_NAME);
    if db_path.exists() {// DEBUG
        fs::remove_file(DATA_FILE_NAME).unwrap();
    }
    if db_path.exists() {
        // 比对数据库
    } else {
        // 生成数据库
        let path = Path::new(".");
        init(path, &mut db);
        let json = serde_json::to_string(&db).unwrap();
        fs::write(DATA_FILE_NAME, json).unwrap();
    };

    // let value = "foo = 'bar'".parse::<Value>().unwrap();
    // println!("{:?}", value["foo"].as_str())
    // let path = Path::new(".");
    // search_path(path);
}
fn init(root: &Path, db: &mut Database) {
    let root_path = Path::new(root);
    for each in fs::read_dir(root_path).unwrap() {
        let dir = each.unwrap();
        let pb = dir.path();
        if pb.is_dir() {
            init(&pb, db)
        } else {
            let metadata = fs::metadata(&pb);
            db.data.push(FileInfo {
                path: pb.to_str().unwrap().to_string(),
                time: metadata
                    .unwrap()
                    .modified()
                    .unwrap()
                    .duration_since(time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            });
        }
    }
}
fn search_path(path: &Path) {
    for each in fs::read_dir(path).unwrap() {
        let dir = each.unwrap();
        let pb = dir.path();
        if pb.is_dir() {
            search_path(&pb)
        } else {
            println!("{:?}", pb.extension());
        }
    }
}
