#![feature(proc_macro_hygiene, decl_macro)]
use rocket;
use rocket::{get, routes};

mod components;

use nexo::component::Component;
// use nexo::render::WebRender;
// use nexo::store::Store;
use nexo::color::Color;
use nexo::length::Length;
use nexo::node::Node;
use nexo::render::web::css::ClassNames;
use nexo::tree::{NodeRef, Tree};

use nexo::render::web::render::Renderer;

use components::app::App;

#[get("/")]
fn index() -> rocket::response::content::Html<String> {
    let mut a = Renderer::new();

    let r = format!(
        include_str!("t.html"),
        styles = include_str!("styles.css"),
        body = a.render(App::new("App Name"))
    );

    // renderer.render(Test::new("App Name"));
    // rocket::response::content::Html(renderer.render(App::new("App Name")))
    rocket::response::content::Html(r)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
