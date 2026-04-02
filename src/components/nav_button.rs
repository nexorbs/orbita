use freya::{
    animation::{AnimColor, use_animation}, prelude::*, router::RouterContext
};
use crate::{router::Route, theme::{BACKGROUND, TEXT_PRIMARY}};

#[derive(PartialEq, Clone)]
pub struct NavButton {
    pub text: String,
    pub route: Route,
    pub icon: Bytes,
    pub icon_color: Option<Color>,
}

impl Component for NavButton {
    fn render(&self) -> impl IntoElement {
        let route = self.route.clone();
        #[rustfmt::skip]
        let mut anim = use_animation(|_| 
            AnimColor::new(
                BACKGROUND,
                (55, 60, 78)
            )
            .time(200)
        );

        rect()
            .content(Content::Flex)
            .horizontal()
            .main_align(Alignment::Center)
            .cross_align(Alignment::Center)
            .spacing(4.)
            .padding(Gaps::new(6., 3., 6., 3.))
            .background(&*anim.read())
            .color(self.icon_color.clone().unwrap_or(TEXT_PRIMARY))
            .on_pointer_enter(move |_| anim.start())
            .on_pointer_leave(move |_| anim.reverse())
            .on_press(move |_| {
                // TODO: Not Route, this open a modal or something
                RouterContext::get().push(route.clone());
            })
            .child(svg(self.icon.clone()).color(self.icon_color.clone().unwrap_or(TEXT_PRIMARY))
            .width(Size::px(15.)).height(Size::px(15.)))
            .child(label().text(self.text.clone()))
    }
}

impl NavButton {
    pub fn new(text: String, route: Route, icon: Bytes) -> Self {
        Self {
            text,
            route,
            icon,
            icon_color: None,
        }
    }

    pub fn with_icon_color(mut self, icon_color: Color) -> Self {
        self.icon_color = Some(icon_color);
        self
    }
}