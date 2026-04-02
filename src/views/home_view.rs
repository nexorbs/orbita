use freya::prelude::*;

use crate::theme::TEXT_PRIMARY;

#[derive(PartialEq)]
pub struct HomeView;
impl Component for HomeView {
    fn render(&self) -> impl IntoElement {
        label().text("Homee").color(TEXT_PRIMARY)
    }
}
