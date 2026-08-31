//! Shader Testing Tools Entry Point - Debug font loading
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;
use bevy::text::{TextFont, FontSize, FontSource, Font};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, debug_text_system, debug_node_system, debug_text2d_system, debug_font_assets))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    println!("DEBUG: Setup called");
    
    // Debug: Print available fonts
    println!("DEBUG: Loading default font handle");
    let default_font_handle: Handle<Font> = asset_server.load("embedded://FiraMono-subset.ttf");
    println!("DEBUG: Default font handle: {:?}", default_font_handle);
    
    // 3D Camera for the cube
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("3D Camera"),
    ));
    
    println!("DEBUG: Spawning 2D camera");
    
    // 2D Camera for UI overlay
    commands.spawn((
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Camera2d,
        Name::new("UI Camera"),
    ));
    
    println!("DEBUG: Spawning light");
    
    // Light
    commands.spawn((
        PointLight {
            intensity: 1000.0,
            ..default()
        },
        Transform::from_xyz(2.0, 2.0, 2.0),
        Name::new("Light"),
    ));
    
    println!("DEBUG: Spawning cube");
    
    // A cube with bright color
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Cube"),
    ));
    
    println!("DEBUG: Spawning UI");
    
    // Try Text2d with explicit default font handle
    commands.spawn((
        Text2d::new("TEXT2D - BRIGHT GREEN"),
        TextFont {
            font: FontSource::Handle(Handle::default()),
            font_size: FontSize::Px(128.0),
            ..default()
        },
        TextColor(Color::srgb(0.0, 1.0, 0.0)),
        Transform::from_xyz(0.0, 0.0, 10.0),
        Name::new("Text2d"),
    ));
    
    // Also try UI text with explicit default font handle
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            top: Val::Px(0.0),
            bottom: Val::Px(0.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.5, 0.0, 0.0)),
        Name::new("Fullscreen UI Panel"),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("UI TEXT - BRIGHT YELLOW"),
            TextFont {
                font: FontSource::Handle(Handle::default()),
                font_size: FontSize::Px(128.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
            Name::new("UI Text"),
        ));
    });
    
    println!("DEBUG: Setup complete");
}

fn rotate_cube(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Mesh3d>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
    }
}

/// Debug system to print text entity info
fn debug_text_system(
    text_query: Query<&Text>,
) {
    let text_count = text_query.iter().count();
    println!("DEBUG: Found {} Text component(s)", text_count);
}

/// Debug system to print text2d entity info
fn debug_text2d_system(
    text2d_query: Query<&Text2d>,
) {
    let text2d_count = text2d_query.iter().count();
    println!("DEBUG: Found {} Text2d component(s)", text2d_count);
}

/// Debug system to print node info
fn debug_node_system(
    node_query: Query<&Node>,
) {
    let node_count = node_query.iter().count();
    println!("DEBUG: Found {} Node component(s)", node_count);
}

/// Debug system to print font assets
fn debug_font_assets(
    fonts: Res<Assets<Font>>,
) {
    let font_count = fonts.len();
    println!("DEBUG: Loaded {} font asset(s)", font_count);
    
    for (handle, font) in fonts.iter() {
        println!("DEBUG: Font handle: {:?}, data length: {} bytes", handle, font.data.len());
    }
}
