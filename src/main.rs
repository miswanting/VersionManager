use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time;
extern crate regex;
use regex::Regex;
// 配置文件结构
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    supported_extension: Vec<String>,
}

// 数据库结构
#[derive(Debug, Serialize, Deserialize)]
struct Database {
    time: u64,
    data: Vec<FileInfo>,
}

// 文件信息结构
#[derive(Debug, Serialize, Deserialize)]
struct FileInfo {
    path: String,
    time: u64,
}

// 默认参数
const DEFAULT_MAJOR: u8 = 0;
const DEFAULT_MINOR: u8 = 1;
const DEFAULT_PATCH: u8 = 0;
const CONFIG_FILE_NAME: &str = "config.yaml";
const DATABASE_FILE_NAME: &str = "database.json";
const RE_INIT: &str = r"^(.*?)-init$";

fn main() {
    // 数据初始化
    let mut cf = Config {
        supported_extension: vec![
            "doc".to_string(),
            "docx".to_string(),
            "ppt".to_string(),
            "pptx".to_string(),
            "xls".to_string(),
            "xlsx".to_string(),
        ],
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
    // 调试功能
    if cf_path.exists() {
        // DEBUG
        fs::remove_file(CONFIG_FILE_NAME).unwrap();
    }
    if db_path.exists() {
        // DEBUG
        fs::remove_file(DATABASE_FILE_NAME).unwrap();
    }
    // 处理配置文件
    if cf_path.exists() {
        // 读取配置
        let f = fs::read_to_string(CONFIG_FILE_NAME).unwrap();
        cf = serde_yaml::from_str(&f).unwrap();
    } else {
        // 生成默认配置
        let yaml = serde_yaml::to_string(&cf).unwrap();
        fs::write(CONFIG_FILE_NAME, yaml).unwrap();
    };
    // 处理数据库
    if db_path.exists() {
        // 比对数据库
    } else {
        // 生成数据库
        let path = Path::new(".");
        init_scan(path, &cf, &mut db);
        let json = serde_json::to_string(&db).unwrap();
        fs::write(DATABASE_FILE_NAME, json).unwrap();
    };
}
fn init_scan(path: &Path, cf: &Config, db: &mut Database) {
    for each in fs::read_dir(path).unwrap() {
        let d = each.unwrap();
        let metadata = d.metadata();
        let p = d.path();
        if p.is_dir() {
            // 处理文件夹
            init_scan(&p, cf, db)
        } else {
            // 处理文件
            let ext_result = p.extension();
            if ext_result.is_some() {
                let ext = ext_result.unwrap().to_str().unwrap();
                let mut is_supported_type: bool = false;
                let l = cf.supported_extension.clone();
                for i in l {
                    if i == ext.to_string() {
                        is_supported_type = true;
                    }
                }
                if is_supported_type {
                    let file_stem = p.file_stem().unwrap().to_str().unwrap();
                    let r_init = Regex::new(RE_INIT).unwrap();
                    if r_init.is_match(file_stem) {
                        // 检测到初始化文件标记
                        let caps = r_init.captures(file_stem).unwrap();
                        let name = caps.get(1).unwrap().as_str().to_string();
                        let mut new_path = p.to_path_buf();
                        let new_file_name =
                            format!("{} v{}.{}.{}", name, DEFAULT_MAJOR, DEFAULT_MINOR, ext);
                        // let new_file_name = name + " v0.1" + ext.to_str().unwrap();
                        new_path.set_file_name(new_file_name);
                        // let new_path = p.to_string_lossy();
                        // let new_path = new_path + " v1.0.1." + ext.to_str().unwrap();
                        // println!("{:?}", new_path);
                        // let new_path=new_path;
                        fs::rename(p.clone(), new_path).unwrap();
                    }
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
    }
}
fn get_current_time_stamp() {}
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
