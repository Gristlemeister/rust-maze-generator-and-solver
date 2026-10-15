use macroquad::{prelude::*, texture::FilterMode::Nearest};
//use macroquad::color;
use image::RgbaImage;
use crate::Coord;

pub const TEX_WIDTH:f32 = 600.0;

fn size_window_to_maze(maze: &RgbaImage){
    let aspect: f32 = maze.height() as f32/maze.width() as f32;
    request_new_screen_size(TEX_WIDTH as f32, aspect * TEX_WIDTH);
}

fn image_to_texture(img: &RgbaImage) -> Texture2D {
    // Convert RgbImage to raw RGBA bytes expected by Macroquad
    let rgba_img = image::DynamicImage::ImageRgba8(img.clone()).into_rgba8();
    
    let mq_image = macroquad::texture::Image {
        width: rgba_img.width() as u16,
        height: rgba_img.height() as u16,
        bytes: rgba_img.into_raw(),
    };

    Texture2D::from_image(&mq_image)
}

pub async fn draw_mazes(mazes: [&RgbaImage; 2]){
    let base_texture = image_to_texture(mazes[0]);
    base_texture.set_filter(Nearest);
    let seeker_texture = image_to_texture(mazes[1]);
    seeker_texture.set_filter(Nearest);
    let aspect = mazes[0].height() as f32/mazes[0].width() as f32;

    let tex_height = TEX_WIDTH * aspect;

    clear_background(DARKGRAY);
    draw_texture_ex(&base_texture, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(TEX_WIDTH, tex_height)), // Forced width & height
        ..Default::default()
    });

    draw_texture_ex(&seeker_texture, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(TEX_WIDTH, tex_height)), // Forced width & height
        ..Default::default()
    });

    size_window_to_maze(mazes[0]);
    next_frame().await;
}

pub fn get_clicked_pixel(maze: &RgbaImage) -> Coord{
    let (mouse_x, mouse_y) = mouse_position();
    let inv_scaling =  maze.width() as f32 / TEX_WIDTH;

    let mut coord: Coord = Coord{x:0, y:0};

    coord.x = ((mouse_x) * inv_scaling) as i32;
    coord.y = ((mouse_y) * inv_scaling) as i32;

    if coord.x >= maze.width() as i32{
        coord.x = (maze.width() - 1) as i32;
    }

    if coord.y >= maze.height() as  i32{
        coord.y = (maze.height() - 1) as i32;
    }

    return coord;
}