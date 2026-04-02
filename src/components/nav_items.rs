use freya::icons;

use crate::{components::NavButton, router::Route, theme::SUCCESS};

pub enum NavItem {
    Button(NavButton),
    Hr,
}

pub fn get_nav_items() -> Vec<NavItem> {
    vec![
        NavItem::Button(NavButton::new(
            "New connection".to_string(),
            Route::HomeView,
            icons::lucide::plus(),
        )),
        NavItem::Button(NavButton::new(
            "Open".to_string(),
            Route::HomeView,
            icons::lucide::folder_open(),
        )),
        NavItem::Hr,
        NavItem::Button(
            NavButton::new(
                "Execute".to_string(),
                Route::HomeView,
                icons::lucide::play(),
            )
            .with_icon_color(SUCCESS),
        ),
        NavItem::Button(NavButton::new(
            "Refresh".to_string(),
            Route::HomeView,
            icons::lucide::rotate_ccw(),
        )),
        NavItem::Hr,
        NavItem::Button(NavButton::new(
            "Save".to_string(),
            Route::HomeView,
            icons::lucide::save(),
        )),
        NavItem::Button(NavButton::new(
            "Settings".to_string(),
            Route::SettingsView,
            icons::lucide::settings(),
        )),
    ]
}
