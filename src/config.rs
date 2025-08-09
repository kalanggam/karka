pub struct Config {
    site: SiteOptions,
    i18n: Option<I18nOptions>,
    build: BuildOptions,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            site: SiteOptions::default(),
            i18n: None,
            build: BuildOptions::default(),
        }
    }
}

struct SiteOptions {
    name: Option<String>,
    author: Option<Vec<SiteAuthor>>,
    base_url: String,
}

impl Default for SiteOptions {
    fn default() -> Self {
        Self {
            name: Some("Example site".into()),
            author: Some(vec![SiteAuthor::default()]),
            base_url: "https://www.example.com".into(),
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

struct BuildOptions {
    verbose: Option<bool>,
    include_drafts: Option<bool>,
    ignore: Option<String>,
    dirs: BuildDirs,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            verbose: Some(false),
            include_drafts: Some(false),
            ignore: Some(".karkaignore".into()),
            dirs: BuildDirs::default(),
        }
    }
}

struct BuildDirs {
    assets: Option<String>,
    data: Option<String>,
    schemas: Option<String>,
    templates: Option<String>,
    pages: Option<String>,
    content: Option<String>,
    output: Option<String>,
}

impl Default for BuildDirs {
    fn default() -> Self {
        BuildDirs {
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
