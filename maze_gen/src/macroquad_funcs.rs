use macroquad::{prelude::*, texture::FilterMode::Nearest};
//use macroquad::color;
use image::{ImageBuffer, Rgb, RgbImage};

pub const TEX_WIDTH:f32 = 750.0;

fn image_to_texture(img: &RgbImage) -> Texture2D {
    // Convert RgbImage to raw RGBA bytes expected by Macroquad
    let rgba_img = image::DynamicImage::ImageRgb8(img.clone()).into_rgba8();
    
    let mq_image = macroquad::texture::Image {
        width: rgba_img.width() as u16,
        height: rgba_img.height() as u16,
        bytes: rgba_img.into_raw(),
    };

    Texture2D::from_image(&mq_image)
}

pub async fn draw_maze(maze: &ImageBuffer<Rgb<u8>, Vec<u8>>){
    let texture = image_to_texture(maze);
    texture.set_filter(Nearest);
    let aspect = maze.height() as f32/maze.width() as f32;

    let tex_height = TEX_WIDTH * aspect;

    clear_background(DARKGRAY);
    draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
        dest_size: Some(vec2(TEX_WIDTH, tex_height)), // Forced width & height
        ..Default::default()
    },);

    let aspect: f32 = maze.height() as f32/maze.width() as f32;
    request_new_screen_size(TEX_WIDTH as f32, aspect * TEX_WIDTH);
    next_frame().await;
}