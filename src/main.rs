mod vectors;
use terminal_size::terminal_size;
use vectors::Vector3;

mod screen;
use screen::{Screen, ScreenPosition, BLACK, SCREEN_HEIGHT, SCREEN_WIDTH};


mod mesh;
use mesh::Mesh;

mod config;
use config::Config;

use std::char;

use clearscreen;

use std::env;

pub const HELP_MESSAGE : &str =
"
Usage: cli3dviewer [FILE] [OPTIONS]

    The file must be in obj format

    -s [SCALE(integer)]  Image scale
    -o [X(integer)] [Y(integer)] Sceen offset
    -p [X(float)] [Y(float)] [Z(float)] Object position
    -d [MILLISECONDS(int)] Delay between frames
    -r [X(float)] [Y(float)] [Z(float)] Rotation per frame
    -c [CHARACTER] Output characters

    Example with default values:
    cli3dviewer [FILE] -s 2 -o 0 0 -p 0 0 3 -d 30 -r 0.02 0.03 0.01 -c \\#
";

fn print_help() {
    println!("{}", HELP_MESSAGE);
}
fn main() {
    let config_res  = Config::from_args(env::args().collect());
    match config_res {
        Ok(config) => {
            let mut screen= Screen::new();
            let mut mesh = Mesh::from_obj_file(&config.model_path).expect("Failed to load model");
            let scale = config.scale/mesh.get_size();
            let position = config.position;
            let mut final_screen_offset : ScreenPosition;
            loop {
                let term_size = terminal_size().expect("Failed to read terminal size");
                let screen_size: f64 = ((term_size.0.0/2 + term_size.1.0)/2) as f64;
                final_screen_offset = ScreenPosition { x: (term_size.0.0/4) as usize + config.screen_offset.x, y: (term_size.1.0/2) as usize + config.screen_offset.y};
                screen.canvas = [[BLACK; SCREEN_HEIGHT]; SCREEN_WIDTH];
                for i in 0..mesh.vertices.len() {
                    mesh.vertices[i] = mesh.vertices[i].rotate(&config.rotation)
                }
                screen.draw_mesh(&mesh, &position, scale * screen_size, &final_screen_offset, config.white_char);
                
                screen.print_screen((term_size.0.0/2) as usize, term_size.1.0 as usize);
                std::thread::sleep(std::time::Duration::from_millis(config.delay_millis));
            }
        }
        Err(_) => {
            print_help();
        }
    }
    
}
