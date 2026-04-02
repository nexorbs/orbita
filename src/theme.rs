use freya::prelude::*;
use freya_components::theming::macros::Preference;

// Brand & Accent
pub const PRIMARY: Color = Color::from_rgb(37, 99, 235);
pub const SECONDARY: Color = Color::from_rgb(123, 129, 236); // #7B81EC
pub const TERTIARY: Color = Color::from_rgb(96, 165, 250); // #60A5FA

// Surfaces
pub const BACKGROUND: Color = Color::from_rgb(12, 12, 12); // #0C0C0C
pub const SURFACE_PRIMARY: Color = Color::from_rgb(31, 34, 40); // #1F2228
pub const SURFACE_SECONDARY: Color = Color::from_rgb(37, 40, 47); // #25282F
pub const SURFACE_TERTIARY: Color = Color::from_rgb(31, 34, 40); // #1F2228
pub const SURFACE_INVERSE: Color = Color::from_rgb(255, 255, 255); // #FFFFFF
pub const SURFACE_INVERSE_SECONDARY: Color = Color::from_rgb(229, 231, 235);
pub const SURFACE_INVERSE_TERTIARY: Color = Color::from_rgb(209, 213, 219);

pub const WIDGET_BG: Color = Color::from_rgb(26, 29, 34);

// Status Colors
pub const SUCCESS: Color = Color::from_rgb(34, 197, 94); // #22C55E
pub const WARNING: Color = Color::from_rgb(234, 179, 8); // #EAB308
pub const ERROR: Color = Color::from_rgb(239, 68, 68); // #EF4444
pub const INFO: Color = Color::from_rgb(59, 130, 246); // #3B82F6

// Text
pub const TEXT_PRIMARY: Color = Color::from_rgb(255, 255, 255); // #FFFFFF
pub const TEXT_SECONDARY: Color = Color::from_rgb(181, 181, 181); // #B5B5B5
pub const TEXT_PLACEHOLDER: Color = Color::from_rgb(103, 107, 117); // #676B75
pub const TEXT_INVERSE: Color = Color::from_rgb(255, 255, 255); // #FFFFFF
pub const TEXT_HIGHLIGHT: Color = Color::from_rgb(59, 130, 246); // #3B82F6

pub const BORDER: Color = Color::from_argb(121, 53, 58, 65); // #373A42
pub const BORDER_FOCUS: Color = Color::from_rgb(59, 130, 246); // #3B82F6
pub const BORDER_DISABLED: Color = Color::from_rgb(45, 48, 55);

pub const TABLE_HEADER: Color = Color::from_rgb(32, 36, 43);
pub const TABLE_ROW: Color = Color::from_rgb(29, 32, 38);

pub const HOVER: Color = Color::from_rgb(37, 40, 47); // #25282F
pub const FOCUS: Color = Color::from_rgb(59, 130, 246); // #3B82F6
pub const ACTIVE: Color = Color::from_rgb(29, 78, 216); // #1D4ED8
pub const DISABLED: Color = Color::from_rgb(55, 58, 66);

pub const OVERLAY: Color = Color::from_af32rgb(0.5, 0, 0, 0);
pub const SHADOW: Color = Color::from_af32rgb(0.3, 0, 0, 0);

pub const MAIN: Theme = Theme {
    name: "main",
    colors: ColorsSheet {
        // Brand & Accent
        primary: PRIMARY,     // #3B82F6
        secondary: SECONDARY, // #7B81EC
        tertiary: TERTIARY,   // #60A5FA

        // Status Colors
        success: SUCCESS, // #22C55E
        warning: WARNING, // #EAB308
        error: ERROR,     // #EF4444
        info: INFO,       // #3B82F6

        // Surfaces
        background: BACKGROUND,               // #181A1F
        surface_primary: SURFACE_PRIMARY,     // #1F2228
        surface_secondary: SURFACE_SECONDARY, // #25282F
        surface_tertiary: SURFACE_TERTIARY,   // #1F2228
        surface_inverse: SURFACE_INVERSE,     // #FFFFFF
        surface_inverse_secondary: SURFACE_INVERSE_SECONDARY,
        surface_inverse_tertiary: SURFACE_INVERSE_TERTIARY,

        // Borders
        border: BORDER,             // #373A42
        border_focus: BORDER_FOCUS, // #3B82F6
        border_disabled: BORDER_DISABLED,

        // Text
        text_primary: TEXT_PRIMARY,         // #FFFFFF
        text_secondary: TEXT_SECONDARY,     // #B5B5B5
        text_placeholder: TEXT_PLACEHOLDER, // #676B75
        text_inverse: TEXT_INVERSE,         // #FFFFFF
        text_highlight: TEXT_HIGHLIGHT,     // #3B82F6

        // States
        hover: HOVER,   // #25282F
        focus: FOCUS,   // #3B82F6
        active: ACTIVE, // #1D4ED8
        disabled: DISABLED,

        // Utility
        overlay: OVERLAY,
        shadow: SHADOW,
    },
    button_layout: ButtonLayoutThemePreference {
        padding: Preference::Specific(Gaps::new(10., 16., 10., 16.)),
        margin: Preference::Specific(Gaps::new_all(0.)),
        corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
    },
    compact_button_layout: ButtonLayoutThemePreference {
        padding: Preference::Specific(Gaps::new(6., 12., 6., 12.)),
        margin: Preference::Specific(Gaps::new_all(0.)),
        corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
    },
    expanded_button_layout: ButtonLayoutThemePreference {
        padding: Preference::Specific(Gaps::new(12., 20., 12., 20.)),
        margin: Preference::Specific(Gaps::new_all(0.)),
        corner_radius: Preference::Specific(CornerRadius::new_all(10.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
    },
    button: ButtonColorsThemePreference {
        background: Preference::Reference("surface_primary"),
        hover_background: Preference::Reference("hover"),
        border_fill: Preference::Reference("border"),
        focus_border_fill: Preference::Reference("focus"),
        color: Preference::Reference("text_primary"),
    },
    filled_button: ButtonColorsThemePreference {
        background: Preference::Reference("primary"),
        hover_background: Preference::Specific(Color::from_rgb(59, 130, 246)),
        border_fill: Preference::Specific(Color::TRANSPARENT),
        focus_border_fill: Preference::Reference("primary"),
        color: Preference::Reference("text_inverse"),
    },
    outline_button: ButtonColorsThemePreference {
        background: Preference::Specific(Color::TRANSPARENT),
        hover_background: Preference::Reference("hover"),
        border_fill: Preference::Reference("border"),
        focus_border_fill: Preference::Reference("primary"),
        color: Preference::Reference("primary"),
    },
    flat_button: ButtonColorsThemePreference {
        background: Preference::Specific(Color::TRANSPARENT),
        hover_background: Preference::Reference("hover"),
        border_fill: Preference::Specific(Color::TRANSPARENT),
        focus_border_fill: Preference::Specific(Color::TRANSPARENT),
        color: Preference::Reference("text_primary"),
    },
    card_layout: CardLayoutThemePreference {
        padding: Preference::Specific(Gaps::new(20., 20., 20., 20.)),
        corner_radius: Preference::Specific(CornerRadius::new_all(12.)),
    },
    compact_card_layout: CardLayoutThemePreference {
        padding: Preference::Specific(Gaps::new(12., 16., 12., 16.)),
        corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
    },
    filled_card: CardColorsThemePreference {
        background: Preference::Reference("surface_primary"),
        hover_background: Preference::Reference("hover"),
        border_fill: Preference::Specific(Color::TRANSPARENT),
        color: Preference::Reference("text_primary"),
        shadow: Preference::Reference("shadow"),
    },
    outline_card: CardColorsThemePreference {
        background: Preference::Reference("surface_primary"),
        hover_background: Preference::Reference("hover"),
        border_fill: Preference::Reference("border"),
        color: Preference::Reference("text_primary"),
        shadow: Preference::Reference("shadow"),
    },
    accordion: AccordionThemePreference {
        color: Preference::Reference("text_primary"),
        background: Preference::Reference("surface_primary"),
        border_fill: Preference::Reference("border"),
    },
    switch: SwitchColorsThemePreference {
        background: Preference::Reference("border"),
        thumb_background: Preference::Reference("text_primary"),
        toggled_background: Preference::Reference("secondary"),
        toggled_thumb_background: Preference::Reference("text_inverse"),
        focus_border_fill: Preference::Reference("focus"),
    },
    scrollbar: ScrollBarThemePreference {
        background: Preference::Reference("surface_primary"),
        thumb_background: Preference::Reference("border"),
        hover_thumb_background: Preference::Reference("text_secondary"),
        active_thumb_background: Preference::Reference("text_primary"),
        size: Preference::Specific(12.),
    },
    progressbar: ProgressBarThemePreference {
        color: Preference::Reference("text_primary"),
        background: Preference::Reference("surface_primary"),
        progress_background: Preference::Reference("primary"),
        height: Preference::Specific(8.),
    },
    sidebar_item: SideBarItemThemePreference {
        color: Preference::Reference("text_primary"),
        background: Preference::Specific(Color::TRANSPARENT),
        active_background: Preference::Reference("surface_secondary"),
        hover_background: Preference::Reference("hover"),
        corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
        margin: Preference::Specific(Gaps::new_all(0.)),
        padding: Preference::Specific(Gaps::new(8., 12., 8., 12.)),
    },
    link: LinkThemePreference {
        color: Preference::Reference("secondary"),
    },
    tooltip: TooltipThemePreference {
        font_size: Preference::Specific(12.),
        background: Preference::Reference("surface_primary"),
        color: Preference::Reference("text_primary"),
        border_fill: Preference::Reference("border"),
    },
    circular_loader: CircularLoaderThemePreference {
        primary_color: Preference::Reference("primary"),
        inversed_color: Preference::Reference("text_secondary"),
    },
    input: InputColorsThemePreference {
        background: Preference::Reference("surface_tertiary"),
        hover_background: Preference::Reference("surface_primary"),
        color: Preference::Reference("text_primary"),
        placeholder_color: Preference::Reference("text_placeholder"),
        border_fill: Preference::Reference("border"),
        focus_border_fill: Preference::Reference("focus"),
    },
    radio: RadioItemThemePreference {
        unselected_fill: Preference::Reference("border"),
        selected_fill: Preference::Reference("primary"),
        border_fill: Preference::Reference("border"),
    },
    checkbox: CheckboxThemePreference {
        unselected_fill: Preference::Reference("border"),
        selected_fill: Preference::Reference("secondary"),
        selected_icon_fill: Preference::Reference("text_inverse"),
        border_fill: Preference::Reference("border"),
    },
    resizable_handle: ResizableHandleThemePreference {
        background: Preference::Reference("border"),
        hover_background: Preference::Reference("text_secondary"),
        corner_radius: Preference::Specific(CornerRadius::new_all(4.)),
    },
    floating_tab: FloatingTabThemePreference {
        corner_radius: Preference::Specific(CornerRadius::new_all(0.)),
        background: Preference::Specific(Color::TRANSPARENT),
        hover_background: Preference::Reference("hover"),
        color: Preference::Reference("text_primary"),
        padding: Preference::Specific(Gaps::new(8., 12., 8., 12.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
    },
    slider: SliderThemePreference {
        background: Preference::Reference("surface_primary"),
        thumb_background: Preference::Reference("primary"),
        thumb_inner_background: Preference::Reference("text_inverse"),
        border_fill: Preference::Reference("border"),
    },
    color_picker: ColorPickerThemePreference {
        background: Preference::Reference("surface_primary"),
        border_fill: Preference::Reference("border"),
        color: Preference::Reference("text_primary"),
    },
    select: SelectThemePreference {
        width: Preference::Specific(Size::Inner),
        margin: Preference::Specific(Gaps::new_all(0.)),
        select_background: Preference::Reference("surface_primary"),
        background_button: Preference::Reference("surface_tertiary"),
        hover_background: Preference::Reference("hover"),
        color: Preference::Reference("text_primary"),
        border_fill: Preference::Reference("border"),
        focus_border_fill: Preference::Reference("focus"),
        arrow_fill: Preference::Reference("text_primary"),
    },
    popup: PopupThemePreference {
        background: Preference::Reference("surface_primary"),
        color: Preference::Reference("text_primary"),
    },
    table: TableThemePreference {
        background: Preference::Specific(TABLE_ROW),
        arrow_fill: Preference::Reference("text_primary"),
        row_background: Preference::Specific(Color::TRANSPARENT),
        hover_row_background: Preference::Specific(Color::TRANSPARENT),
        divider_fill: Preference::Reference("border"),
        corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
        color: Preference::Reference("text_primary"),
    },
    chip: ChipThemePreference {
        background: Preference::Reference("surface_primary"),
        hover_background: Preference::Reference("hover"),
        selected_background: Preference::Reference("primary"),
        border_fill: Preference::Reference("border"),
        hover_border_fill: Preference::Reference("border"),
        selected_border_fill: Preference::Reference("primary"),
        focus_border_fill: Preference::Reference("focus"),
        padding: Preference::Specific(Gaps::new(6., 12., 6., 12.)),
        margin: Preference::Specific(0.),
        corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
        color: Preference::Reference("text_primary"),
        hover_color: Preference::Reference("text_primary"),
        selected_color: Preference::Reference("text_inverse"),
        selected_icon_fill: Preference::Reference("text_inverse"),
        hover_icon_fill: Preference::Reference("text_primary"),
    },
    menu_item: MenuItemThemePreference {
        background: Preference::Specific(Color::TRANSPARENT),
        hover_background: Preference::Reference("hover"),
        select_background: Preference::Reference("surface_secondary"),
        border_fill: Preference::Specific(Color::TRANSPARENT),
        select_border_fill: Preference::Reference("focus"),
        corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        color: Preference::Reference("text_primary"),
    },
    menu_container: MenuContainerThemePreference {
        background: Preference::Reference("surface_primary"),
        padding: Preference::Specific(Gaps::new_all(4.)),
        shadow: Preference::Reference("shadow"),
        border_fill: Preference::Reference("border"),
        corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
    },
    button_segment: ButtonSegmentThemePreference {
        background: Preference::Reference("surface_primary"),
        hover_background: Preference::Reference("hover"),
        disabled_background: Preference::Reference("disabled"),
        selected_background: Preference::Reference("surface_secondary"),
        focus_background: Preference::Reference("hover"),
        padding: Preference::Specific(Gaps::new(10., 16., 10., 16.)),
        selected_padding: Preference::Specific(Gaps::new(10., 16., 10., 16.)),
        width: Preference::Specific(Size::Inner),
        height: Preference::Specific(Size::Inner),
        color: Preference::Reference("text_primary"),
        selected_icon_fill: Preference::Reference("primary"),
    },
    segmented_button: SegmentedButtonThemePreference {
        background: Preference::Reference("surface_primary"),
        border_fill: Preference::Reference("border"),
        corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
    },
    input_layout: InputLayoutThemePreference {
        corner_radius: Preference::Specific(CornerRadius::new_all(8.)),
        inner_margin: Preference::Specific(Gaps::new(10., 12., 10., 12.)),
    },
    compact_input_layout: InputLayoutThemePreference {
        corner_radius: Preference::Specific(CornerRadius::new_all(6.)),
        inner_margin: Preference::Specific(Gaps::new(6., 8., 6., 8.)),
    },
    expanded_input_layout: InputLayoutThemePreference {
        corner_radius: Preference::Specific(CornerRadius::new_all(10.)),
        inner_margin: Preference::Specific(Gaps::new(14., 16., 14., 16.)),
    },
    filled_input: InputColorsThemePreference {
        background: Preference::Reference("primary"),
        hover_background: Preference::Specific(Color::from_rgb(37, 99, 235)),
        color: Preference::Reference("text_inverse"),
        placeholder_color: Preference::Specific(Color::from_argb(180, 255, 255, 255)),
        border_fill: Preference::Specific(Color::TRANSPARENT),
        focus_border_fill: Preference::Reference("primary"),
    },
    flat_input: InputColorsThemePreference {
        background: Preference::Specific(Color::TRANSPARENT),
        hover_background: Preference::Reference("hover"),
        color: Preference::Reference("text_primary"),
        placeholder_color: Preference::Reference("text_placeholder"),
        border_fill: Preference::Specific(Color::TRANSPARENT),
        focus_border_fill: Preference::Reference("focus"),
    },
    switch_layout: SwitchLayoutThemePreference {
        margin: Preference::Specific(Gaps::new_all(0.)),
        width: Preference::Specific(48.),
        height: Preference::Specific(28.),
        padding: Preference::Specific(4.),
        thumb_size: Preference::Specific(16.),
        toggled_thumb_size: Preference::Specific(20.),
        thumb_offset: Preference::Specific(2.),
        toggled_thumb_offset: Preference::Specific(20.),
    },
    expanded_switch_layout: SwitchLayoutThemePreference {
        margin: Preference::Specific(Gaps::new_all(0.)),
        width: Preference::Specific(56.),
        height: Preference::Specific(32.),
        padding: Preference::Specific(4.),
        thumb_size: Preference::Specific(18.),
        toggled_thumb_size: Preference::Specific(22.),
        thumb_offset: Preference::Specific(2.),
        toggled_thumb_offset: Preference::Specific(26.),
    },
};
