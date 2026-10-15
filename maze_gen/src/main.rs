use image::{Rgb, RgbImage};

use macroquad::{prelude::*, telemetry::frame};
use ::rand::random_range;
use std::io;

mod macroquad_funcs;
use macroquad_funcs::*;

const WINDOW_WIDTH: i32  = 1280;
const WINDOW_HEIGHT: i32 = 720;

#[derive(Clone, Copy)]
struct Coord{
    x: i32,
    y: i32
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Generator".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

fn valid_coord(coord: &Coord, maze: &RgbImage) -> bool{
    return coord.x >= 0 && coord.x < maze.width() as i32 && coord.y >=0 && coord.y < maze.height() as i32;
}

fn check_neighbours(coord: &Coord, maze: &RgbImage) -> Vec<[Coord; 2]>{
    let mut neighbours: Vec<[Coord; 2]> = Vec::new();

    let up    = Coord{x: coord.x, y: coord.y - 2};
    let right = Coord{x: coord.x + 2, y: coord.y};
    let down  = Coord{x: coord.x, y: coord.y + 2};
    let left  = Coord{x: coord.x - 2, y: coord.y};

    if valid_coord(&up, maze){
        if *maze.get_pixel(up.x as u32, up.y as u32) == Rgb([0, 0, 0]){
            let up2 = Coord{x: coord.x, y: coord.y - 1};
            neighbours.push([up, up2]);
        }
    }
    if valid_coord(&right, maze){
        if *maze.get_pixel(right.x as u32, right.y as u32) == Rgb([0, 0, 0]){
            let right2 = Coord{x: coord.x + 1, y: coord.y};
            neighbours.push([right, right2]);
        }
    }
    if valid_coord(&down, maze){
        if *maze.get_pixel(down.x as u32, down.y as u32) == Rgb([0, 0, 0]){
            let down2 = Coord{x: coord.x, y: coord.y + 1};
            neighbours.push([down, down2]);
        }
    }
    if valid_coord(&left, maze){
        if *maze.get_pixel(left.x as u32, left.y as u32) == Rgb([0, 0, 0]){
            let left2 = Coord{x: coord.x - 1, y: coord.y};
            neighbours.push([left, left2]);
        }
    }

    return neighbours;
}

#[macroquad::main(window_conf)]
async fn main(){
    let mut maze_width: u32;
    let mut maze_height: u32;

    let mut speed: u8;
    let mut step_count: u8 = 0;

    request_new_screen_size(TEX_WIDTH as f32, TEX_WIDTH as f32);
    next_frame().await;

    //Get width input
    loop{
        let mut maze_width_s = String::new();
        println!("Enter maze width:");
        io::stdin()
        .read_line(&mut maze_width_s)
        .expect("Failed to read line");

        maze_width = match maze_width_s.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again");
                continue;
            }
        };
        if maze_width % 2 == 0{
            println!("Input must be an odd number, try again");
            continue;
        }
        break;
    }

    //Get height input
    loop{
        let mut maze_height_s = String::new();
        println!("Enter maze height:");
        io::stdin()
        .read_line(&mut maze_height_s)
        .expect("Failed to read line");

        maze_height = match maze_height_s.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again");
                continue;
            }
        };
        if maze_height % 2 == 0{
            println!("Input must be an odd number, try again");
            continue;
        }
        break;
    }

    //Get animation speed input
    loop{
        let mut speed_s = String::new();
        println!("Enter search animation speed (1 slowest, 20 fastest):");
        io::stdin()
        .read_line(&mut speed_s)
        .expect("Failed to read line");

        speed = match speed_s.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again");
                continue;
            }
        };
        if speed == 0 || speed > 20{
            println!("Value must be between 1 and 20, try again");
            continue;
        }
        break;
    }

    let start_coord = Coord{x: 1, y: 1};
    let mut junction_stack: Vec<Coord> = Vec::new();

    let mut maze = RgbImage::new(maze_width, maze_height);

    draw_maze(&maze).await;

    maze.put_pixel(start_coord.x as u32, start_coord.y as u32, Rgb([255, 255, 255]));
    junction_stack.push(start_coord);

    while let Some(current_coord) = junction_stack.pop(){
        let neighbours = check_neighbours(&current_coord, &maze);
        let mut neighbour_num:usize = 0;
        if neighbours.len() > 1{
            junction_stack.push(current_coord);
            neighbour_num = random_range(0..neighbours.len());
        }
        
        if neighbours.len() > 0{
            let chosen_neighbour = &neighbours[neighbour_num];
            junction_stack.push(chosen_neighbour[0]);

            maze.put_pixel(chosen_neighbour[1].x as u32, chosen_neighbour[1].y as u32, Rgb([255, 255, 255]));
            maze.put_pixel(chosen_neighbour[0].x as u32, chosen_neighbour[0].y as u32, Rgb([255, 255, 255]));
            
            step_count += 1;
            if step_count >= speed{
                step_count = 0;
                draw_maze(&maze).await;
            }
        }
    }


    println!("Program end");
    let _result = maze.save("maze.png");
    //Just draw at end of program
    loop{
        draw_maze(&maze).await;
    }
    
}
