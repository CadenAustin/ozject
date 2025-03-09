use crate::types::{Face, ObjContent, ParameterSpaceVertex, TextureCoordinate, Vertex, VertexNormal};
use std::fs;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
pub struct Parser {
    obj_content: ObjContent,
    file_buffer: BufReader<File>,
}

impl Parser {
    pub fn new(file_name: String) -> Parser {
        let f = File::open(file_name).expect("file not found");
        let mut reader = BufReader::new(f);

        Parser {
            obj_content: ObjContent::default(),
            file_buffer: reader,
        }
    }

    pub fn parse(&mut self) {
        let mut line = String::new();
        loop {
            let bytes_read = self.file_buffer.read_line(&mut line).expect("error reading line");
            if bytes_read == 0 {
                break;
            }
            self.parse_line(&line);
            line.clear();
        }

        debug_println!("{:?}", self.obj_content);
    }

    pub fn parse_line(&mut self, line: &String) {
        let mut tokens = line.split_whitespace();
        
        match tokens.next() {
            Some("#") => {
                debug_println!("Comment: {}", line);
            },
            Some("v") => {
                debug_println!("Vertex: {}", line);
                let x: f32 = tokens.next().unwrap().parse().unwrap();
                let y: f32 = tokens.next().unwrap().parse().unwrap();
                let z: f32 = tokens.next().unwrap().parse().unwrap();
                let w: f32 = tokens.next().unwrap_or("1.0").parse().unwrap();
                self.obj_content.vertices.push(Vertex { x, y, z, w });
            },
            Some("vt") => {
                debug_println!("Texture Coordinate: {}", line);
                let u: f32 = tokens.next().unwrap().parse().unwrap();
                let v: f32 = tokens.next().unwrap_or("0.0").parse().unwrap();
                let w: f32 = tokens.next().unwrap_or("0.0").parse().unwrap();
                self.obj_content.texture_coordinates.push(TextureCoordinate { u, v, w });
            },
            Some("vn") => {
                debug_println!("Vertex Normal: {}", line);
                let i: f32 = tokens.next().unwrap().parse().unwrap();
                let j: f32 = tokens.next().unwrap().parse().unwrap();
                let k: f32 = tokens.next().unwrap().parse().unwrap();
                self.obj_content.vertex_normals.push(VertexNormal { i, j, k });
            },
            Some("vp") => {
                debug_println!("Parameter Space Vertex: {}", line);
                let u: f32 = tokens.next().unwrap().parse().unwrap();
                let v: f32 = tokens.next().unwrap().parse().unwrap();
                let w: f32 = tokens.next().unwrap_or("1.0").parse().unwrap();
                self.obj_content.parameter_space_vertices.push(ParameterSpaceVertex { u, v, w });
            },
            Some("f") => {
                debug_println!("Face: {}", line);
                loop {
                    let face_index = tokens.next();
                    if face_index.is_none() {
                        break;
                    }
                    let mut indexs = face_index.unwrap().split("/");
                    let v: usize = indexs.next().unwrap().parse().unwrap();
                    debug_println!("Vertex: {}", v);
                    let mut value = indexs.next().unwrap();
                    if value != "" {
                        let vt: usize = value.parse().unwrap();
                        debug_println!("Texture Coordinate: {}", vt);
                    }
                    value = indexs.next().unwrap();
                    if value != "" {
                        let vn: usize = value.parse().unwrap();
                        debug_println!("Vertex Normal: {}", vn);
                    }
                }
            },
            Some("l") => {
                debug_println!("Line: {}", line);
            },
            None => {},
            _ => unreachable!("Invalid token: {}", line),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_parse() {
        let mut parser = Parser::new("resources/obj/cube.obj".to_string());
        parser.parse();
    }
}