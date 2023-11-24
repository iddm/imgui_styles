//! The Immediate GUI style themes. Supports
//! [ImGUI](https://github.com/imgui-rs/imgui-rs), but in the future
//! there are plans to also support
//! [eGUI](https://github.com/emilk/egui).
//!
//! Just obtain the theme object and use it! For example:
//!
//! ```rust,ignore
//! let mut imgui_context = ...; // obtain imgui::Context
//! imgui_styles::embrace_the_darkness::context_patch(&mut imgui_context);
//! ```

/// Published by `janekb04` on
/// <https://github.com/ocornut/imgui/issues/707>.
pub mod embrace_the_darkness {
    const FONT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fonts/roboto/Roboto-Regular.ttf"
    ));

    /// Patches the passed style.
    pub fn style_patch(style: &mut imgui::Style) {
        let colors = &mut style.colors;

        colors[imgui::StyleColor::Text as usize] = [1.00f32, 1.00f32, 1.00f32, 1.00f32];
        colors[imgui::StyleColor::TextDisabled as usize] = [0.50f32, 0.50f32, 0.50f32, 1.00f32];
        colors[imgui::StyleColor::WindowBg as usize] = [0.10f32, 0.10f32, 0.10f32, 1.00f32];
        colors[imgui::StyleColor::ChildBg as usize] = [0.00f32, 0.00f32, 0.00f32, 0.00f32];
        colors[imgui::StyleColor::PopupBg as usize] = [0.19f32, 0.19f32, 0.19f32, 0.92f32];
        colors[imgui::StyleColor::Border as usize] = [0.19f32, 0.19f32, 0.19f32, 0.29f32];
        colors[imgui::StyleColor::BorderShadow as usize] = [0.00f32, 0.00f32, 0.00f32, 0.24f32];
        colors[imgui::StyleColor::FrameBg as usize] = [0.05f32, 0.05f32, 0.05f32, 0.54f32];
        colors[imgui::StyleColor::FrameBgHovered as usize] = [0.19f32, 0.19f32, 0.19f32, 0.54f32];
        colors[imgui::StyleColor::FrameBgActive as usize] = [0.20f32, 0.22f32, 0.23f32, 1.00f32];
        colors[imgui::StyleColor::TitleBg as usize] = [0.00f32, 0.00f32, 0.00f32, 1.00f32];
        colors[imgui::StyleColor::TitleBgActive as usize] = [0.06f32, 0.06f32, 0.06f32, 1.00f32];
        colors[imgui::StyleColor::TitleBgCollapsed as usize] = [0.00f32, 0.00f32, 0.00f32, 1.00f32];
        colors[imgui::StyleColor::MenuBarBg as usize] = [0.14f32, 0.14f32, 0.14f32, 1.00f32];
        colors[imgui::StyleColor::ScrollbarBg as usize] = [0.05f32, 0.05f32, 0.05f32, 0.54f32];
        colors[imgui::StyleColor::ScrollbarGrab as usize] = [0.34f32, 0.34f32, 0.34f32, 0.54f32];
        colors[imgui::StyleColor::ScrollbarGrabHovered as usize] =
            [0.40f32, 0.40f32, 0.40f32, 0.54f32];
        colors[imgui::StyleColor::ScrollbarGrabActive as usize] =
            [0.56f32, 0.56f32, 0.56f32, 0.54f32];
        colors[imgui::StyleColor::CheckMark as usize] = [0.33f32, 0.67f32, 0.86f32, 1.00f32];
        colors[imgui::StyleColor::SliderGrab as usize] = [0.34f32, 0.34f32, 0.34f32, 0.54f32];
        colors[imgui::StyleColor::SliderGrabActive as usize] = [0.56f32, 0.56f32, 0.56f32, 0.54f32];
        colors[imgui::StyleColor::Button as usize] = [0.05f32, 0.05f32, 0.05f32, 0.54f32];
        colors[imgui::StyleColor::ButtonHovered as usize] = [0.19f32, 0.19f32, 0.19f32, 0.54f32];
        colors[imgui::StyleColor::ButtonActive as usize] = [0.20f32, 0.22f32, 0.23f32, 1.00f32];
        colors[imgui::StyleColor::Header as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
        colors[imgui::StyleColor::HeaderHovered as usize] = [0.00f32, 0.00f32, 0.00f32, 0.36f32];
        colors[imgui::StyleColor::HeaderActive as usize] = [0.20f32, 0.22f32, 0.23f32, 0.33f32];
        colors[imgui::StyleColor::Separator as usize] = [0.28f32, 0.28f32, 0.28f32, 0.29f32];
        colors[imgui::StyleColor::SeparatorHovered as usize] = [0.44f32, 0.44f32, 0.44f32, 0.29f32];
        colors[imgui::StyleColor::SeparatorActive as usize] = [0.40f32, 0.44f32, 0.47f32, 1.00f32];
        colors[imgui::StyleColor::ResizeGrip as usize] = [0.28f32, 0.28f32, 0.28f32, 0.29f32];
        colors[imgui::StyleColor::ResizeGripHovered as usize] =
            [0.44f32, 0.44f32, 0.44f32, 0.29f32];
        colors[imgui::StyleColor::ResizeGripActive as usize] = [0.40f32, 0.44f32, 0.47f32, 1.00f32];
        colors[imgui::StyleColor::Tab as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
        colors[imgui::StyleColor::TabHovered as usize] = [0.14f32, 0.14f32, 0.14f32, 1.00f32];
        colors[imgui::StyleColor::TabActive as usize] = [0.20f32, 0.20f32, 0.20f32, 0.36f32];
        colors[imgui::StyleColor::TabUnfocused as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
        colors[imgui::StyleColor::TabUnfocusedActive as usize] =
            [0.14f32, 0.14f32, 0.14f32, 1.00f32];
        colors[imgui::StyleColor::DockingPreview as usize] = [0.33f32, 0.67f32, 0.86f32, 1.00f32];
        colors[imgui::StyleColor::DockingEmptyBg as usize] = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
        colors[imgui::StyleColor::PlotLines as usize] = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
        colors[imgui::StyleColor::PlotLinesHovered as usize] = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
        colors[imgui::StyleColor::PlotHistogram as usize] = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
        colors[imgui::StyleColor::PlotHistogramHovered as usize] =
            [1.00f32, 0.00f32, 0.00f32, 1.00f32];
        colors[imgui::StyleColor::TableHeaderBg as usize] = [0.00f32, 0.00f32, 0.00f32, 0.52f32];
        colors[imgui::StyleColor::TableBorderStrong as usize] =
            [0.00f32, 0.00f32, 0.00f32, 0.52f32];
        colors[imgui::StyleColor::TableBorderLight as usize] = [0.28f32, 0.28f32, 0.28f32, 0.29f32];
        colors[imgui::StyleColor::TableRowBg as usize] = [0.00f32, 0.00f32, 0.00f32, 0.00f32];
        colors[imgui::StyleColor::TableRowBgAlt as usize] = [1.00f32, 1.00f32, 1.00f32, 0.06f32];
        colors[imgui::StyleColor::TextSelectedBg as usize] = [0.20f32, 0.22f32, 0.23f32, 1.00f32];
        colors[imgui::StyleColor::DragDropTarget as usize] = [0.33f32, 0.67f32, 0.86f32, 1.00f32];
        colors[imgui::StyleColor::NavHighlight as usize] = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
        colors[imgui::StyleColor::NavWindowingHighlight as usize] =
            [1.00f32, 0.00f32, 0.00f32, 0.70f32];
        colors[imgui::StyleColor::NavWindowingDimBg as usize] =
            [1.00f32, 0.00f32, 0.00f32, 0.20f32];
        colors[imgui::StyleColor::ModalWindowDimBg as usize] = [1.00f32, 0.00f32, 0.00f32, 0.35f32];

        style.window_padding = [8.00f32, 8.00f32];
        style.frame_padding = [5.00f32, 2.00f32];
        style.frame_rounding = 6.00f32;
        style.item_spacing = [6.00f32, 6.00f32];
        style.item_inner_spacing = [6.00f32, 6.00f32];
        style.touch_extra_padding = [0.00f32, 0.00f32];
        style.indent_spacing = 25.0f32;
        style.scrollbar_size = 15.0f32;
        style.grab_min_size = 10.0f32;
        style.window_border_size = 1.0f32;
        style.child_border_size = 1.0f32;
        style.popup_border_size = 1.0f32;
        style.frame_border_size = 1.0f32;
        style.tab_border_size = 1.0f32;
        style.window_rounding = 7.0f32;
        style.child_rounding = 4.0f32;
        style.frame_rounding = 3.0f32;
        style.popup_rounding = 4.0f32;
        style.scrollbar_rounding = 9.0f32;
        style.grab_rounding = 3.0f32;
        style.log_slider_deadzone = 4.0f32;
        style.tab_rounding = 4.0f32;
    }

    /// Adds the font to the font atlas and sets it as default.
    pub fn font_patch(context: &mut imgui::Context) {
        let source = [imgui::FontSource::TtfData {
            data: FONT,
            size_pixels: 16.0f32,
            config: None,
        }];
        context.fonts().clear();
        context.fonts().add_font(source.as_slice());
    }

    /// Creates a new style object.
    pub fn new() -> imgui::Style {
        let mut style: imgui::Style = unsafe { std::mem::zeroed() };
        style_patch(&mut style);
        style
    }

    /// Patches the style for the entire context.
    pub fn context_patch(context: &mut imgui::Context) {
        style_patch(context.style_mut());
        font_patch(context);
    }
}
