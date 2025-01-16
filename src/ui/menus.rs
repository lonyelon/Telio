use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use bevy_panorbit_camera::{
    PanOrbitCamera,
};
use bevy_egui::{
    egui,
    EguiContexts,
};

use crate::ui::OccupiedScreenSpace;
use crate::ui::TelescopeConfig;
use crate::ui::ViewConfig;
use crate::ui::OriginalCameraConfig;
use crate::ui::sphere::TopBarTab;
use crate::ui::sphere::TopBarTabOption;

pub fn egui_system(
    mut contexts: EguiContexts,

    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut telescope_config: ResMut<TelescopeConfig>,
    mut view_config: ResMut<ViewConfig>,
    original_camera_config: Res<OriginalCameraConfig>,

    mut camera_query: Query<&mut PanOrbitCamera>,

    mut top_bar: Local<TopBarTab>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .min_width(450.0)
        .show(ctx, |ui| {
            egui::TopBottomPanel::top("my_panel")
                .show_inside(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.selectable_value(&mut top_bar.tab, TopBarTabOption::Stars, "Stars");
                    ui.selectable_value(&mut top_bar.tab, TopBarTabOption::TelescopeControl, "Telescope control");
                });
            });

            match top_bar.tab {
                TopBarTabOption::Stars => {
                },
                TopBarTabOption::TelescopeControl => {
                    let mut radio = 0;

                    egui::Grid::new("my_grid")
                        .num_columns(3)
                        .spacing([20.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Telescope serial port:");
                            egui::ComboBox::from_label("")
                                .selected_text(format!("{radio:?}"))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut radio, 0, "First");
                                    ui.selectable_value(&mut radio, 1, "Second");
                                    ui.selectable_value(&mut radio, 2, "Third");
                                });
                            ui.add(egui::widgets::Button::new("Refresh"));
                            ui.end_row();

                            ui.label("SDR++ host:");
                            ui.add(egui::TextEdit::singleline(&mut telescope_config.sdrpp_url).interactive(true));
                            ui.add(egui::widgets::Button::new("Test"));
                            ui.end_row();

                            ui.label("Latitude");
                            ui.add(egui::DragValue::new(&mut telescope_config.latitude)
                                .speed(0.01)
                                .range(-180.0..=180.0),
                            );
                            ui.end_row();

                            ui.label("Longitude");
                            ui.add(egui::DragValue::new(&mut telescope_config.longitude)
                                .speed(0.01)
                                .range(-180.0..=180.0),
                            );
                            ui.end_row();
                    });
                },
            }
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(false)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(egui::widgets::Button::new("Reset view")).clicked() {
                    if let Ok(mut camera) = camera_query.get_single_mut() {
                        camera.target_radius = original_camera_config.radius;
                        camera.target_pitch = original_camera_config.pitch;
                        camera.target_yaw = original_camera_config.yaw;
                    }
                };
                if ui.add(egui::widgets::Button::new("Azimuthal grid")
                    .selected(view_config.show_azimuthal_grid))
                    .clicked() {
                    view_config.show_azimuthal_grid = !view_config.show_azimuthal_grid;
                };
                if ui.add(egui::widgets::Button::new("Equatorial grid")
                    .selected(view_config.show_equatorial_grid))
                    .clicked() {
                    view_config.show_equatorial_grid = !view_config.show_equatorial_grid;
                };

                let stop_time_button_name = if view_config.time_stopped {
                    "S"
                } else {
                    "R"
                };
                if ui.add(egui::widgets::Button::new(stop_time_button_name)
                    .selected(view_config.time_stopped))
                    .clicked() {
                    view_config.time_stopped = !view_config.time_stopped;
                };
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}

pub fn adjust_viewport_to_egui(
    occupied_screen_space: Res<OccupiedScreenSpace>,

    mut camera_query: Query<&mut Camera>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();

    if let Ok(mut camera) = camera_query.get_single_mut() {
        camera.viewport = Some(bevy::render::camera::Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(
                (window.width() - occupied_screen_space.right) as u32,
                (window.height() - occupied_screen_space.bottom) as u32,
            ),
            ..default()
        });
    }
}
