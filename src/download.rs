use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct Downloader {
    rules_url: String,
    list_url: String,
    cache_dir: PathBuf,
}

impl Downloader {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            rules_url: "https://raw.githubusercontent.com/espeak-ng/espeak-ng/59eb19938f12e30881c81d86ce4a7de25414c9f4/dictsource/en_rules".to_string(),
            list_url: "https://raw.githubusercontent.com/espeak-ng/espeak-ng/59eb19938f12e30881c81d86ce4a7de25414c9f4/dictsource/en_list".to_string(),
            cache_dir,
        }
    }

    #[allow(dead_code)]
    pub fn with_custom_urls(cache_dir: PathBuf, rules_url: &str, list_url: &str) -> Self {
        Self {
            rules_url: rules_url.to_string(),
            list_url: list_url.to_string(),
            cache_dir,
        }
    }

    pub fn download_if_needed(&self) -> Result<(PathBuf, PathBuf), Box<dyn std::error::Error>> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)
                .map_err(|e| format!("Failed to create cache directory: {}", e))?;
        }

        let rules_path = self.cache_dir.join("en_rules");
        let list_path = self.cache_dir.join("en_list");

        let rules_exists = rules_path.exists();
        let list_exists = list_path.exists();

        if rules_exists && list_exists {
            return Ok((rules_path, list_path));
        }

        if !rules_exists {
            println!("Downloading en_rules...");
            self.download_file(&self.rules_url, &rules_path)?;
            println!("Downloaded en_rules to {:?}", rules_path);
        }

        if !list_exists {
            println!("Downloading en_list...");
            self.download_file(&self.list_url, &list_path)?;
            println!("Downloaded en_list to {:?}", list_path);
        }

        Ok((rules_path, list_path))
    }

    pub fn download_file(
        &self,
        url: &str,
        path: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = ureq::get(url).call()?;
        let mut file = fs::File::create(path)?;

        let mut reader = response.into_reader();
        let mut buffer = vec![0; 8192];

        loop {
            let n = std::io::Read::read(&mut reader, &mut buffer)?;
            if n == 0 {
                break;
            }
            file.write_all(&buffer[..n])?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_rules_path(&self) -> PathBuf {
        self.cache_dir.join("en_rules")
    }

    #[allow(dead_code)]
    pub fn get_list_path(&self) -> PathBuf {
        self.cache_dir.join("en_list")
    }
}
