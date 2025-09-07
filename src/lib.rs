use serde::{Deserialize, Serialize};
pub mod console;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Seo {
    /// Title (~60 chars)
    pub title: String,
    /// Description (~160 chars)
    pub description: String,
    pub keywords: Vec<String>,
    pub author: Option<String>,

    pub canonical_url: Option<String>,
    pub lang: Option<String>,    // ex: "fr"
    pub updated: Option<String>, // ISO8601

    // Social
    pub og_image: Option<String>,
    pub og_type: Option<String>,      // "website" | "article" | "book"…
    pub twitter_card: Option<String>, // "summary_large_image"

    // (Optionnel) Pour générer du JSON-LD
    pub json_ld: Option<String>,

    // (Optionnel) Jardin
    pub content_type: Option<String>, // "work" | "author" | "season" | "event"
    pub slug: Option<String>,
}

/// Implementation of builder-style methods for the `Seo` struct, enabling convenient and fluent
/// configuration of SEO-related metadata for web content.
///
/// # Methods
///
/// - `new()`: Constructs a new `Seo` instance with default values.
/// - `with_title(&mut self, t: &str)`: Sets the page title.
/// - `with_desc(&mut self, d: &str)`: Sets the page description.
/// - `with_keywords(&mut self, ks: &[&str])`: Sets the list of keywords for SEO.
/// - `with_author(&mut self, a: &str)`: Sets the author of the content.
/// - `with_lang(&mut self, l: &str)`: Sets the language of the content.
/// - `with_updated(&mut self, u: &str)`: Sets the last updated timestamp.
/// - `with_json_ld(&mut self, j: &str)`: Sets the JSON-LD structured data.
/// - `with_content_type(&mut self, c: &str)`: Sets the content type (e.g., "article").
/// - `with_canonical(&mut self, url: &str)`: Sets the canonical URL for the page.
/// - `with_slug(&mut self, s: &str)`: Sets the slug for the page.
/// - `with_og_image(&mut self, img: &str)`: Sets the Open Graph image URL.
/// - `with_og_type(&mut self, t: &str)`: Sets the Open Graph type (e.g., "website").
/// - `with_twitter_card(&mut self, c: &str)`: Sets the Twitter card type.
/// - `twitter_summary(&mut self)`: Sets the Twitter card type to "summary".
///
/// All builder methods return a mutable reference to `Self`, allowing for method chaining.
/// These methods ensure that SEO metadata can be set in a clear, concise, and type-safe manner,
/// facilitating the generation of rich meta tags for web pages.
impl Seo {
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            keywords: Vec::new(),
            author: None,
            canonical_url: None,
            lang: None,
            updated: None,
            og_image: None,
            og_type: None,
            twitter_card: None,
            json_ld: None,
            content_type: None,
            slug: None,
        }
    }
    pub fn with_title(&mut self, t: &str) -> &mut Self {
        self.title.clear();
        self.title.push_str(t);
        self
    }
    pub fn with_desc(&mut self, d: &str) -> &mut Self {
        self.description.clear();
        self.description.push_str(d);
        self
    }
    pub fn with_keywords(&mut self, ks: &[&str]) -> &mut Self {
        self.keywords.clear();
        self.keywords.extend(ks.iter().map(|s| s.to_string()));
        self
    }
    pub fn with_author(&mut self, a: &str) -> &mut Self {
        self.author.replace(a.to_string());
        self
    }
    pub fn with_lang(&mut self, l: &str) -> &mut Self {
        self.lang.replace(l.to_string());
        self
    }
    pub fn with_updated(&mut self, u: &str) -> &mut Self {
        self.updated.replace(u.to_string());
        self
    }
    pub fn with_json_ld(&mut self, j: &str) -> &mut Self {
        self.json_ld.replace(j.to_string());
        self
    }
    pub fn with_content_type(&mut self, c: &str) -> &mut Self {
        self.content_type.replace(c.to_string());
        self
    }
    pub fn with_canonical(&mut self, url: &str) -> &mut Self {
        self.canonical_url.replace(url.to_string());
        self
    }
    pub fn with_slug(&mut self, s: &str) -> &mut Self {
        self.slug.replace(s.to_string());
        self
    }
    pub fn with_og_image(&mut self, img: &str) -> &mut Self {
        self.og_image.replace(img.to_string());
        self
    }
    pub fn with_og_type(&mut self, t: &str) -> &mut Self {
        self.og_type.replace(t.to_string());
        self
    }

    pub fn with_twitter_card(&mut self, c: &str) -> &mut Self {
        self.twitter_card.replace(c.to_string());
        self
    }

    pub fn twitter_summary(&mut self) -> &mut Self {
        self.twitter_card.replace("summary".to_string());
        self
    }
}

///
/// Module `render` provides functionality for integrating Tera templates with Rocket,
/// including custom filters and functions, as well as handling embedded assets using RustEmbed.
/// It sets up the templating engine, registers various utilities for content rendering,
/// and ensures that public assets are served correctly.
///     
pub mod render {
    use rocket::fairing::AdHoc;
    use rocket_dyn_templates::tera::Tera;
    use rust_embed::RustEmbed;

    pub mod filters;
    pub mod funcs;
    pub mod helpers;
    pub mod jsonld;

    #[derive(RustEmbed)]
    #[folder = "templates/"]
    struct KitTemplates;

    #[derive(RustEmbed)]
    #[folder = "front/"]
    struct KitAssets;

    pub fn attach() -> AdHoc {
        AdHoc::on_ignite("roots_kit", |rocket| async {
            let rocket = rocket.attach(rocket_dyn_templates::Template::custom(|engines| {
                let tera: &mut Tera = &mut engines.tera;

                // Charger les templates embarqués
                for file in KitTemplates::iter() {
                    let path = file.as_ref();
                    if let Some(content) = KitTemplates::get(path) {
                        let s = std::str::from_utf8(content.data.as_ref()).unwrap();
                        tera.add_raw_template(path, s).expect(path);
                    }
                }

                tera.register_function("num_format", helpers::num_format);
                tera.register_function("money_eur", helpers::money_eur);
                tera.register_function("percent", helpers::percent);
                tera.register_function("safe_url", helpers::safe_url);
                tera.register_function("external_rel", helpers::external_rel);
                tera.register_function("csrf_input", helpers::csrf_input);
                tera.register_function("active_link", helpers::active_link);
                tera.register_function("paginate", helpers::paginate);

                // Filtres
                tera.register_filter("markdown", filters::markdown);
                tera.register_filter("md_excerpt", filters::md_excerpt);
                tera.register_filter("date", filters::date);
                tera.register_filter("ago", filters::ago);
                tera.register_filter("truncate", filters::truncate);
                tera.register_filter("slugify", filters::slugify);
                tera.register_filter("nl2br", filters::nl2br);
                tera.register_filter("json", filters::json_pp);

                // Fonctions
                tera.register_function("asset", funcs::asset);
                tera.register_function("asset_tag", funcs::asset_tag);
                tera.register_function("img_srcset", funcs::img_srcset);
                tera.register_function("picture", funcs::picture);
                tera.register_function("canonical", funcs::canonical);
            }));

            // Exposer assets (roots.css, icons.svg)

            rocket
                .mount(
                    "/kit",
                    rocket::fs::FileServer::from("target/roots_kit_assets"),
                )
                .attach(AdHoc::on_liftoff("extract_assets", |_| {
                    Box::pin(async {
                        // à la liftoff, on écrit les assets embarqués sur disque (ou servez via route custom)
                        use std::{fs, path::Path};
                        let out = "target/roots_kit_assets";
                        fs::create_dir_all(out).ok();
                        for f in KitAssets::iter() {
                            let path = format!("{out}/{}", f.as_ref());
                            if let Some(content) = KitAssets::get(f.as_ref()) {
                                if let Some(parent) = Path::new(&path).parent() {
                                    fs::create_dir_all(parent).ok();
                                }
                                fs::write(path, content.data.as_ref()).ok();
                            }
                        }
                    })
                }))
        })
    }
}
