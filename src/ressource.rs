use crate::http::MimeType;
use std::collections::HashMap;
use std::fs::{metadata, OpenOptions};
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;

/// a ressource loader and cacher
#[derive(Clone)]
pub struct RessourceLoader {
    ressource_cache: HashMap<String, Ressource>,
    ressource_root: String,
    use_cache: bool,
}

/// a ressource loaded by the ressource loader
#[derive(Clone)]
pub struct Ressource {
    content: String,
    path: String,
    accessed: SystemTime,
    mime_type: MimeType,
}

impl Ressource {
    /// load a new ressource
    pub fn load(path: String) -> std::io::Result<Ressource> {
        let mime_type: MimeType = match Path::new(&path).extension() {
            Some(n) => match n.to_str().unwrap_or("") {
                "html" | "htm" => MimeType::Html,
                "js" => MimeType::Javascript,
                "css" => MimeType::Css,
                "jpeg" | "jpg" => MimeType::Jpeg,
                "png" => MimeType::Png,
                "pdf" => MimeType::Pdf,
                "txt" => MimeType::Plaintext,
                _ => MimeType::Plaintext,
            },
            None => MimeType::Plaintext,
        };
        let mut file = match OpenOptions::new().read(true).open(path.clone()) {
            Ok(n) => n,
            Err(e) => {
                return Err(e);
            }
        };
        let mut content: String = String::new();
        if let Err(e) = file.read_to_string(&mut content) {
            return Err(e);
        }
        let accessed = SystemTime::now();

        Ok(Ressource {
            content,
            path,
            accessed,
            mime_type,
        })
    }

    /// get the content of a ressource
    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    /// get the time when the file was accessed
    pub fn get_accessed(&self) -> SystemTime {
        self.accessed.clone()
    }

    /// get the path where the ressource is located
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// get the mime type of the ressource
    pub fn get_mime(&self) -> MimeType {
        self.mime_type.clone()
    }
}

impl RessourceLoader {
    /// create a new RessourceLoader with a specified capacity and ressource root
    pub fn new(capacity: usize, root: String, use_cache: bool) -> RessourceLoader {
        RessourceLoader {
            ressource_cache: HashMap::with_capacity(capacity),
            ressource_root: root,
            use_cache,
        }
    }

    /// load a resource from cache or file system
    /// # Arguments
    /// `path`: the path relative to the ressource root to look for ressources
    pub fn load(&mut self, path: String) -> (String, MimeType) {
        let mime_type: MimeType = match Path::new(&path).extension() {
            Some(n) => match n.to_str().unwrap_or("") {
                "html" | "htm" => MimeType::Html,
                "js" => MimeType::Javascript,
                "css" => MimeType::Css,
                "jpeg" | "jpg" => MimeType::Jpeg,
                "png" => MimeType::Png,
                "pdf" => MimeType::Pdf,
                "txt" => MimeType::Plaintext,
                _ => MimeType::Plaintext,
            },
            None => MimeType::Plaintext,
        };
        if self.use_cache {
            if let Some(n) = self.ressource_cache.clone().get(&path) {
                if let Ok(md) = metadata(path.as_str()) {
                    if let Ok(time) = md.modified() {
                        if let Ok(elapsed) = time.elapsed() {
                            if let Ok(res_elapsed) = n.get_accessed().elapsed() {
                                if elapsed < res_elapsed {
                                    let new = match Ressource::load(path.clone()) {
                                        Ok(n) => n,
                                        Err(_) => {
                                            return (String::new(), MimeType::Other(String::new()));
                                        }
                                    };
                                    let _ = self.ressource_cache.insert(path, new);
                                }
                            }
                        }
                    }
                }
                return (n.get_content(), mime_type);
            }
        }
        let p = Path::new(path.as_str());
        if !p.exists() {
            (String::new(), MimeType::Plaintext)
        } else {
            let mut f = match OpenOptions::new().read(true).open(p.to_str().unwrap()) {
                Ok(n) => n,
                Err(_) => {
                    return (String::new(), MimeType::Plaintext);
                }
            };
            let mut buf: String = String::new();
            match f.read_to_string(&mut buf) {
                Ok(_) => (buf, mime_type),
                Err(_) => (String::new(), MimeType::Plaintext),
            }
        }
    }

    /// load a ressource into cache
    /// # Arguments
    /// `path`: the path relative to the ressource root to look for ressources
    pub fn preload(&mut self, path: String) -> String {
        if !self.use_cache {
            return String::new();
        }
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
            Ok(_) => buf,
            Err(_) => String::new(),
        }
    }
}
