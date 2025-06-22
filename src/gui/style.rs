use egui::{Color32, CornerRadius, Stroke, Style};

const R: u8 = 8;
const CORNER_RADIUS: CornerRadius = CornerRadius {
    nw: R,
    ne: R,
    sw: R,
    se: R,
};
const EXPANSION: f32 = 2.0;

pub struct ColorScheme {
    pub bg1: Color32,
    pub bg2: Color32,

    pub hover_mute: Color32,
    pub stroke_mute: Color32,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            bg1: Color32::from_hex("#09090b").unwrap(),
            bg2: Color32::from_hex("#0c0c0e").unwrap(),

            hover_mute: Color32::from_hex("#1f1f24").unwrap(),
            stroke_mute: Color32::from_hex("#1f1f24").unwrap(),
        }
    }
}

pub fn apply_style(style: &mut Style) {
    let colors = ColorScheme::default();

    style.visuals.window_fill = colors.bg1;
    style.visuals.panel_fill = colors.bg1;
    //style.visuals.w

    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    //style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    style.visuals.widgets.noninteractive.bg_stroke.color = colors.stroke_mute;
    //style.visuals.widgets.open.bg_stroke = Stroke::NONE;

    //style.visuals.widgets.active.weak_bg_fill = ;
    style.visuals.widgets.hovered.weak_bg_fill = colors.hover_mute;
    //style.visuals.widgets.inactive.weak_bg_fill = ;
    //style.visuals.widgets.noninteractive.weak_bg_fill = ;
    //style.visuals.widgets.open.weak_bg_fill = ;

    style.visuals.widgets.active.corner_radius = CORNER_RADIUS;
    style.visuals.widgets.hovered.corner_radius = CORNER_RADIUS;
    style.visuals.widgets.inactive.corner_radius = CORNER_RADIUS;
    style.visuals.widgets.noninteractive.corner_radius = CORNER_RADIUS;
    style.visuals.widgets.open.corner_radius = CORNER_RADIUS;

    style.visuals.widgets.active.expansion = EXPANSION;
    style.visuals.widgets.hovered.expansion = EXPANSION;
    style.visuals.widgets.inactive.expansion = EXPANSION;
    style.visuals.widgets.noninteractive.expansion = EXPANSION;
    style.visuals.widgets.open.expansion = EXPANSION;

    //Visuals {
    //    dark_mode: true,
    //    // override_text_color: (),
    //    widgets: Widgets {
    //        //noninteractive: WidgetVisuals {
    //        //    bg_fill: todo!(),
    //        //    weak_bg_fill: todo!(),
    //        //    bg_stroke: Stroke::NONE,
    //        //    corner_radius: CORNER_RADIUS,
    //        //    fg_stroke: Stroke::NONE,
    //        //    expansion: EXPANSION,
    //        //},
    //        inactive: WidgetVisuals {
    //            bg_fill: Color32::DARK_BLUE,
    //            weak_bg_fill: Color32::TRANSPARENT,
    //            bg_stroke: Stroke::NONE,
    //            corner_radius: CORNER_RADIUS,
    //            fg_stroke: Stroke::NONE,
    //            expansion: EXPANSION,
    //        },
    //        hovered: WidgetVisuals {
    //            bg_fill: Color32::LIGHT_BLUE,
    //            weak_bg_fill: Color32::GRAY,
    //            bg_stroke: Stroke::NONE,
    //            corner_radius: CORNER_RADIUS,
    //            fg_stroke: Stroke::NONE,
    //            expansion: EXPANSION,
    //        },
    //        active: WidgetVisuals {
    //            bg_fill: Color32::BLUE,
    //            weak_bg_fill: Color32::DARK_GRAY,
    //            bg_stroke: Stroke::NONE,
    //            corner_radius: CORNER_RADIUS,
    //            fg_stroke: Stroke::NONE,
    //            expansion: EXPANSION,
    //        },
    //        /*
    //        open: WidgetVisuals {
    //            bg_fill: todo!(),
    //            weak_bg_fill: todo!(),
    //            bg_stroke: Stroke::NONE,
    //            corner_radius: CORNER_RADIUS,
    //            fg_stroke: Stroke::NONE,
    //            expansion: EXPANSION,
    //        },*/
    //        ..Default::default()
    //    },
    //    // selection: (),
    //    // hyperlink_color: (),
    //    // faint_bg_color: (),
    //    // extreme_bg_color: (),
    //    // code_bg_color: (),
    //    // warn_fg_color: (),
    //    // error_fg_color: (),
    //    // window_corner_radius: (),
    //    // window_shadow: (),
    //    // window_fill: (),
    //    // window_stroke: (),
    //    // window_highlight_topmost: (),
    //    // menu_corner_radius: (),
    //    // panel_fill: (),
    //    // popup_shadow: (),
    //    // resize_corner_size: (),
    //    // text_cursor: (),
    //    // clip_rect_margin: (),
    //    // button_frame: (),
    //    // collapsing_header_frame: (),
    //    // indent_has_left_vline: (),
    //    // striped: (),
    //    // slider_trailing_fill: (),
    //    // handle_shape: (),
    //    // interact_cursor: (),
    //    // image_loading_spinners: (),
    //    // numeric_color_space: (),
    //    ..Default::default()
}
