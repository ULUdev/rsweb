use crate::http::MimeType;
use std::collections::HashMap;
use std::fs::{metadata, OpenOptions};
use std::io::Read;
use std::path::Path;
use std::time::SystemTime;

/// a resource loader and cacher
#[derive(Clone)]
pub struct ResourceLoader {
    resource_cache: HashMap<String, Resource>,
    resource_root: String,
    use_cache: bool,
}

/// a resource loaded by the resource loader
#[derive(Clone)]
pub struct Resource {
    content: String,
    path: String,
    accessed: SystemTime,
    mime_type: MimeType,
}

impl Resource {
    /// load a new resource
    pub fn load(path: String) -> std::io::Result<Resource> {
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

        Ok(Resource {
            content,
            path,
            accessed,
            mime_type,
        })
    }

    /// get the content of a resource
    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    /// get the time when the file was accessed
    pub fn get_accessed(&self) -> SystemTime {
        self.accessed.clone()
    }

    /// get the path where the resource is located
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    /// get the mime type of the resource
    pub fn get_mime(&self) -> MimeType {
        self.mime_type.clone()
    }
}

impl ResourceLoader {
    /// create a new ResourceLoader with a specified capacity and resource root
    pub fn new(capacity: usize, root: String, use_cache: bool) -> ResourceLoader {
        ResourceLoader {
            resource_cache: HashMap::with_capacity(capacity),
            resource_root: root,
            use_cache,
        }
    }

    /// load a resource from cache or file system
    /// # Arguments
    /// `path`: the path relative to the resource root to look for resources
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
            if let Some(n) = self.resource_cache.clone().get(&path) {
                if let Ok(md) = metadata(path.as_str()) {
                    if let Ok(time) = md.modified() {
                        if let Ok(elapsed) = time.elapsed() {
                            if let Ok(res_elapsed) = n.get_accessed().elapsed() {
                                if elapsed < res_elapsed {
                                    let new = match Resource::load(path.clone()) {
                                        Ok(n) => n,
                                        Err(_) => {
                                            return (String::new(), MimeType::Other(String::new()));
                                        }
                                    };
                                    let _ = self.resource_cache.insert(path, new);
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

    /// load a resource into cache
    /// # Arguments
    /// `path`: the path relative to the resource root to look for resources
    pub fn preload(&mut self, path: String) -> String {
        if !self.use_cache {
            return String::new();
        }
        let p = Path::new(path.as_str());
        //let p = p.join(Path::new(self.resource_root.as_str()));
        let p = Path::new(self.resource_root.as_str()).join(p);
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
