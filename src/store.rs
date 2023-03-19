use std::{path::PathBuf, env, fs};

pub struct Store {
  store_dir: PathBuf
}

#[allow(dead_code)]
impl Store {
  pub fn new(store_name: &str) -> Store {
    let mut cwd = PathBuf::new();
    let name = store_name.clone();

    if let Ok(dir) = env::current_dir() {
      cwd = dir;
    }

    Store {
      store_dir: cwd
        .join("__rusty_store__")
        .join(hex::encode(name))
    }
  }

  pub fn has(&self, key: &str) -> bool {
    self.store_dir.join(hex::encode(key)).exists()
  }

  pub fn get(&self, key: &str) -> Option<String> {
    let file_path = self.store_dir.join(hex::encode(key));

    if file_path.exists() {
      if let Ok(content) = fs::read_to_string(file_path) {
        return Some(content);
      }
    }

    None
  }

  pub fn set(&self, key: &str, value: &str) -> bool {
    if let Ok(_) = fs::create_dir_all(self.store_dir.clone()) {
      if let Ok(_) = fs::write(self.store_dir.join(hex::encode(key)), value) {
        return true;
      }
    }

    false
  }

  pub fn remove(&self, key: &str) -> bool {
    let file_path = self.store_dir.join(hex::encode(key));

    if file_path.exists() {
      if let Ok(_) = fs::remove_file(file_path) {
        return true;
      }
    }

    false
  }

}

/*
if let Ok(dir) = fs::read_dir(self.store_dir.clone()) {
  for each in dir {
    if let Ok(file) = each {
      if let Ok(file_type) = file.file_type() {
        if file_type.is_file() {
          if let Some(file_name) = file.file_name().to_str() {

          }
        }
      }
    }
  }
}
 */