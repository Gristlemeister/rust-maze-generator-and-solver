use macroquad::prelude::*;
use image::{Rgba, RgbaImage};

use std::io;

mod macroquad_funcs;
use macroquad_funcs::*;

const WINDOW_WIDTH: i32  = 1280;
const WINDOW_HEIGHT: i32 = 720;

#[derive(Clone, Copy, PartialEq)]
struct Coord{
    x: i32,
    y: i32
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Maze Solver".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

fn valid_coord(coord: &Coord, maze: &RgbaImage, dead_ends: &Vec<Coord>) -> bool{
    let within_borders = coord.x >= 0 &&
    coord.x < maze.width() as i32 &&
    coord.y >=0 &&
    coord.y < maze.height() as i32;
    let dead_end = dead_ends.contains(coord);
    return within_borders && !dead_end;
}

fn check_neighbours(coord: &Coord, maze: &RgbaImage, dead_ends: &Vec<Coord>) -> Vec<[Coord; 2]>{
    let mut neighbours: Vec<[Coord; 2]> = Vec::new();


    //Calculate coords of cell and path to cell
    let up1    = Coord{x: coord.x, y: coord.y - 1};
    let right1 = Coord{x: coord.x + 1, y: coord.y};
    let down1  = Coord{x: coord.x, y: coord.y + 1};
    let left1  = Coord{x: coord.x - 1, y: coord.y};

    let up2    = Coord{x: coord.x, y: coord.y - 2};
    let right2 = Coord{x: coord.x + 2, y: coord.y};
    let down2  = Coord{x: coord.x, y: coord.y + 2};
    let left2  = Coord{x: coord.x - 2, y: coord.y};

    //Check if cell is valid and there is a path from the current cell to that cell
    if valid_coord(&up2, maze, dead_ends){
        if *maze.get_pixel(up1.x as u32, up1.y as u32) == Rgba([255, 255, 255, 255]){
            neighbours.push([up1, up2]);//Cell and path are pushed
        }
    }
    if valid_coord(&right2, maze, dead_ends){
        if *maze.get_pixel(right1.x as u32, right1.y as u32) == Rgba([255, 255, 255, 255]){
            neighbours.push([right1, right2]);
        }
    }
    if valid_coord(&down2, maze, dead_ends){
        if *maze.get_pixel(down1.x as u32, down1.y as u32) == Rgba([255, 255, 255, 255]){
            neighbours.push([down1, down2]);
        }
    }
    if valid_coord(&left2, maze, dead_ends){
        if *maze.get_pixel(left1.x as u32, left1.y as u32) == Rgba([255, 255, 255, 255]){
            neighbours.push([left1, left2]);
        }
    }

    return neighbours;
}


#[macroquad::main(window_conf)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maze = image::open("maze.png")?.into_rgba8();
    let mut seeker_maze = RgbaImage::new(maze.width(), maze.height());
    let mut is_valid_pixel = false;
    let mut start_coord: Coord = Coord{x: 0, y: 0};
    let mut end_coord: Coord = Coord{x: 0, y: 0};

    //Counts steps to track when to draw next frame. Steps taken before drawing next frame is speed
    let mut speed: u8;
    let mut step_count: u8 = 0;

    
    draw_mazes([&maze, &seeker_maze]).await;

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

    //Get start position
    while !is_valid_pixel{
        if is_mouse_button_pressed(MouseButton::Left){
            start_coord = get_clicked_pixel(&maze);
            if *maze.get_pixel(start_coord.x as u32, start_coord.y as u32) == Rgba([255u8, 255, 255, 255]) &&
            start_coord.x % 2 == 1 && start_coord.y % 2 == 1{
                is_valid_pixel = true;
                seeker_maze.put_pixel(start_coord.x as u32, start_coord.y as u32, Rgba([255, 0, 0, 255]));
            }
        }
        draw_mazes([&maze, &seeker_maze]).await;
    }

    is_valid_pixel = false;
    //Get end position
    while !is_valid_pixel{
        if is_mouse_button_pressed(MouseButton::Left){
            end_coord = get_clicked_pixel(&maze);
            if *maze.get_pixel(end_coord.x as u32, end_coord.y as u32) == Rgba([255u8, 255, 255, 255]) &&
            end_coord.x % 2 == 1 && end_coord.y % 2 == 1{
                is_valid_pixel = true;
                seeker_maze.put_pixel(end_coord.x as u32, end_coord.y as u32, Rgba([255, 0, 0, 255]));
            }
        }
        draw_mazes([&maze, &seeker_maze]).await;
    }

    let mut junction_stack: Vec<Coord> = Vec::new();
    let mut dead_ends: Vec<Coord> = Vec::new();
    junction_stack.push(start_coord);

    while let Some(&current_coord) = junction_stack.last(){
        let neighbours = check_neighbours(&current_coord, &maze, &dead_ends);

        seeker_maze.put_pixel(current_coord.x as u32, current_coord.y as u32, Rgba([200, 30, 200, 255]));

        if current_coord != end_coord{
            if current_coord == start_coord{
                junction_stack.push(neighbours[0][1]);
            }
            else if neighbours.len() == 1{
                seeker_maze.put_pixel(current_coord.x as u32, current_coord.y as u32, Rgba([200, 200, 30, 255]));
                dead_ends.push(current_coord.clone());
                junction_stack.pop();
            }
            else{
                let chosen_neighbour: Coord;
                if neighbours[0][1] == junction_stack[junction_stack.len()-2]{
                    chosen_neighbour = neighbours[1][1];
                }
                else{
                    chosen_neighbour = neighbours[0][1];
                }
                junction_stack.push(chosen_neighbour);
                //prev_coord = current_coord;
            }
        }
        else{
            break;
        }

        step_count += 1;
        if step_count >= speed{
            step_count = 0;
            draw_mazes([&maze, &seeker_maze]).await;
        }
    }

    junction_stack.reverse(); // Orders stack as start to finish when popping for animation
    while let Some(current_coord) = junction_stack.pop(){
        if junction_stack.len() > 0{
            let next = junction_stack[junction_stack.len()-1];
            let path_x = (current_coord.x + next.x) as u32 >> 1;
            let path_y = (current_coord.y + next.y) as u32 >> 1;
            seeker_maze.put_pixel(current_coord.x as u32, current_coord.y as u32, Rgba([255, 0, 0, 255]));
            seeker_maze.put_pixel(path_x, path_y, Rgba([255, 0, 0, 255]));
            step_count += 1;
            if step_count >= speed{
                step_count = 0;
                draw_mazes([&maze, &seeker_maze]).await;
            }
        }
        else{
            seeker_maze.put_pixel(current_coord.x as u32, current_coord.y as u32, Rgba([255, 0, 0, 255]));
        }
    }
    

    //When program ends just draw result
    println!("Program finished");
    loop{
        draw_mazes([&maze, &seeker_maze]).await;
    }
    //Ok(())
}
