use std::{error::Error, fs};
use crate::vectors::Vector3;
pub struct Mesh {
    pub vertices: Vec<Vector3>,
    pub edges: Vec<(usize, usize)>
}
impl Mesh {
    pub fn from_obj_file(filepath : &str) -> Result<Self, Box<dyn Error>> {
        let obj_text = fs::read_to_string(filepath)?;
        let obj_lines : Vec<&str> = obj_text.split('\n').collect();

        let mut amount_of_vertices = 0;
        let mut amount_of_edges = 0;
        for line in obj_lines.clone() {
            let line_type= line.split(' ').next();
            if line_type == Some("v") {
                amount_of_vertices += 1;
            }
            else if line_type == Some("f") {
                amount_of_edges += 1;
            }
        }

        let mut vertices : Vec<Vector3> = Vec::with_capacity(amount_of_vertices);
        let mut edges : Vec<(usize, usize)> = Vec::with_capacity(amount_of_edges);
        
        for line in obj_lines {
            let splited_line : Vec<&str> = line.split(' ').collect();
            
            if splited_line[0] == "v" {
                let x = splited_line[1].parse()?;
                let y = splited_line[2].parse()?;
                let z = splited_line[3].parse()?;
                vertices.push(Vector3 { x: x, y: y, z: z });
            }
            else if splited_line[0] == "f" {
                let mut vertices_in_face : Vec<usize> = Vec::with_capacity(4);
                for i in 1..splited_line.len() {
                    //println!("{}", splited_line[i].split('/').collect::<Vec<&str>>()[0]);
                    let b : usize = splited_line[i].split('/').collect::<Vec<&str>>()[0].parse()?;
                    vertices_in_face.push(b);
                }
                for i in 1..vertices_in_face.len() {
                    edges.push((vertices_in_face[i-1]-1, vertices_in_face[i]-1));
                }
            }
        }
        Ok(Mesh { vertices, edges })
    }
    pub fn get_size(&self) -> f64 {
        let mut min = Vector3{x : 0f64, y: 0f64, z: 0f64};
        let mut max = Vector3{x : 0f64, y: 0f64, z: 0f64};
        for v in &self.vertices {
            if v.x < min.x {
                min.x = v.x;
            }
            if v.y < min.y {
                min.y = v.y;
            }
            if v.z < min.z {
                min.z = v.z;
            }
            if v.x > max.x {
                max.x = v.x;
            }
            if v.y > max.y {
                max.y = v.y;
            }
            if v.z > max.z {
                max.z = v.z;
            }
        }
        min.distanse(&max)
    }
}