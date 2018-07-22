use ::graphics::renderer::Renderer;
use ::graphics::colour::Colour;

use ::util::vector::Vector2D;

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::fs;

pub const MAP_WIDTH: i16 = 100;
pub const MAP_HEIGHT: i16 = 50;

pub struct Map {
    world_position: Vector2D<i16>,
    tile_data: Vec<String>,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

impl Map {
    /**
     * Loads a map from a file for coordinates (x, y)
     */
    pub fn load(x: i16, y: i16) -> Option<Map> {
        let mut map = Map {
            world_position: Vector2D::new(x, y),
            tile_data: Vec::with_capacity((MAP_HEIGHT) as usize)
        };

        let mut file_name = String::from("maps/");
        file_name.push_str(x.to_string().as_str());
        file_name.push(' ');
        file_name.push_str(y.to_string().as_str());

        if !path_exists(&file_name) {
            return None
        }
        else {
            let file = File::open(file_name)
                .expect(&format!("Unable to open file for map {} {}", x, y));

            for line in BufReader::new(file).lines() {
                
                map.tile_data.push(line.unwrap());
                if map.tile_data.len() == MAP_HEIGHT as usize {
                    break;
                }
            }

            Some(map)
        }
    }

    pub fn draw_line(&self, renderer: &Renderer, line: usize, begin: usize, end: usize, draw_point: &Vector2D<i16>) {
        let mut render_string = String::with_capacity((MAP_WIDTH * 2) as usize);
        let ref_string = &self.tile_data[line];

        let mut cur_char = ' ';
        for c in ref_string[begin..end].chars() {
            if c != cur_char {
                cur_char = c;
                match c {
                    '.' => render_string.push_str(&Colour::ansi_text_colour_string(100, 255, 25)),
                    '#' => render_string.push_str(&Colour::ansi_text_colour_string(160, 82, 45)),
                     _ => {}
                }
            }
            render_string.push(c);
        } 


        renderer.draw_string("game", 
            &render_string, 
            &draw_point);
    }

    pub fn get_tile(&self, x: i16, y: i16) -> char {
        let line = &self.tile_data[y as usize];
        
        //rust is annoying with finding the char of a string via index, so only way i can think of is to literally iterate over the string
        for (i, c) in line.chars().enumerate() {
            if i == x as usize {
                return c
            }
        } 
        //failure to find the index (which cannot happen) results in returning a . 
        '.'
    }

    pub fn world_position(&self) -> &Vector2D<i16> {
        &self.world_position
    }
}