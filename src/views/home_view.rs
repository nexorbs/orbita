use freya::prelude::*;

use crate::theme::TEXT_PRIMARY;

#[derive(PartialEq)]
pub struct HomeView;
impl Component for HomeView {
    fn render(&self) -> impl IntoElement {
        let h_controller = use_state(|| ResizableContext {
            panels: vec![],
            direction: Direction::Horizontal,
        });
        let v_controller = use_state(|| ResizableContext {
            panels: vec![],
            direction: Direction::Vertical,
        });

        rect().expanded().child(
            ResizableContainer::new()
                .direction(Direction::Horizontal)
                .controller(h_controller)
                .panel(
                    // TODO: Replace with a real component not with a .rect() placeholder
                    ResizablePanel::new(PanelSize::percent(20.))
                        .min_size(10.)
                        .child(
                            rect()
                                .expanded()
                                .center()
                                .child(label().text("Left").color(TEXT_PRIMARY)),
                        ),
                )
                .panel(
                    ResizablePanel::new(PanelSize::percent(80.))
                        .min_size(10.)
                        .child(
                            ResizableContainer::new()
                                .direction(Direction::Vertical)
                                .controller(v_controller)
                                .panel(
                                    // TODO: Replace with a real component not with a .rect() placeholder
                                    ResizablePanel::new(PanelSize::percent(50.))
                                        .min_size(10.)
                                        .child(
                                            rect().expanded().center().child(
                                                label().text("Center Top").color(TEXT_PRIMARY),
                                            ),
                                        ),
                                )
                                .panel(
                                    // TODO: Replace with a real component not with a .rect() placeholder
                                    ResizablePanel::new(PanelSize::percent(50.0))
                                        .min_size(10.)
                                        .child(rect().expanded().center().child(
                                            label().text("Center Bottom").color(TEXT_PRIMARY),
                                        )),
                                ),
                        ),
                ),
        )
    }
}
