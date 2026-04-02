use freya::{prelude::*, router::Outlet};

use crate::{
    components::{NavButton, get_nav_items, nav_items::NavItem},
    router::Route,
    theme::TEXT_PRIMARY,
};

#[derive(Clone, PartialEq)]
pub struct AppLayout;

impl Component for AppLayout {
    fn render(&self) -> impl IntoElement {
        rect()
            .expanded()
            .content(Content::Flex)
            .theme_background()
            .child(
                get_nav_items().into_iter().fold(
                    rect()
                        .width(Size::fill())
                        .height(Size::px(35.))
                        .horizontal()
                        .cross_align(Alignment::Center)
                        .padding(2.)
                        .spacing(2.),
                    |nav, item| match item {
                        NavItem::Button(btn) => nav.child(btn),
                        NavItem::Hr => nav.child(
                            rect()
                                .margin(Gaps::new(0., 5., 0., 5.))
                                .width(Size::px(1.))
                                .height(Size::px(20.))
                                .background(TEXT_PRIMARY),
                        ),
                    },
                ),
            )
            .child(
                rect()
                    .width(Size::fill())
                    .height(Size::flex(1.))
                    .padding(16.)
                    .child(Outlet::<Route>::new()),
            )
    }
}
