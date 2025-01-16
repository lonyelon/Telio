use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_resource::PrimitiveTopology,
    },
    window::PrimaryWindow,
};
use bevy_panorbit_camera::{
    PanOrbitCamera,
};
use chrono::{
    Timelike,
    Datelike,
};

use crate::config;
use crate::ui::TelescopeConfig;
use crate::ui::ViewConfig;

pub struct SkySpherePlugin;

impl Plugin for SkySpherePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup_sky_sphere,
                setup_default_stars,
                setup_line,
                setup_telescope_floor,
            ))
            .add_systems(Update, (
                handle_star_clicks,
                handle_grid_visibility,
                (
                    handle_sky_rotation,
                    handle_stars_size,
                ).chain()
            ));
    }
}

fn setup_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let positions = Vec::from([
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0]
    ]);
    let indices = Vec::from([0, 1]);
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, bevy::render::render_asset::RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; positions.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
    mesh.insert_indices(Indices::U32(indices));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::linear_rgb(1.0, 0.0, 0.0).into(),
                emissive: Color::linear_rgb(0.2, 0.2, 0.5).into(),
                unlit: true,
                ..default()
            }),
            ..default()
        },
        Rotator,
    ));
}

fn setup_telescope_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let n_vertices = 24;
    let mut positions = Vec::new();
    let radius = 0.75;
    for j in 0..n_vertices {
        positions.push([0.0, 0.0, 0.0]);

        let theta = 2.0 * std::f32::consts::PI * j as f32 / n_vertices as f32;
        positions.push([
            radius * theta.cos(),
            0.0,
            radius * theta.sin(),
        ]);

        let theta = 2.0 * std::f32::consts::PI * (j + 1) as f32 / n_vertices as f32;
        positions.push([
            radius * theta.cos(),
            0.0,
            radius * theta.sin(),
        ]);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, bevy::render::render_asset::RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; positions.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::linear_rgba(0.0, 1.0, 0.0, 0.5).into(),
                emissive: Color::linear_rgb(0.2, 0.2, 0.5).into(),
                unlit: true,
                ..default()
            }),
            ..default()
        },
        Rotator,
    ));
}

fn setup_sky_sphere(
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let n_vertices = 100;
    let n_circles = 36;

    for circle in 0..=n_circles/2 {
        let mut positions = Vec::new();
        for vertex in 0..n_vertices {
            let phi = circle as f32 / n_circles as f32 * std::f32::consts::PI * 2.0;
            let theta = (vertex as f32 / n_vertices as f32 * 2.0 - 1.0) * std::f32::consts::PI;
            positions.push([
                phi.sin() * theta.cos(),
                phi.cos(),
                phi.sin() * theta.sin(),
            ]);
        }
        positions.push(positions[0]);
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, bevy::render::render_asset::RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; positions.len()]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: Color::linear_rgba(0.175, 0.175, 0.25, 0.1),
                    emissive: Color::linear_rgb(0.2, 0.2, 0.5).into(),
                    unlit: true,
                    ..default()
                }),
                ..default()
            },
            Rotator,
            Sky,
            EquatorialGrid,
        ));
    }

    for circle in 0..=n_circles/2 {
        let mut positions = Vec::new();
        for vertex in 0..n_vertices {
            let phi = circle as f32 / n_circles as f32 * std::f32::consts::PI * 2.0;
            let theta = (vertex as f32 / n_vertices as f32 * 2.0 - 1.0) * std::f32::consts::PI;
            positions.push([
                theta.sin() * phi.sin(),
                theta.cos(),
                theta.sin() * phi.cos(),
            ]);
        }
        positions.push(positions[0]);
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, bevy::render::render_asset::RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; positions.len()]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: Color::linear_rgb(0.175, 0.175, 0.25),
                    emissive: Color::linear_rgb(0.2, 0.2, 0.5).into(),
                    unlit: true,
                    ..default()
                }),
                ..default()
            },
            Rotator,
            Sky,
            EquatorialGrid,
        ));
    }

    for circle in 0..=n_circles/2 {
        let mut positions = Vec::new();
        for vertex in 0..n_vertices {
            let phi = circle as f32 / n_circles as f32 * std::f32::consts::PI * 2.0;
            let theta = (vertex as f32 / n_vertices as f32 * 2.0 - 1.0) * std::f32::consts::PI;
            positions.push([
                phi.sin() * theta.cos(),
                phi.cos(),
                phi.sin() * theta.sin(),
            ]);
        }
        positions.push(positions[0]);
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, bevy::render::render_asset::RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; positions.len()]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: Color::linear_rgb(0.8431372549, 0.6, 0.1294117647),
                    emissive: Color::linear_rgb(0.2, 0.2, 0.5).into(),
                    unlit: true,
                    ..default()
                }),
                ..default()
            },
            Rotator,
            AzimuthalGrid,
        ));
    }

    for circle in 0..=n_circles/2 {
        let mut positions = Vec::new();
        for vertex in 0..n_vertices {
            let phi = circle as f32 / n_circles as f32 * std::f32::consts::PI * 2.0;
            let theta = (vertex as f32 / n_vertices as f32 * 2.0 - 1.0) * std::f32::consts::PI;
            positions.push([
                theta.sin() * phi.sin(),
                theta.cos(),
                theta.sin() * phi.cos(),
            ]);
        }
        positions.push(positions[0]);
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, bevy::render::render_asset::RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; positions.len()]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color: Color::linear_rgb(0.8431372549, 0.6, 0.1294117647),
                    emissive: Color::linear_rgb(0.2, 0.2, 0.5).into(),
                    unlit: true,
                    ..default()
                }),
                ..default()
            },
            Rotator,
            AzimuthalGrid,
        ));
    }

    let positions = Vec::from([
        [0.0, 1.0, 0.0],
        [0.0, 1.25, 0.0],
    ]);
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, bevy::render::render_asset::RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; positions.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::linear_rgb(0.7, 0.7, 1.0),
                emissive: Color::linear_rgb(0.2, 0.2, 0.5).into(),
                unlit: true,
                ..default()
            }),
            ..default()
        },
        Rotator,
        Sky,
        EquatorialGrid,
    ));

    let positions = Vec::from([
        [-1.0, 0.0, 0.0],
        [-1.25, 0.0, 0.0],
    ]);
    let mut mesh = Mesh::new(PrimitiveTopology::LineStrip, bevy::render::render_asset::RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[1.0, 0.0, 0.0]; positions.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; positions.len()]);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::linear_rgb(0.7, 0.7, 1.0),
                emissive: Color::linear_rgb(0.2, 0.2, 0.5).into(),
                unlit: true,
                ..default()
            }),
            ..default()
        },
        Rotator,
        Sky,
        EquatorialGrid,
    ));
}

#[derive(Resource)]
struct TimeTracker {
    start: bevy::utils::Instant,
}

impl Default for TimeTracker {
    fn default() -> Self {
        TimeTracker {
            start: bevy::utils::Instant::now()
        }
    }
}

fn handle_star_clicks(
    mouse_button_input: Res<ButtonInput<MouseButton>>,

    mut panorbit_camera_query: Query<&mut PanOrbitCamera>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    star_query: Query<(Entity, &GlobalTransform, &Star)>,

    mut double_click_time: Local<TimeTracker>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let time_since_last_click = double_click_time.start.elapsed().as_millis();
        double_click_time.start = bevy::utils::Instant::now();

        let window = windows.single();
        if let Some(cursor_position) = window.cursor_position() {
            let (camera, camera_transform) = camera.single();

            if let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                for (_entity, transform, star) in star_query.iter() {
                    let star_pos = transform.translation();
                    let distance = ray.direction.cross(star_pos - ray.origin).length();

                    if distance < 0.05 {
                        println!("Star {} {} {}", star.name, star.right_ascension, star.declination);
                        if time_since_last_click < 200 {
                            if let Ok(mut panorbit_camera) = panorbit_camera_query.get_single_mut() {
                                let position = transform.translation();
                                panorbit_camera.target_yaw = position.x.atan2(position.z);
                                panorbit_camera.target_pitch = position.y.asin();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn handle_grid_visibility(
    view_config: Res<ViewConfig>,

    mut equatorial_grid_query: Query<&mut Visibility, (With<EquatorialGrid>, Without<AzimuthalGrid>)>,
    mut azimuthal_grid_query: Query<&mut Visibility, (With<AzimuthalGrid>, Without<EquatorialGrid>)>,
) {
    for mut visibility in &mut equatorial_grid_query {
        *visibility = if view_config.show_equatorial_grid {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }

    for mut visibility in &mut azimuthal_grid_query {
        *visibility = if view_config.show_azimuthal_grid {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }
}

#[derive(PartialEq)]
pub enum TopBarTabOption {
    Stars,
    TelescopeControl,
}

#[derive(Resource, PartialEq)]
pub struct TopBarTab {
    pub tab: TopBarTabOption,
}

impl Default for TopBarTab {
    fn default() -> Self {
        TopBarTab {
            tab: TopBarTabOption::Stars,
        }
    }
}

fn handle_stars_size(
    mut star_query: Query<&mut Transform, With<Star>>,
    camera_query: Query<&PanOrbitCamera>,
) {
    for mut star_transform in star_query.iter_mut() {
        if let Ok(camera) = camera_query.get_single() {
            let radius = 1.5*camera.radius.unwrap()/5.0;
            if radius < 1.5 {
                star_transform.scale = Vec3::splat(radius);
            } else {
                star_transform.scale = Vec3::splat(1.5);
            }
        }
    }
}

#[derive(Component)]
struct Sky;

#[derive(Component)]
struct Rotator;

#[derive(Component)]
struct AzimuthalGrid;

#[derive(Component)]
struct EquatorialGrid;

#[derive(Component)]
struct Star {
    name: String,
    right_ascension: f32,
    declination: f32,
}

/// Rotate sky to match current location and time.
fn handle_sky_rotation(
    telescope_config: Res<TelescopeConfig>,
    view_config: Res<ViewConfig>,

    mut sky_sphere_query: Query<&mut Transform, (With<Sky>, Without<Star>)>,
    mut sky_star_query: Query<(&mut Transform, &Star), With<Star>>,
) {
    if !view_config.time_stopped {
        let now = chrono::prelude::Local::now();
        let astro_time = astronav::time::AstroTime {
            day: now.day() as u8,
            month: now.month() as u8,
            year: now.year() as u16,
            hour: now.hour() as u8,
            min: now.minute() as u8,
            sec: now.second() as u8,
            timezone: now.offset().local_minus_utc() as f32 / 3600.0,
        };
        let lmst = astro_time.lmst_in_degrees(telescope_config.longitude as f64);
        let generic_alt_az = astronav::coords::star::AltAzBuilder::new()
                .lat(telescope_config.latitude as f64)
                .lmst(lmst);

        let sky_alt_az = generic_alt_az.clone()
                .dec(90.0f64)
                .ra(0.0f64)
                .seal()
                .build();
        let sky_alt_az_2 = generic_alt_az.clone()
                .dec(0.0f64)
                .ra(0.0f64)
                .seal()
                .build();

        for mut transform in &mut sky_sphere_query {
            let model_fix = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
            let mut new_transform = Transform::from_xyz(0.0, 0.0, 0.0);

            let alt_rad = -sky_alt_az.get_altitude().to_radians() as f32;
            let rotation_z = Quat::from_rotation_z(alt_rad);

            let az_rad  = -sky_alt_az_2.get_azimuth().to_radians() as f32;
            let rotation_y = Quat::from_rotation_y(az_rad);

            new_transform.rotate_around(Vec3::ZERO,
                                        model_fix * rotation_z * rotation_y);

            *transform = new_transform;
        }


        for (mut transform, star) in &mut sky_star_query {
            let alt_az = generic_alt_az.clone()
                .dec(star.declination as f64)
                .ra(star.right_ascension as f64)
                .seal()
                .build();

            let mut new_transform = Transform::from_xyz(-1.0, 0.0, 0.0);

            let alt_rad = -alt_az.get_altitude().to_radians() as f32;
            let rotation_z = Quat::from_rotation_z(alt_rad);

            let az_rad  = -alt_az.get_azimuth().to_radians() as f32;
            let rotation_y = Quat::from_rotation_y(az_rad);

            new_transform.rotate_around(Vec3::ZERO, rotation_y * rotation_z);

            *transform = new_transform;
        }
    }
}

/// Add default stars to the system, so the software can display something.
fn setup_default_stars(
    client_config: Res<config::ClientConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut commands: Commands,
) {
    for star in &client_config.stars {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(bevy::math::primitives::Sphere {
                    radius: 0.05,
                }),
                material: materials.add(StandardMaterial {
                    base_color: Color::linear_rgb(1.0, 0.7, 0.5),
                    emissive: Color::linear_rgba(1.0, 1.0, 0.0, 1.0).into(),
                    unlit: true,
                    ..default()
                }),
                transform: Transform::from_xyz(1.0, 0.0, 0.0),
                ..default()
            },
            Star {
                name: star.name.clone(),
                right_ascension: star.ra,
                declination: star.dec,
            },
            Rotator,
            Sky,
        ));
    }
}
