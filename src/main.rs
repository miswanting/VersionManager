use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ignore_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    time: u64,
    data: Vec<FileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    path: String,
    time: u64,
}

const DEFAULT_MAJOR: u8 = 0;
const DEFAULT_MINOR: u8 = 1;
const DEFAULT_PATCH: u8 = 0;
const CONFIG_FILE_NAME: &str = "config.yaml";
const DATABASE_FILE_NAME: &str = "database.json";

fn main() {
    let mut cf = Config {
        ignore_file: "123".to_string(),
    };
    let mut db = Database {
        time: time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        data: Vec::new(),
    };
    let cf_path = Path::new(CONFIG_FILE_NAME);
    let db_path = Path::new(DATABASE_FILE_NAME);
    if cf_path.exists() {
        // DEBUG
        fs::remove_file(CONFIG_FILE_NAME).unwrap();
    }
    if db_path.exists() {
        // DEBUG
        fs::remove_file(DATABASE_FILE_NAME).unwrap();
    }
    if cf_path.exists() {
        // 读取配置
        let f = fs::read_to_string(CONFIG_FILE_NAME).unwrap();
        cf = serde_yaml::from_str(&f).unwrap();
    } else {
        // 生成默认配置
        let yaml = serde_yaml::to_string(&cf).unwrap();
        fs::write(CONFIG_FILE_NAME, yaml).unwrap();
    };
    if db_path.exists() {
        // 比对数据库
    } else {
        // 生成数据库
        let path = Path::new(".");
        init_scan(path, &mut db);
        let json = serde_json::to_string(&db).unwrap();
        fs::write(DATABASE_FILE_NAME, json).unwrap();
    };
}
fn init_scan(path: &Path, db: &mut Database) {
    for each in fs::read_dir(path).unwrap() {
        let d = each.unwrap();
        let p = d.path();
        if p.is_dir() {
            init_scan(&p, db)
        } else {
            let metadata = fs::metadata(&p);
            db.data.push(FileInfo {
                path: p.to_str().unwrap().to_string(),
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
// fn init(root: &Path, db: &mut Database) {
//     let root_path = Path::new(root);
//     for each in fs::read_dir(root_path).unwrap() {
//         let dir = each.unwrap();
//         let pb = dir.path();
//         if pb.is_dir() {
//             init(&pb, db)
//         } else {
//             let metadata = fs::metadata(&pb);
//             db.data.push(FileInfo {
//                 path: pb.to_str().unwrap().to_string(),
//                 time: metadata
//                     .unwrap()
//                     .modified()
//                     .unwrap()
//                     .duration_since(time::UNIX_EPOCH)
//                     .unwrap()
//                     .as_secs(),
//             });
//         }
//     }
// }
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
fn file2data() {}
fn data2file() {}
