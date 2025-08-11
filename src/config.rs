use std::path::PathBuf;

pub struct Config {
    pub site_name: Option<String>,
    pub base_url: String,
    pub verbose: bool,
    pub include_drafts: bool,
    pub ignore: Option<String>,
    pub authors: Option<Vec<SiteAuthor>>,
    pub paths: ConfigPaths,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            site_name: Some("Example site".into()),
            base_url: "https://www.example.com".into(),
            verbose: true,
            include_drafts: false,
            ignore: Some(".karkaignore".into()),
            authors: Some(vec![SiteAuthor::default()]),
            paths: ConfigPaths::default(),
        }
    }
}

pub struct SiteAuthor {
    pub name: Option<String>,
    pub email: Option<String>,
    pub fediverse: Option<String>,
}

impl Default for SiteAuthor {
    fn default() -> Self {
        Self {
            name: Some("Your name here".into()),
            email: Some("name@example.com".into()),
            fediverse: Some("name@example.social".into()),
        }
    }
}

pub struct ConfigPaths {
    pub assets: Option<PathBuf>,
    pub data: Option<PathBuf>,
    pub schemas: Option<PathBuf>,
    pub templates: PathBuf,
    pub pages: PathBuf,
    pub content: Option<PathBuf>,
    pub output: PathBuf,
}

impl Default for ConfigPaths {
    fn default() -> Self {
        ConfigPaths {
            assets: Some("_assets".into()),
            data: Some("_data".into()),
            schemas: Some("_schemas".into()),
            templates: "_templates".into(),
            pages: "pages".into(),
            content: Some("content".into()),
            output: "dist".into(),
        }
    }
}
