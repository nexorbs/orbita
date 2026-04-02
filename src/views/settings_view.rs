use freya::prelude::*;

use crate::{components::NavButton, theme::TEXT_PRIMARY};

#[derive(PartialEq)]
pub struct SettingsView;
impl Component for SettingsView {
    fn render(&self) -> impl IntoElement {
        rect()
            .expanded()
            .content(Content::Flex)
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .spacing(8.)
            .child(
                NavButton::new(
                    "Go back".to_string(),
                    crate::router::Route::HomeView,
                    freya::icons::lucide::arrow_left(),
                )
                .with_icon_color(TEXT_PRIMARY),
            )
            .child(label().text("Settings").color(TEXT_PRIMARY))
    }
}
