use serde::{Deserialize, Serialize};

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
