pub struct Config {
    site_name: Option<String>,
    base_url: String,
    verbose: bool,
    include_drafts: bool,
    ignore: Option<String>,
    authors: Option<Vec<SiteAuthor>>,
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
        }
    }
}

struct SiteAuthor {
    name: Option<String>,
    email: Option<String>,
    fediverse: Option<String>,
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

struct ConfigDirs {
    assets: Option<String>,
    data: Option<String>,
    schemas: Option<String>,
    templates: Option<String>,
    pages: Option<String>,
    content: Option<String>,
    output: Option<String>,
}

impl Default for ConfigDirs {
    fn default() -> Self {
        ConfigDirs {
            assets: Some("_assets".into()),
            data: Some("_data".into()),
            schemas: Some("_schemas".into()),
            templates: Some("_templates".into()),
            pages: Some("pages".into()),
            content: Some("content".into()),
            output: Some("dist".into()),
        }
    }
}
