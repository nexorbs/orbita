use crate::layouts::AppLayout;
pub use crate::views::HomeView;
pub use crate::views::SettingsView;
use freya::{prelude::*, router::*};

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        HomeView,
        #[route("/settings")]
        SettingsView,
}
