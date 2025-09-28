#![allow(dead_code)]
#![allow(unused_variables)]
use std::panic::PanicHookInfo;

use image::{ImageBuffer};
use rand::{self, random, Rng, SeedableRng};
use rand::rngs::StdRng;

type Buf = ImageBuffer<image::Rgb<u8>, Vec<u8>>;

const BLACK: [u8; 3] = [0, 0, 0];
const WHITE: [u8; 3] = [255, 255, 255];
const BLUE: [u8; 3] = [0, 0, 255];
const GREEN: [u8; 3] = [0, 255, 0];
const RED: [u8; 3] = [255, 0, 0];
const SEA_BLUE1: [u8; 3] = [0, 131, 255];
const SEA_BLUE2: [u8; 3] = [0, 107, 209];
const SEA_BLUE3: [u8; 3] = [0, 81, 158];
const SEA_BLUE4: [u8; 3] = [0, 61, 120];
const SEA_BLUE5: [u8; 3] = [0, 47, 92];

/* 
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
*/

fn create_solid_image(width: u32, height: u32, color: [u8; 3]) -> Buf{
    let mut buffer: Buf = image::ImageBuffer::new(width, height);
    for (_x, _y, pixel) in buffer.enumerate_pixels_mut() {
        *pixel = image::Rgb(color);
    }
    buffer
}

// probability dicates the chance that a pixel will be white
// the lower the number, the lower the chance
fn generate_noisemap_binary(width: u32, height: u32, probability: u8) -> Buf{
    let mut buffer: Buf = image::ImageBuffer::new(width, height);
    for (_x, _y, pixel) in buffer.enumerate_pixels_mut() {
        let number: u8 = rand::random();
        if number < probability {
            *pixel = image::Rgb([255, 255, 255]);
        } else {
            *pixel = image::Rgb([0, 0, 0]);
        }
    }
    buffer
}

fn generate_noisemap_bw(width: u32, height: u32) -> Buf{
    let mut buffer: Buf = image::ImageBuffer::new(width, height);
    for (_x, _y, pixel) in buffer.enumerate_pixels_mut() {
        let number: u8 = rand::random();
        *pixel = image::Rgb([number, number, number]);
    }
    buffer
}

// upscales an input buffer of size width, height by a scale of factor
fn upscale_image_square(factor: u32, input: Buf) -> Buf {
    let output_width: u32 = input.width() * factor;
    let output_height: u32 = input.height() * factor;
    let mut buffer: Buf = image::ImageBuffer::new(output_width, output_height);
    for(x, y, pixel) in input.enumerate_pixels() {
        for sub_y in 0..factor{
            for sub_x in 0..factor{
                let red = pixel[0];
                let green = pixel[1];
                let blue = pixel[2];
                if (red != 255 && red != 0) && (green != 255 && green != 0) && (blue != 255 && blue != 0){
                    println!("Red: {}, Green: {}, Blue: {}", red, green, blue);
                }
                let buffer_pixel = buffer.get_pixel_mut(sub_x + (x * factor), sub_y + (y * factor));
                *buffer_pixel = *pixel;
            }
        }
    }
    buffer
}

fn upscale_image_lines(factor: u32, input: Buf) -> Buf {
    let mut buffer: Buf = image::ImageBuffer::new(input.width() * (factor * 2 + 1), input.height() * (factor * 2 + 1));
    for (x, y, pixel) in input.enumerate_pixels() {
        if(pixel[0] == 255){
            for i in 0..=factor * 2 {
                let sub_pixel = buffer.get_pixel_mut((x  * (factor * 2 + 1)) + i - factor, y  * (factor * 2 + 1));
                *sub_pixel = image::Rgb(WHITE);
                let sub_pixel2 = buffer.get_pixel_mut((x  * (factor * 2 + 1)), y  * (factor * 2 + 1) + i - factor);
                *sub_pixel2 = image::Rgb(WHITE);
            }
        }
    }

    buffer
}

fn invert(input: Buf) -> Buf {
    let mut buffer: Buf = image::ImageBuffer::new(input.width(), input.height());
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let input_pixel = input.get_pixel(x, y);
        *pixel = image::Rgb([255 - input_pixel[0], 255 - input_pixel[1], 255 - input_pixel[2]]);
    }

    buffer
}

fn dla(output_width: u32, output_height: u32, ratio: u32) -> Buf {
    let mut width: u32 = output_width;
    let mut height: u32 = output_height;
    // shrink to correct size
    let mut buffer: Buf = create_solid_image(width, height, [0, 0, 0]);
    let mut count: u32 = 4;
    let mut x: u32;
    let mut y: u32;
    // starting coordinates are right around the center
    let buffer_pixel= buffer.get_pixel_mut(width as u32 / 2, height as u32 / 2);
    *buffer_pixel = image::Rgb([255, 255, 255]);    
    loop {
        loop {
            x = rand::random();
            y = rand::random();
            x = x % width;
            y = y % height;
            let test_pixel = buffer.get_pixel(x, y);
            if test_pixel[0] == 0 {
                break;
            }
        }
        loop {
            // check the pixel for collision
            if y != 0 {
                let test_pixel = buffer.get_pixel(x, y - 1);
                // if the pixel is white, place it
                if test_pixel[0] == 255 {
                    let pixel = buffer.get_pixel_mut(x, y);
                    *pixel = image::Rgb([255, 255, 255]);
                    break;
                }
            }

            if y != height - 1 {
                let test_pixel = buffer.get_pixel(x, y + 1);
                // if the pixel is white, place it
                if test_pixel[0] == 255 {
                    let pixel = buffer.get_pixel_mut(x, y);
                    *pixel = image::Rgb([255, 255, 255]);
                    break;
                }
            }

            if x != width - 1 {
                let test_pixel = buffer.get_pixel(x + 1, y);
                // if the pixel is white, place it
                if test_pixel[0] == 255 {
                    let pixel = buffer.get_pixel_mut(x, y);
                    *pixel = image::Rgb([255, 255, 255]);
                    break;
                }
            }
            if x != 0 {
                let test_pixel = buffer.get_pixel(x - 1, y);
                // if the pixel is white, place it
                if test_pixel[0] == 255 {
                    let pixel = buffer.get_pixel_mut(x, y);
                    *pixel = image::Rgb([255, 255, 255]);
                    break;
                }
            }

            // move the pixel
            let direction: u8 = rand::random();
            // move up
            if direction % 4 == 0 && y != 0 {
                y -= 1;
            // move right
            } else if  direction % 4 == 1 && x != width - 1 {
                x += 1;
            }
            // move down
            else if direction % 4 == 2 && y != height - 1 {
                y += 1;
            }
            // move left
            else if direction % 4 == 3 && x != 0 {
                x -= 1;
            }
        }
        //println!("{} / {}", count, (ratio * (width * height)) / 100);
        count += 1;
        // desired number of pixels is reached
        if ((count as f32 / (width as f32 * height as f32)) * 100.0) as u32 >= ratio || count >= width * height {
            break;
        }
    }
    buffer
}

fn and(input1: Buf, input2: Buf) -> Buf{
    let mut buffer: Buf = image::ImageBuffer::new(input1.width(), input2.height());
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        if input1.get_pixel(x, y)[0] == 255 && input2.get_pixel(x, y)[0] == 255 {
            *pixel = image::Rgb(WHITE);
        } else {
            *pixel = image::Rgb(BLACK);
        }
    }

    buffer
}

fn or(input1: Buf, input2: Buf) -> Buf{
    let mut buffer: Buf = image::ImageBuffer::new(input1.width(), input2.height());
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        if input1.get_pixel(x, y)[0] == 255 || input2.get_pixel(x, y)[0] == 255 {
            *pixel = image::Rgb(WHITE);
        } else {
            *pixel = image::Rgb(BLACK);
        }
    }

    buffer
}

fn not(target: [u8; 3], input: Buf) -> Buf{
    let mut buffer: Buf = image::ImageBuffer::new(input.width(), input.height());
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let sub_pixel = input.get_pixel(x, y);
        if target[0] == sub_pixel[0] && target[1] == sub_pixel[1] && target[2] == sub_pixel[2] {
            *pixel = image::Rgb(BLACK);
        } else {
            *pixel = image::Rgb(WHITE);
        }
    }

    buffer
}

fn subtract(input1: Buf, input2: Buf) -> Buf{
    let mut buffer: Buf = image::ImageBuffer::new(input1.width(), input2.height());
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        if input1.get_pixel(x, y)[0] == 255 && input2.get_pixel(x, y)[0] == 255 {
            *pixel = image::Rgb(BLACK);
        } else if input1.get_pixel(x, y)[0] == 255{
            *pixel = image::Rgb(WHITE);
        } else {
            *pixel = image::Rgb(BLACK);
        }
    }

    buffer
}

fn expand(radius: u32, color: [u8; 3], input: Buf) -> Buf {
    let mut buffer: Buf = image::ImageBuffer::new(input.width(), input.height());
    for (x, y, pixel) in input.enumerate_pixels() {
        if pixel[0] == color[0] && pixel[1] == color[1] && pixel[2] == color[2] {
            for sub_y in (y as i32 - radius as i32)..(y as i32 + radius as i32 * 2){
                for sub_x in (x as i32 - radius as i32)..(x as i32 + radius as i32 * 2){
                    if sub_y < input.height() as i32 && sub_x < input.width() as i32 && sub_y > 0 && sub_x > 0{
                        // if cell is valid, change it's color
                        let sub_pixel = buffer.get_pixel_mut(sub_x as u32, sub_y as u32);
                        *sub_pixel = image::Rgb(color);
                    }
                }
            }
        }
    }

    buffer
}

// ! basic implementation of voronoi noise
// TODO: add normalization so it looks better
// divides a board into "cells"
// each cell is assigned a single white point at a random location relative to that cell
// then, each non-white pixel is assigned a brightness depeding on how far it is from that point
fn voronoi(width: u32, height: u32, points: u32) -> Buf {
    let mut buffer: Buf = image::ImageBuffer::new(width, height);
    let cell_width: u32 = width / points;
    let cell_height: u32 = height / points;
    let mut coords: Vec<(u32, u32)> = Vec::new();
    // create random points (this squares the number of points from input unfortunately)
    // redo this later by using some sort of wrapping-esq approach for an exact 
    for iter_y in 0..points {
        for iter_x in 0..points {
            let mut x: u32 = rand::random();
            let mut y: u32 = rand::random();
            x = (x % cell_width) + (iter_x * cell_width);
            y = (y % cell_height) + (iter_y * cell_height);
            coords.push((x, y));
            let pixel = buffer.get_pixel_mut(x, y);
            *pixel = image::Rgb([255, 255, 255]);
        }
    }
    // iterating through each pixel to find minimum distance
    // this is really slow but it's fine
    // for every pixel
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        // loop through untilyou find a white pixel
        let mut min_dist = 1000000000;
        for (x2, y2) in coords.clone() {
            let dist = ((((x as i32 - x2 as i32) * (x as i32 - x2 as i32)) + ((y as i32 - y2 as i32) * (y as i32 - y2 as i32))) as f64).sqrt() as i32;
            if dist < min_dist as i32 {
                min_dist = dist as u32;
            }
        }

        if min_dist >= 255 {
            *pixel = image::Rgb([255, 255, 255]);
        } else {
            *pixel = image::Rgb([min_dist as u8, min_dist as u8, min_dist as u8]);
        }
    }
    buffer
}

fn linear_interpolate(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
    // prevent divide by zero errors
    if x1 == x2 {
        return y1 as f32;
    }
    y1 as f32 + (x as f32 - x1 as f32) * ((y1 as f32 - y2 as f32) / (x1 as f32 - x2 as f32))
}

fn overlay(input: &mut Buf, input2: Buf){
    for (x, y, pixel) in input2.enumerate_pixels() {
        if pixel[0] != 0 || pixel[1] != 0 || pixel[2] != 0 {
            let sub_pixel = input.get_pixel_mut(x, y);
            *sub_pixel = image::Rgb([pixel[0], pixel[1], pixel[2]]);
        }
        
    }
}

fn threshhold(input: &Buf, lower: u8, higher: u8, preserve: bool) -> Buf {
    let mut buffer: Buf = image::ImageBuffer::new(input.width(), input.height());
    for (x, y, pixel) in input.enumerate_pixels() {
        if pixel[0] >= lower && pixel[0] <= higher {
            if(preserve){
                let sub_pixel = buffer.get_pixel_mut(x, y);
                *sub_pixel = image::Rgb([pixel[0], pixel[1], pixel[2]]);
            } else {
                let sub_pixel = buffer.get_pixel_mut(x, y);
                *sub_pixel = image::Rgb([255, 255, 255]);
            }
        }
    }
    buffer
}

fn recolor_proportion(input: Buf, mut red: f32, mut green: f32, mut blue: f32) -> Buf {
    red = 255.0 / red;
    green = 255.0 / green;
    blue = 255.0 / blue;
    let mut buffer: Buf = image::ImageBuffer::new(input.width(), input.height());
    for (x, y, pixel) in input.enumerate_pixels() {
        let sub_pixel = buffer.get_pixel_mut(x, y);
        *sub_pixel = image::Rgb([(pixel[0] as f32 / red) as u8, (pixel[0] as f32 / green) as u8, (pixel[0] as f32 / blue) as u8]);
    }
    buffer
}

fn recolor_solid(input: Buf, red: u8, green: u8, blue: u8) -> Buf {
    let mut buffer: Buf = image::ImageBuffer::new(input.width(), input.height());
    for (x, y, pixel) in input.enumerate_pixels() {
        let sub_pixel = buffer.get_pixel_mut(x, y);
        if sub_pixel[0] == 255 && sub_pixel[1] == 255 && sub_pixel[2] == 255 {
            *sub_pixel = image::Rgb([red, green, blue]);
        }
    }
    buffer
}

fn normalize(input: &mut Buf) {
    let mut lightest: u8 = 0;
    for (_x, _y, pixel) in input.enumerate_pixels() {
        if pixel[0] > lightest {
            lightest = pixel[0];
        }
    }

    let ratio: f32 = 255.0 / lightest as f32;

    for (_x, _y, pixel) in input.enumerate_pixels_mut() {
        let color: u16 = (pixel[0] as f32 * ratio) as u16;
        *pixel = image::Rgb([color as u8, color as u8, color as u8]);
    }
}

fn save(name: &str, data: &Buf){
    image::save_buffer(name, &data, data.width(), data.height(), image::ExtendedColorType::Rgb8).unwrap();
}

// ! implementation of value noise
fn value(width: u32, height: u32, points_wide: u32, points_tall: u32) -> Buf {
    let mut buffer: Buf = image::ImageBuffer::new(width, height);
    let cell_width: f32 = (width as f32 - 1.0) / (points_wide as f32 - 1.0);
    let cell_height: f32 = (height as f32 - 1.0) / (points_tall as f32 - 1.0);
    let mut coords: Vec<Vec<(u32, u32, u8)>> = Vec::new();
    let mut temp: Vec<(u32, u32, u8)> = Vec::new();
    // placing original points
    for iter_y in 0..points_tall {
        for iter_x in 0..points_wide {
            let x: u32 = (iter_x as f32 * cell_width) as u32;
            let y: u32 = (iter_y as f32 * cell_height) as u32;
            let value: u8 = random();
            temp.push((x, y, value));
            let pixel = buffer.get_pixel_mut(x, y);
            *pixel = image::Rgb([value, value, value]);
        }
        // create a 2d list of each coordinate pair
        coords.push(temp.clone());
        temp = [].to_vec();
    }
    // interpolate horizontally
    
    for sub_list in coords {
        let mut x: u32 = 0;
        for i in 0..points_wide - 1 {
            for k in sub_list[i as usize].0..sub_list[i as usize + 1].0 {
                let value: u8 = linear_interpolate(sub_list[i as usize].0 as f32, sub_list[i as usize].2 as f32, sub_list[i as usize + 1].0 as f32, sub_list[i as usize + 1].2 as f32, x as f32) as u8;
                let pixel = buffer.get_pixel_mut(x, sub_list[i as usize].1);
                *pixel = image::Rgb([value as u8, value as u8, value as u8]);
                x += 1;
            }
        }
    }
    // interpolate vertically
    let mut output: Buf = buffer.clone();
    for i in 0..points_tall - 1 {
        for x in 0..width {
            
            // get the chunk
            let lower: u32 = (i as f32 * cell_height) as u32;
            let higher: u32 = ((i + 1) as f32 * cell_height) as u32;
            for y in lower..higher{
                let pixel_1 = buffer.get_pixel(x, lower);
                let pixel_2 = buffer.get_pixel(x, higher);
                let target_pixel = output.get_pixel_mut(x, y);
                let value: u8 = linear_interpolate(lower as f32, pixel_1[0] as f32, higher as f32, pixel_2[0] as f32, y as f32) as u8;
                *target_pixel = image::Rgb([value, value, value])
            }
        }
    }
    output
}

fn scale_noise(input: &mut Buf, level: f32){
    for (_x, _y, pixel) in input.enumerate_pixels_mut() {
        let mut red: u32 = ((pixel[0] as f32) * level) as u32;
        let mut green: u32 = ((pixel[1] as f32) * level) as u32;
        let mut blue: u32 = ((pixel[2] as f32) * level) as u32;
        if red > 255 {
            red = 255;
        }
        if green > 255 {
            green = 255;

        }
        if blue > 255 {
            blue = 255;
        }
        *pixel = image::Rgb([red as u8, green as u8, blue as u8]);
    }
}

fn linear_scale_noise(input: &mut Buf, level: i32){
    for (_x, _y, pixel) in input.enumerate_pixels_mut() {
        let mut red: i32 = pixel[0] as i32 + level;
        let mut green: i32 = pixel[1] as i32 + level;
        let mut blue: i32 = pixel[2] as i32 + level;
        if red > 255 {
            red = 255;
        } else if red < 0 {
            red = 0;
        }
        if green > 255 {
            green = 255;
        } else if green < 0 {
            green = 0;
        }
        if blue > 255 {
            blue = 255;
        } else if blue < 0 {
            blue = 0;
        }
        *pixel = image::Rgb([red as u8, green as u8, blue as u8]);
    }
}

fn add(input: &mut Buf, input2: Buf){
    for (x, y, pixel) in input.enumerate_pixels_mut() {
        let second_pixel = input2.get_pixel(x, y);
        let mut red: u32 = pixel[0] as u32 + second_pixel[0] as u32;
        if red > 255 {
            red = 255;
        }
        let mut green: u32 = pixel[1] as u32 + second_pixel[1] as u32;
        if green > 255 {
            green = 255;
        }
        let mut blue: u32 = pixel[2] as u32 + second_pixel[2] as u32;
        if blue > 255 {
            blue = 255;
        }
        *pixel = image::Rgb([red as u8, green as u8, blue as u8]);
    }
}

fn fractal_value(width: u32, height: u32, mut points_wide: u32, mut points_tall: u32, inc: u32, octaves: u8) -> Buf{
    let mut buffer: Buf = value(width, height, points_wide, points_tall);
    let mut level: f32 = 0.25;
    for i in 1..octaves {
        points_tall *= inc;
        points_wide *= inc;
        println!("Adding another octave at {points_wide} by {points_tall}");
        let mut octave: Buf = value(width, height, points_tall, points_wide);
        scale_noise(&mut octave, level);
        level /= 2.0;
        add(&mut buffer, octave);
    }
    buffer
}

fn interpolate_smoothing(input: &mut Buf, lower: u8, higher: u8){
    let mut dim: u8 = 255;
    let mut bright: u8 = 0;
    for (_x, _y, pixel) in input.enumerate_pixels() {
        if pixel[0] < dim {
            dim = pixel[0];
        }
        if pixel[0] > bright {
            bright = pixel[0];
        }
    }
    let lower_ratio: f32;
    let higher_ratio: f32;

    if dim == 0 {
        lower_ratio = lower as f32;
    } else {
        lower_ratio = lower as f32 / dim as f32;
    }
    if bright == 0 {
        higher_ratio = higher as f32;
    } else {
        higher_ratio = higher as f32 / bright as f32;
    }


    for (_x, _y, pixel) in input.enumerate_pixels_mut() {
        let color: u8 = (pixel[0] as f32 * linear_interpolate(dim as f32, lower_ratio, bright as f32, higher_ratio, pixel[0] as f32)) as u8;
        *pixel = image::Rgb([color, color, color]);
    }
}

// TODO: Add Voronoi Noise [X]
// TODO: Fix Voronoi Noise (make it work better) [X] - added a normalization function which should handle it for the most part
// TODO: Add Value Noise [X]
// TODO: Fix Value Noise (divide width and height by (points - 1)) [X] 
// TODO: Create a linear-interpolation smoothing function or something [ ]
// TODO: Reimpliment Into Gaia Maybe? [ ]

fn main() {
    let width: u32 = 1024;
    let height: u32 = 1024;
    let name: &str = "final.png";

    let water_level: u8 = 64; 
    let mountain_level: u8 = 196;
    
    // let mut data: Buf = fractal_value(width, height, 9, 9, 3, 5);
    let mut data: Buf = fractal_value(width, height, 3, 3, 3, 6);
    // data = invert(data);
    // linear_scale_noise(&mut data, 1);
    // normalize(&mut data);
    interpolate_smoothing(&mut data, 0, 255);

    /* 
    */

    let mut snow: Buf = threshhold(&data, mountain_level, 255, true);
    snow = recolor_proportion(snow, 220.0, 220.0, 220.0);

    let mut land: Buf = threshhold(&data, water_level, mountain_level - 1, true);
    normalize(&mut land);
    //land = recolor_proportion(land, 250.0, 227.0, 180.0);
    land = recolor_proportion(land, 128.0, 128.0, 128.0);
    let mut water: Buf = threshhold(&data, 0, water_level - 1, true);
    linear_scale_noise(&mut water, 32);
    normalize(&mut water);
    water = recolor_proportion(water, 1.0, 87.0, 255.0);

    overlay(&mut water, land);
    overlay(&mut water, snow);

    save(name, &water);
    
}
