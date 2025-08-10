pub struct Config {
    site_name: Option<String>,
    base_url: String,
    verbose: bool,
    include_drafts: bool,
    ignore: Option<String>,
    authors: Option<Vec<SiteAuthor>>,
    i18n: Option<I18nOptions>,
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
            i18n: None,
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

struct I18nOptions {
    input: Option<I18nInputMode>,
    output: Option<I18nOutputMode>,
    default: Option<String>,
    languages: Vec<String>,
}

impl Default for I18nOptions {
    fn default() -> Self {
        Self {
            input: Some(I18nInputMode::Frontmatter),
            output: Some(I18nOutputMode::SubdirExceptDefault),
            default: Some("en".into()),
            languages: vec!["en".into()],
        }
    }
}

enum I18nInputMode {
    Frontmatter,
}

enum I18nOutputMode {
    SubdirExceptDefault,
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
