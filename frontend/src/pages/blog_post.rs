use flate2::write::DeflateEncoder;
use flate2::Compression;
use leptos::*;
use leptos_meta::Title;
use leptos_router::use_params_map;
use pulldown_cmark::*;
use std::io::{Cursor, Write};
use syntect::{highlighting::ThemeSet, html::highlighted_html_for_string, parsing::SyntaxSet};

use crate::{components::header::Header, pages::loading::LoadingPage, types::Post};

#[component]
pub fn BlogPostPage(
    logged_in: ReadSignal<bool>,
    blog_posts: ReadSignal<Vec<Post>>,
) -> impl IntoView {
    let (blog_post, set_blog_post) = create_signal(None::<Post>);
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned().unwrap());

    // filter slug to find blog post
    set_blog_post(blog_posts.get().iter().find(|&b| b.slug == slug()).cloned());

    view! {
        <Title text=move || blog_post.get().map_or("No Title Found".into(), |p| p.title)/>
        <Header logged_in/>
        <div class="min-w-full pt-10 px-64">
            <Show when=move || blog_post.get().is_some() fallback=LoadingPage>
                <BlogPost content=blog_post.get().unwrap().markdown_content />
            </Show>
        </div>
    }
}

#[component]
pub fn BlogPost(#[prop(into)] content: String) -> impl IntoView {
    view! {
        <div class="markdown" inner_html=markdown_to_html(content.as_str())></div>
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
    let events = add_markdown_heading_ids(parser.into_iter().collect());
    let events = highlight_code(events);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, events.into_iter());

    html_output
}

fn add_markdown_heading_ids(events: Vec<Event<'_>>) -> Vec<Event<'_>> {
    let mut parsing_header = false;
    let mut heading_id = String::new();
    let mut events_to_return = Vec::new();

    for event in events {
        match event {
            Event::Start(pulldown_cmark::Tag::Heading { .. }) => {
                parsing_header = true;
                heading_id.clear();
            }
            Event::End(pulldown_cmark::TagEnd::Heading { .. }) => {
                parsing_header = false;
                heading_id = heading_id.replace(" ", "_");

                events_to_return.push(Event::Text(CowStr::from(" ")));
                events_to_return.push(Event::Html(CowStr::from(format!(
                    "<a href=\"#{}\" id=\"{}\"><span class=\"anchor-icon\">#</span></a>",
                    heading_id, heading_id
                ))));
            }
            Event::Text(ref text) => {
                if parsing_header {
                    heading_id.push_str(text);
                }
            }
            _ => {}
        }
        events_to_return.push(event);
    }

    events_to_return
}

fn highlight_code(events: Vec<Event<'_>>) -> Vec<Event<'_>> {
    let mut in_code_block = false;
    let syntax_set = SyntaxSet::load_defaults_nonewlines();
    let mut syntax = syntax_set.find_syntax_plain_text();

    // TODO: dark
    // let theme = ThemeSet::load_from_reader(&mut Cursor::new(include_str!(
    //     "../../public/assets/rose-pine.tmTheme"
    // )))
    // .unwrap();
    let theme = ThemeSet::load_from_reader(&mut Cursor::new(include_str!(
        "../../public/assets/peel-light.tmTheme"
    )))
    .unwrap();

    let mut to_highlight = String::new();
    let mut out_events = Vec::new();

    let mut plantuml = false;

    for event in events {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                match kind {
                    CodeBlockKind::Fenced(lang) => {
                        plantuml = lang == "plantuml".into();
                        syntax = syntax_set.find_syntax_by_token(&lang).unwrap_or(syntax);
                    }
                    CodeBlockKind::Indented => {}
                }
                in_code_block = true;
            }
            Event::End(TagEnd::CodeBlock) => {
                if !in_code_block {
                    panic!("this should never happen");
                }

                if plantuml {
                    let diagram_url = generate_plantuml_diagram_url(&to_highlight);
                    let img_tag =
                        format!("<img src=\"{}\" alt=\"PlantUML Diagram\" />", diagram_url);
                    out_events.push(Event::Html(CowStr::from(img_tag)));
                } else {
                    // Regular code block, highlight syntax
                    let html =
                        highlighted_html_for_string(&to_highlight, &syntax_set, syntax, &theme)
                            .unwrap();
                    out_events.push(Event::Html(CowStr::from(html)));
                }

                to_highlight.clear();
                in_code_block = false;
            }
            Event::Text(t) => {
                if in_code_block {
                    to_highlight.push_str(&t);
                } else {
                    out_events.push(Event::Text(t));
                }
            }
            e => {
                out_events.push(e);
            }
        }
    }

    out_events
}

fn generate_plantuml_diagram_url(plantuml_code: &str) -> String {
    let encoded = encode64(&compress_data(plantuml_code));
    let url = format!("http://www.plantuml.com/plantuml/png/{}", encoded);
    url
}

fn compress_data(data: &str) -> Vec<u8> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(data.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

fn encode6bit(b: u8) -> char {
    match b {
        0..=9 => (b + 48) as char,        // '0'..'9'
        10..=35 => (b - 10 + 65) as char, // 'A'..'Z'
        36..=61 => (b - 36 + 97) as char, // 'a'..'z'
        62 => '-',                        // '-'
        63 => '_',                        // '_'
        _ => '?',                         // Fallback (should not happen)
    }
}

fn append3bytes(b1: u8, b2: u8, b3: u8) -> String {
    let c1 = b1 >> 2;
    let c2 = ((b1 & 0x3) << 4) | (b2 >> 4);
    let c3 = ((b2 & 0xF) << 2) | (b3 >> 6);
    let c4 = b3 & 0x3F;

    let mut r = String::new();
    r.push(encode6bit(c1 & 0x3F));
    r.push(encode6bit(c2 & 0x3F));
    r.push(encode6bit(c3 & 0x3F));
    r.push(encode6bit(c4 & 0x3F));

    r
}

fn encode64(c: &[u8]) -> String {
    let mut str = String::new();
    let len = c.len();

    let mut i = 0;
    while i < len {
        if i + 2 == len {
            str.push_str(&append3bytes(c[i], c[i + 1], 0));
        } else if i + 1 == len {
            str.push_str(&append3bytes(c[i], 0, 0));
        } else {
            str.push_str(&append3bytes(c[i], c[i + 1], c[i + 2]));
        }
        i += 3;
    }
    str
}
