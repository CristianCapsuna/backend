use axum::{
    response::Html,
    routing::get,
    Router,
};
// use serde::{Deserialize, Serialize};
use pulldown_cmark::{Parser, html};
use std::fs::File;
use std::io::Read;
use dioxus::prelude::*;
use dioxus_ssr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(render_markdown))
        .nest_service("/static", ServeDir::new("static/"));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on \u{1b}]8;;{}\u{1b}\\{}\u{1b}]8;;\u{1b}\\", listener.local_addr().unwrap(), listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn render_markdown() -> Html<String> {
    let mut file = File::open("/srv/content/CV.md").unwrap();
    let mut cv_string = String::new();
    file.read_to_string(&mut cv_string).unwrap();
    
    let parser = Parser::new(&cv_string);
    let mut cv_html = String::new();
    html::push_html(&mut cv_html, parser);
    let html_wrapper = Html(dioxus_ssr::render_element(rsx! {
            link { rel: "stylesheet", href: "/static/style.css" }
            head {
                title { "About" }
                link {
                    rel: "stylesheet",
                    href: "/srv/fonts/roboto/Roboto-BlackItalic.ttf"
                }
                link { href: "/srv/fonts/roboto/Roboto-Black.ttf", rel: "stylesheet" }
                link {
                    href: "/srv/fonts/roboto/Roboto-BoldItalic.ttf",
                    rel: "stylesheet"
                }
                link {
                    href: "/srv/fonts/roboto/Roboto-BoldItalic.ttf",
                    rel: "stylesheet"
                }
                link { rel: "stylesheet", href: "/srv/fonts/roboto/Roboto-Bold.ttf" }
                link {
                    rel: "stylesheet",
                    href: "/srv/fonts/roboto/Roboto-Italic.ttf"
                }
                link {
                    href: "/srv/fonts/roboto/Roboto-LightItalic.ttf",
                    rel: "stylesheet"
                }
                link { rel: "stylesheet", href: "/srv/fonts/roboto/Roboto-Light.ttf" }
                link {
                    rel: "stylesheet",
                    href: "/srv/fonts/roboto/Roboto-MediumItalic.ttf"
                }
                link {
                    href: "/srv/fonts/roboto/Roboto-Medium.ttf",
                    rel: "stylesheet"
                }
                link {
                    href: "/srv/fonts/roboto/Roboto-Regular.ttf",
                    rel: "stylesheet"
                }
                link {
                    href: "/srv/fonts/roboto/Roboto-ThinItalic.ttf",
                    rel: "stylesheet"
                }
                link { rel: "stylesheet", href: "/srv/fonts/roboto/Roboto-Thin.ttf" }
                link { rel: "stylesheet", href: "styles.css", r#type: "text/css" }
                meta {
                    content: "width=device-width, initial-scale=1",
                    name: "viewport"
                }
                meta { charset: "UTF-8" }
            }
            body {
                button { id: "toggleBtn", "Toggle Table of Contents" }
                div { id: "container",
                    div { id: "table_of_content",
                        ul {
                            li {
                                a { href: "#about_me", "About me" }
                            }
                            li {
                                a { href: "#personal_projects", "Personal projects" }
                            }
                            li {
                                a { href: "#work_experience", "Work experience" }
                            }
                            li {
                                a { href: "#education", "Education" }
                            }
                        }
                    }
                    div { id: "content",
                        dangerous_inner_html: "{cv_html}"
                }
            }
            script { src: "/static/mobile_toc_hide_and_show.js" }
        }
    }));

    html_wrapper
}