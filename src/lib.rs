#![allow(unused)]

use egui::epaint::Shadow;
use egui::style::{Selection, WidgetVisuals, Widgets};
use egui::{Color32, Context, Stroke, Visuals};

// https://spec.draculatheme.com/#sec-Standard
const BACKGROUND: Color32 = Color32::from_rgb(0x28, 0x2A, 0x36);
const FOREGROUND: Color32 = Color32::from_rgb(0xF8, 0xF8, 0xF2);
const SELECTION: Color32 = Color32::from_rgb(0x44, 0x47, 0x5A);
const COMMENT: Color32 = Color32::from_rgb(0x62, 0x72, 0xA4);
const RED: Color32 = Color32::from_rgb(0xFF, 0x55, 0x55);
const ORANGE: Color32 = Color32::from_rgb(0xFF, 0xB8, 0x6C);
const YELLOW: Color32 = Color32::from_rgb(0xF1, 0xFA, 0x8C);
const GREEN: Color32 = Color32::from_rgb(0x50, 0xFA, 0x7B);
const PURPLE: Color32 = Color32::from_rgb(0xBD, 0x93, 0xF9);
const CYAN: Color32 = Color32::from_rgb(0x8B, 0xE9, 0xFD);
const PINK: Color32 = Color32::from_rgb(0xFF, 0x79, 0xC6);

// https://spec.draculatheme.com/#sec-ANSI
const ANSI_BLACK: Color32 = Color32::from_rgb(0x21, 0x22, 0x2C);
const ANSI_RED: Color32 = Color32::from_rgb(0xFF, 0x55, 0x55);
const ANSI_GREEN: Color32 = Color32::from_rgb(0x50, 0xFA, 0x7B);
const ANSI_YELLOW: Color32 = Color32::from_rgb(0xF1, 0xFA, 0x8C);
const ANSI_BLUE: Color32 = Color32::from_rgb(0xBD, 0x93, 0xF9);
const ANSI_MAGENTA: Color32 = Color32::from_rgb(0xFF, 0x79, 0xC6);
const ANSI_CYAN: Color32 = Color32::from_rgb(0x8B, 0xE9, 0xFD);
const ANSI_WHITE: Color32 = Color32::from_rgb(0xF8, 0xF8, 0xF2);
const ANSI_BRIGHT_BLACK: Color32 = Color32::from_rgb(0x62, 0x72, 0xA4);
const ANSI_BRIGHT_RED: Color32 = Color32::from_rgb(0xFF, 0x6E, 0x6E);
const ANSI_BRIGHT_GREEN: Color32 = Color32::from_rgb(0x69, 0xFF, 0x94);
const ANSI_BRIGHT_YELLOW: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xA5);
const ANSI_BRIGHT_BLUE: Color32 = Color32::from_rgb(0xD6, 0xAC, 0xFF);
const ANSI_BRIGHT_MAGENTA: Color32 = Color32::from_rgb(0xFF, 0x92, 0xDF);
const ANSI_BRIGHT_CYAN: Color32 = Color32::from_rgb(0xA4, 0xFF, 0xFF);
const ANSI_BRIGHT_WHITE: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);

// Non-spec colors
// https://github.com/dracula/visual-studio-code/blob/5bf7bc071ba927cd5461b739bcaf6304c6813830/src/dracula.yml#L47-L51
const BACKGROUND_LIGHTER: Color32 = Color32::from_rgb(0x42, 0x44, 0x50);
const BACKGROUND_LIGHT: Color32 = Color32::from_rgb(0x34, 0x37, 0x46);
const BACKGROUND_DARK: Color32 = Color32::from_rgb(0x21, 0x22, 0x2C);
const BACKGROUND_DARKER: Color32 = Color32::from_rgb(0x19, 0x1A, 0x21);

pub fn set_theme(ctx: &Context) {
    let default = ctx.style().visuals.clone();

    ctx.set_visuals(Visuals {
        dark_mode: true,
        widgets: Widgets {
            noninteractive: WidgetVisuals {
                bg_fill: BACKGROUND,
                weak_bg_fill: BACKGROUND,
                bg_stroke: Stroke {
                    color: BACKGROUND_DARK,
                    ..default.widgets.noninteractive.bg_stroke
                },
                fg_stroke: Stroke {
                    color: FOREGROUND,
                    ..default.widgets.noninteractive.fg_stroke
                },
                ..default.widgets.noninteractive
            },
            inactive: WidgetVisuals {
                bg_fill: BACKGROUND_LIGHTER,
                weak_bg_fill: BACKGROUND_LIGHT,
                bg_stroke: Stroke {
                    color: BACKGROUND_DARK,
                    ..default.widgets.inactive.bg_stroke
                },
                fg_stroke: Stroke {
                    color: FOREGROUND,
                    ..default.widgets.inactive.fg_stroke
                },
                ..default.widgets.inactive
            },
            hovered: WidgetVisuals {
                bg_fill: BACKGROUND_LIGHTER,
                weak_bg_fill: BACKGROUND_LIGHTER,
                bg_stroke: Stroke {
                    color: BACKGROUND_DARK,
                    ..default.widgets.hovered.bg_stroke
                },
                fg_stroke: Stroke {
                    color: FOREGROUND,
                    ..default.widgets.hovered.fg_stroke
                },
                ..default.widgets.hovered
            },
            active: WidgetVisuals {
                bg_fill: BACKGROUND_LIGHTER,
                weak_bg_fill: BACKGROUND_LIGHT,
                bg_stroke: Stroke {
                    color: BACKGROUND_DARK,
                    ..default.widgets.active.bg_stroke
                },
                fg_stroke: Stroke {
                    color: FOREGROUND,
                    ..default.widgets.active.fg_stroke
                },
                ..default.widgets.active
            },
            open: WidgetVisuals {
                bg_fill: BACKGROUND_LIGHTER,
                weak_bg_fill: BACKGROUND_LIGHT,
                bg_stroke: Stroke {
                    color: BACKGROUND_DARK,
                    ..default.widgets.open.bg_stroke
                },
                fg_stroke: Stroke {
                    color: FOREGROUND,
                    ..default.widgets.open.fg_stroke
                },
                ..default.widgets.open
            },
        },
        selection: Selection {
            bg_fill: SELECTION,
            stroke: Stroke {
                color: BACKGROUND_DARK,
                ..default.selection.stroke
            },
        },
        hyperlink_color: PINK,
        faint_bg_color: BACKGROUND_DARK,
        extreme_bg_color: BACKGROUND_DARKER,
        code_bg_color: BACKGROUND_DARKER,
        error_fg_color: RED,
        warn_fg_color: ORANGE,
        window_shadow: Shadow {
            color: BACKGROUND_DARKER,
            ..default.window_shadow
        },
        window_fill: BACKGROUND,
        window_stroke: Stroke {
            color: BACKGROUND_DARK,
            ..default.window_stroke
        },
        panel_fill: BACKGROUND_DARK,
        popup_shadow: Shadow {
            color: BACKGROUND_DARK,
            ..default.popup_shadow
        },
        ..default
    });
}
