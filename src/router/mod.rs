use crate::layouts::AppLayout;
pub use crate::views::HomeView;
use freya::{prelude::*, router::*};

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        HomeView,
}
