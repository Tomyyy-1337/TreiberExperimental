use crate::global::Global;

pub static PLUGINS: Global<PluginList> = Global::new(
    PluginList {
        paths: Vec::new(),
    }
);

pub struct PluginList {
    pub paths: Vec<String>
}

impl PluginList {
    pub fn load(&mut self) {
        self.paths.clear();
        match std::fs::read_dir("plugins") {
            Ok(entry) => {
                for entry in entry {
                    if let Ok(entry) = entry {
                        let path = format!("/plugins/{}", entry.file_name().to_string_lossy());
                        self.paths.push(path);
                    }
                }
            },
            Err(_) => println!("Failed to read plugins directory"),
        }
    }
}