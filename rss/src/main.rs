use std::cmp::Reverse;
use std::convert::Into;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use anyhow::Context;
use atom_syndication::{
    CategoryBuilder, EntryBuilder, FeedBuilder, LinkBuilder, PersonBuilder, WriteConfig,
};
use chrono::{FixedOffset, TimeZone, Utc};
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    /// Path where the Atom feed should be generated
    output: PathBuf,

    /// Path to a data directory containing YAML files.
    #[clap(long = "data-path")]
    data_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut entries = parse_feed_entries(&args.data_dir)?;
    println!("Found entries: {entries:#?}");
    entries.sort_by_key(|entry| Reverse(entry.published_at));

    let last_update = entries.iter().map(|entry| entry.published_at).max();
    let web_uri = "https://rustlang.cz";

    let mut builder = FeedBuilder::default();
    builder.title("Czech Rust language events")
        .subtitle(Some("Events and talk recordings related to the Rust language community in the Czech Republic".into()))
        .id(web_uri)
        .links(vec![LinkBuilder::default()
            .title("Atom feed".to_string())
            .href(format!("{web_uri}/feed.atom"))
            .build()])
        .authors(vec![PersonBuilder::default()
            .name("Jakub Beránek")
            .email("berykubik@gmail.com".to_string())
            .uri("https://github.com/kobzol".to_string())
            .build()]);
    if let Some(updated) = last_update {
        builder.updated(updated);
    }

    builder.entries(
        entries
            .into_iter()
            .map(|entry| {
                EntryBuilder::default()
                    .title(entry.name)
                    .id(&entry.link)
                    .links(vec![LinkBuilder::default().href(entry.link).build()])
                    .updated(entry.published_at)
                    .published(Some(entry.published_at))
                    .categories(vec![CategoryBuilder::default()
                        .term(entry.category.as_str().to_string())
                        .label(Some(entry.category.as_str().to_string()))
                        .build()])
                    .build()
            })
            .collect::<Vec<_>>(),
    );

    let feed = builder.build();
    let output = BufWriter::new(std::fs::File::create(args.output)?);

    let mut config = WriteConfig::default();
    config.indent_size = Some(4);
    feed.write_with_config(output, config)?;

    Ok(())
}

#[derive(serde::Deserialize, Debug)]
struct YamlFeedEntry {
    name: String,
    link: String,
    #[serde(default)]
    published: Option<chrono::DateTime<FixedOffset>>,
}

#[derive(Debug, Copy, Clone)]
enum Category {
    Event,
    Recording,
}

impl Category {
    fn as_str(&self) -> &str {
        match self {
            Category::Event => "event",
            Category::Recording => "recording",
        }
    }
}

#[derive(Debug)]
struct FeedEntry {
    name: String,
    link: String,
    published_at: chrono::DateTime<FixedOffset>,
    category: Category,
}

fn parse_feed_entries(data_dir: &Path) -> anyhow::Result<Vec<FeedEntry>> {
    let mut entries = vec![];
    parse_entries_from(&data_dir.join("events.yaml"), Category::Event, &mut entries)?;
    parse_entries_from(
        &data_dir.join("recordings.yaml"),
        Category::Recording,
        &mut entries,
    )?;
    Ok(entries)
}

fn parse_entries_from(
    path: &Path,
    category: Category,
    entries: &mut Vec<FeedEntry>,
) -> anyhow::Result<()> {
    let DEFAULT_PUBLISH_DATE = Utc.with_ymd_and_hms(2024, 02, 24, 11, 0, 0).unwrap();

    let data = std::fs::read_to_string(path)
        .with_context(|| format!("Cannot read entries from {}", path.display()))?;
    let parsed = serde_yaml::from_str::<Vec<YamlFeedEntry>>(&data)?;
    entries.extend(parsed.into_iter().map(
        |YamlFeedEntry {
             name,
             link,
             published,
         }| FeedEntry {
            name,
            link,
            published_at: published.unwrap_or(DEFAULT_PUBLISH_DATE.into()),
            category,
        },
    ));

    Ok(())
}
