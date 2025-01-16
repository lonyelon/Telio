use bevy::{
    prelude::*,
    render::camera::ScalingMode,
};
use bevy_panorbit_camera::{
    PanOrbitCamera,
    PanOrbitCameraPlugin
};
use bevy_egui::{
    EguiPlugin
};

mod menus;
mod sphere;

use crate::ui::menus::egui_system;
use crate::ui::menus::adjust_viewport_to_egui;
use crate::ui::sphere::SkySpherePlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(DefaultPlugins)
            .add_plugins(EguiPlugin)
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(SkySpherePlugin)
            .insert_resource(ClearColor(Color::rgb(0.1059, 0.1059, 0.1059)))
            .init_resource::<TelescopeConfig>()
            .init_resource::<ViewConfig>()
            .init_resource::<OriginalCameraConfig>()
            .init_resource::<OccupiedScreenSpace>()
            .add_systems(Startup, (
                setup_camera,
            ))
            .add_systems(Update, (
                egui_system,
                adjust_viewport_to_egui,
            ));
    }
}

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
    right: f32,
    bottom: f32,
}

#[derive(Resource)]
struct TelescopeConfig {
    serial_path: String,
    sdrpp_url: String,
    latitude: f32,
    longitude: f32,
}

impl Default for TelescopeConfig {
    fn default() -> Self {
        TelescopeConfig {
            latitude: 42.5950581,
            longitude: -8.74306467245,
            sdrpp_url: "https://localhost:7777".to_string(),
            serial_path: "/dev/sTTY_ACM0".to_string(),
        }
    }
}

#[derive(Resource)]
struct ViewConfig {
    show_azimuthal_grid: bool,
    show_equatorial_grid: bool,
    time_stopped: bool,
}

impl Default for ViewConfig {
    fn default() -> Self {
        ViewConfig {
            show_azimuthal_grid: true,
            show_equatorial_grid: false,
            time_stopped: false,
        }
    }
}

#[derive(Resource)]
struct OriginalCameraConfig {
    radius: f32,
    pitch: f32,
    yaw: f32,
}

impl Default for OriginalCameraConfig {
    fn default() -> Self {
        Self {
            radius: 4.0,
            pitch: 20.0f32.to_radians(),
            yaw: -15.0f32.to_radians(),
        }
    }
}

fn setup_camera(
    mut commands: Commands,
    original_camera_config: Res<OriginalCameraConfig>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                viewport: Some(bevy::render::camera::Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(256, 256),
                    ..default()
                }),
                ..default()
            },
            projection: Projection::Orthographic(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(1.0),
                ..default()
            }),
            ..default()
        },
        PanOrbitCamera {
            pan_sensitivity: 0.0,
            //zoom_lower_limit: 1.5,
            zoom_upper_limit: Some(20.0),
            button_orbit: MouseButton::Right,
            radius: Some(original_camera_config.radius),
            pitch: Some(original_camera_config.pitch),
            yaw: Some(original_camera_config.yaw),
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
