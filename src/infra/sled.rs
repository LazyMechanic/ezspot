use std::env;
use uuid::Uuid;

pub fn new_sled_db() -> sled::Result<sled::Db> {
    let tmp_file_path = env::temp_dir().join(format!("ezspot-{}", Uuid::new_v4()));
    sled::Config::default()
        .temporary(true)
        .path(tmp_file_path)
        .open()
}
