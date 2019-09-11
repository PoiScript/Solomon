use chrono::Utc;
use maud::{html, Markup, Render};
use orgize::Org;

use crate::entry::Entry;
use crate::handlers::SolomonRssHandler;

struct Rss<'a>(&'a Org<'a>);

impl<'a> Render for Rss<'a> {
    fn render_to(&self, w: &mut String) {
        let mut vec = Vec::new();
        self.0
            .html_with_handler(&mut vec, SolomonRssHandler::default())
            .unwrap();
        w.push_str(&String::from_utf8(vec).unwrap());
    }
}

pub fn markup(entries: &[Entry]) -> Markup {
    html! {
        rss version="2.0"
            xmlns:atom="http://www.w3.org/2005/Atom"
            xmlns:content="http://purl.org/rss/1.0/modules/content/"
            xmlns:dc="http://purl.org/dc/elements/1.1/"
        {
            channel {
                title { "solomon" }
                description { "PoiScript's Blog" }
                link rel="self" href="https://blog.poi.cat/atom.xml" {}
                link rel="alternate" href="https://blog.poi.cat" {}
                generator { "solomon "(env!("CARGO_PKG_VERSION")) }
                lastBuildDate { (Utc::now().to_rfc2822()) }
                language { "zh-Hans" }
                copyright { "Content licensed under CC-BY-SA-4.0." }
                @for entry in entries {
                    item {
                        title { (&entry.title) }
                        author { "PoiScript" }
                        link { "https://blog.poi.cat/post/"(entry.slug) }
                        guid isPermaLink="false" { (entry.slug) }
                        @for tag in &entry.tags {
                            category { (tag) }
                        }
                        pubDate { (entry.date.format("%a, %e %b %Y"))" 00:00:00 +0000" }
                        description { "<![CDATA["(Rss(&entry.org))"]]>" }
                    }
                }
            }
        }
    }
}
