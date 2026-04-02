#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::{
    prelude::*,
    router::{Router, RouterConfig},
    winit::window::Fullscreen,
};

use crate::{router::Route, theme::MAIN};
mod components;
mod layouts;
mod router;
mod theme;
mod views;

fn main() {
    dotenv::dotenv().ok();
    let is_dev = std::env::var("RUST_ENV").as_deref() == Ok("development");

    let window = WindowConfig::new(app);

    let window = if is_dev {
        window
            .with_title("Orbita - Dev")
            .with_window_attributes(|attrs, _| {
                attrs
                    .with_window_level(freya::winit::window::WindowLevel::AlwaysOnTop)
                    .with_fullscreen(if cfg!(target_os = "macos") {
                        Some(Fullscreen::Borderless(None))
                    } else {
                        None
                    })
            })
    } else {
        window
            .with_title("Orbita - Production")
            .with_size(500., 450.)
    };

    launch(LaunchConfig::new().with_window(window))
}

fn app() -> impl IntoElement {
    use_init_theme(|| MAIN);
    Router::<Route>::new(|| RouterConfig::default().with_initial_path(Route::HomeView))
        .into_element()
}
