extern crate serde_json;

use anyhow::Result;
use mdbook_preprocessor::book::{Book, BookItem, Chapter};
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag, TagEnd};
use regex::Regex;

pub mod process_diagram;
use process_diagram::process_diagram;
pub mod handlebar_helpers;

pub struct MermaidAnimatePreprocessor;

fn run_section(section: &BookItem) -> Result<BookItem> {
    // dbg!("section: {}", &section);
    let section = match section {
        BookItem::Chapter(chapter) => run_chapter(chapter)?,
        _ => unimplemented!(),
    };
    Ok(BookItem::Chapter(section))
}

fn run_chapter(chapter: &Chapter) -> Result<Chapter> {
    let re = Regex::new(r#"(?ms)(?<mermaid><pre .*?class=\"mermaid\".*?</pre>)"#).unwrap();
    let mut count = 1;

    let mut replacements: Vec<(String, String)> = vec![];
    let mut data = chapter.content.clone();
    loop {
        let caps = re.captures(&data);
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        // dbg!("caps: {:?}", &caps);
        let diagram = &caps["mermaid"];
        let processed_diagram = process_diagram(&diagram)?;
        // dbg!("before diagram: {}", &diagram);
        // dbg!("processed diagram: {}", &processed_diagram);
        let id = uuid::Uuid::new_v4();
        // dbg!("generated id: {}", &id);
        replacements.push((id.to_string(), processed_diagram.clone()));
        let mut dst = String::new();
        dst.push_str(&data[..caps.get(0).unwrap().start()]);
        dst.push_str(id.to_string().as_str());
        dst.push_str(&data[caps.get(0).unwrap().end()..]);
        data = dst;
    }

    let mut chapter = chapter.clone();
    for (id, processed_diagram) in replacements {
        data = data.replace(&id, &processed_diagram);
    }

    chapter.content = data;

    let sub_items: Vec<_> = chapter
        .sub_items
        .iter()
        .map(|c| run_section(c))
        .collect::<Result<Vec<_>>>()?;

    chapter.sub_items = sub_items;

    Ok(chapter.clone())
}

fn run_all(book: &Book) -> Result<Book> {
    let items: &Vec<BookItem> = &book
        .items
        .iter()
        .map(|section| run_section(section))
        .collect::<Result<Vec<_>>>()?;

    let mut book: &mut Book = &mut book.clone();
    book.items = items.clone();
    Ok(book.clone())
}

impl Preprocessor for MermaidAnimatePreprocessor {
    fn name(&self) -> &str {
        "mermaid-animate"
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book> {
        log::info!("Running mermaid-animate preprocessor");

        let processed_book = run_all(&book);
        match processed_book {
            Err(e) => {
                eprintln!("Error during processing: {}", e);
                Ok(book)
            }
            Ok(b) => Ok(b),
        }
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool> {
        Ok(renderer == "html")
    }
}
