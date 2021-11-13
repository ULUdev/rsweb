use std::collections::HashMap;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;

#[derive(Clone)]
pub struct RessourceLoader {
    ressource_cache: HashMap<String, String>,
    ressource_root: String,
}

impl RessourceLoader {

    /// create a new RessourceLoader with a specified capacity and ressource root
    pub fn new(capacity: usize, root: String) -> RessourceLoader {
        RessourceLoader { ressource_cache: HashMap::with_capacity(capacity), ressource_root: root }
    }

    /// load a resource from cache or file system
    /// # Arguments
    /// `path`: the path relative to the ressource root to look for ressources
    pub fn load(&mut self, path: String) -> String {
        match self.ressource_cache.get(&path) {
            Some(n) => n.to_string(),
            None => {
                let p = Path::new(path.as_str());
                //let p = p.join(Path::new(self.ressource_root.as_str()));
                //let p = Path::new(self.ressource_root.as_str()).join(p);
                //println!("path of ressource: {}", p.as_path().to_str().unwrap());
                println!("path of ressource: {}", p.to_str().unwrap());
                if !p.exists() {
                    return String::new();
                } else {
                    let mut f = match OpenOptions::new().read(true).open(p.to_str().unwrap()) {
                        Ok(n) => n,
                        Err(_) => {
                            return String::new();
                        }
                    };
                    let mut buf: String = String::new();
                    match f.read_to_string(&mut buf) {
                        Ok(_) => {
                            return buf;
                        }
                        Err(_) => {
                            return String::new();
                        }
                    }
                }
            }
        }
    }

    /// load a ressource into cache
    /// # Arguments
    /// `path`: the path relative to the ressource root to look for ressources
    pub fn preload(&mut self, path: String) -> String {
        let p = Path::new(path.as_str());
        //let p = p.join(Path::new(self.ressource_root.as_str()));
        let p = Path::new(self.ressource_root.as_str()).join(p);
        if !p.exists() {
            return String::new();
        }
        let mut f = match OpenOptions::new().read(true).open(p.to_str().unwrap()) {
            Ok(n) => n,
            Err(_) => {
                return String::new();
            }
        };
        let mut buf: String = String::new();
        match f.read_to_string(&mut buf) {
            Ok(_) => {
                return buf;
            }
            Err(_) => {
                return String::new();
            }
        }
    }
}
