use crate::vectors::Vector3;
use crate::mesh::Mesh;

pub const SCREEN_WIDTH : usize = 256;
pub const SCREEN_HEIGHT : usize = 256;
pub const BLACK : char = ' ';
pub struct Screen {
    pub canvas : [[char; SCREEN_HEIGHT]; SCREEN_WIDTH]
}

impl Screen {
    pub fn new() -> Self {
        Screen { canvas: [[BLACK; SCREEN_HEIGHT]; SCREEN_WIDTH] }
    }

    pub fn print_screen(&self, width : usize, height : usize) {
        clearscreen::clear().unwrap();
        //println!("{}x{}", width, height);
        for i in 0..if SCREEN_HEIGHT < height {SCREEN_HEIGHT} else {height} {
            for j in 0..if SCREEN_WIDTH < width {SCREEN_WIDTH} else {width} {
                let ch = self.canvas[j][i];
                print!("{}{}", ch, ch);
            }
            println!("");
        }
    }

    pub fn draw_point(&mut self, screen_pos : ScreenPosition, ch : char) {
        if screen_pos.x < self.canvas.len() && screen_pos.y < self.canvas[screen_pos.x].len() {
            self.canvas[screen_pos.x][screen_pos.y] = ch;
        }
    }

    pub fn draw_line(&mut self, a : ScreenPosition, b : ScreenPosition, ch : char) {
        let k = (b.y as f32 - a.y as f32) /(b.x as f32 - a.x as f32);
        if k <= 1.0 && k >= -1.0 {
            let mut y : f32 = (if a.x < b.x {a.y} else {b.y}) as f32;
            for x in std::cmp::min(a.x, b.x)..std::cmp::max(a.x, b.x) {
                y += k;
                self.draw_point(ScreenPosition { x: x, y: y as usize }, ch);
            }
        }
        else {
            let mut x : f32 = (if a.y < b.y {a.x} else {b.x}) as f32;
            for y in std::cmp::min(a.y, b.y)..std::cmp::max(a.y, b.y) {
                x += 1.0/k;
                self.draw_point(ScreenPosition { x: x as usize, y: y }, ch);
            }
        }
    }

    pub fn draw_mesh(&mut self, mesh : &Mesh, offset : &Vector3, scale : f64, screen_offset: &ScreenPosition, white_char : char) {
        let vs = &mesh.vertices;
        let eg = &mesh.edges;
        
        for e in eg {
            if (vs[e.0].z + offset.z) > 0.0 && (vs[e.1].z + offset.z) > 0.0 { // checks is vertex behind the camera
                let a = ScreenPosition::from_v3_projection(&vs[e.0], offset, scale, screen_offset);
                let b = ScreenPosition::from_v3_projection(&vs[e.1], offset, scale, screen_offset);
                self.draw_line(a, b, white_char);
            }
        }
    }
}
pub struct ScreenPosition {
    pub x : usize,
    pub y : usize
}
impl ScreenPosition {
    pub fn from_v3_projection(pos : &Vector3, offset : &Vector3, scale : f64, screen_offset : &ScreenPosition) -> Self{
        let x = (((pos.x + offset.x)/(pos.z + offset.z))*scale + screen_offset.x as f64) as usize;
        let y = (((pos.y + offset.y)/(pos.z + offset.z))*scale + screen_offset.y as f64) as usize;
        Self { x: x, y: y}
    }
}