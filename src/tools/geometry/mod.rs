//! Geometry utilities for shader testing

use bevy::prelude::*;
use bevy::math::primitives::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;

/// Create a circle mesh
pub fn create_circle_mesh(radius: f32, segments: usize) -> Mesh {
    let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    
    let center = [0.0, 0.0, 0.0];
    let mut positions = vec![center];
    let mut uvs = vec![[0.5, 0.5]];
    let mut normals = vec![[0.0, 0.0, 1.0]];
    
    for i in 0..=segments {
        let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        positions.push([x, y, 0.0]);
        uvs.push([0.5 + 0.5 * angle.cos(), 0.5 + 0.5 * angle.sin()]);
        normals.push([0.0, 0.0, 1.0]);
    }
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    
    mesh
}

/// Create a triangle mesh
pub fn create_triangle_mesh() -> Mesh {
    let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    
    let vertices = vec![
        ([0.0, 1.0, 0.0], [0.5, 1.0]),
        ([-1.0, -1.0, 0.0], [0.0, 0.0]),
        ([1.0, -1.0, 0.0], [1.0, 0.0]),
    ];
    
    let positions: Vec<[f32; 3]> = vertices.iter().map(|(pos, _)| *pos).collect();
    let uvs: Vec<[f32; 2]> = vertices.iter().map(|(_, uv)| *uv).collect();
    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 3];
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    
    mesh
}

/// Component to mark geometry as a test geometry
#[derive(Component, Debug)]
pub struct TestGeometry;

/// Create a simple circle component
#[derive(Component, Debug)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

impl From<Circle> for Mesh {
    fn from(circle: Circle) -> Self {
        create_circle_mesh(circle.radius, 32)
    }
}
