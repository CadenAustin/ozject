#[derive(Debug)]
pub struct ObjContent{
    pub vertices: Vec<Vertex>,
    pub texture_coordinates: Vec<TextureCoordinate>,
    pub vertex_normals: Vec<VertexNormal>,
    pub parameter_space_vertices: Vec<ParameterSpaceVertex>,
}

impl Default for ObjContent {
    fn default() -> Self {
        ObjContent {
            vertices: Vec::new(),
            texture_coordinates: Vec::new(),
            vertex_normals: Vec::new(),
            parameter_space_vertices: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug)]
pub struct TextureCoordinate {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

#[derive(Debug)]
pub struct VertexNormal {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

#[derive(Debug)]
pub struct ParameterSpaceVertex {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

#[derive(Debug)]
pub struct Point {
    pub vertex: usize,
}


#[derive(Debug)]
pub struct Line {
    pub vertices: Vec<usize>,
    pub texture_coordinates: Vec<usize>,
}

#[derive(Debug)]
pub struct Face {
    pub vertices: Vec<usize>,
    pub texture_coordinates: Vec<usize>,
    pub vertex_normals: Vec<usize>,
}