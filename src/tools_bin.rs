//! Shader Testing Tools Entry Point - Test with ONLY Camera2d
//!
//! Run with: cargo run --bin shader-tools

use bevy::prelude::*;
use bevy::text::{TextFont, FontSize, FontSource, Font};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (debug_text_system, debug_node_system, debug_text2d_system, debug_font_assets))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("DEBUG: Setup called");
    
    // Debug: Print available fonts
    println!("DEBUG: Loading default font handle");
    let default_font_handle: Handle<Font> = asset_server.load("embedded://FiraMono-subset.ttf");
    println!("DEBUG: Default font handle: {:?}", default_font_handle);
    
    // ONLY use Camera2d - no 3D camera
    println!("DEBUG: Spawning 2D camera ONLY");
    commands.spawn((
        Camera2d,
        Name::new("2D Camera"),
    ));
    
    println!("DEBUG: Spawning UI text at center");
    
    // Spawn text at center of screen - simple as possible
    commands.spawn((
        Text2d::new("HELLO WORLD - BRIGHT WHITE"),
        TextFont {
            font: FontSource::Handle(Handle::default()),
            font_size: FontSize::Px(64.0),
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Text2d"),
    ));
    
    // Also try UI text
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(100.0),
            width: Val::Px(400.0),
            height: Val::Px(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.0, 0.5)),
        Name::new("UI Panel"),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("UI TEXT TEST"),
            TextFont {
                font: FontSource::Handle(Handle::default()),
                font_size: FontSize::Px(48.0),
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
            Name::new("UI Text"),
        ));
    });
    
    println!("DEBUG: Setup complete");
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
