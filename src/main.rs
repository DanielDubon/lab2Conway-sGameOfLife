mod color;
mod framebuffer;
mod bmp;
mod line_impl;
mod vertex;

use framebuffer::Framebuffer;
use line_impl::Line;
use crate::vertex::Vertex;
use nalgebra_glm::{Vec3};
use crate::color::Color;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use rand::Rng;


fn initialize_pattern(framebuffer: &mut Framebuffer, count: usize) {
    let mut rng = rand::thread_rng();
    let max_x = framebuffer.width - 3;
    let max_y = framebuffer.height - 3;

    for _ in 0..count {
        let x = rng.gen_range(0..max_x);
        let y = rng.gen_range(0..max_y);
        match rng.gen_range(0..14) {
            0 => add_block(framebuffer, x, y),
            1 => add_beehive(framebuffer, x, y),
            2 => add_loaf(framebuffer, x, y),
            3 => add_boat(framebuffer, x, y),
            4 => add_tub(framebuffer, x, y),
            5 => add_blinker(framebuffer, x, y),
            6 => add_toad(framebuffer, x, y),
            7 => add_beacon(framebuffer, x, y),
            8 => add_pulsar(framebuffer, x, y),
            9 => add_pentadecathlon(framebuffer, x, y),
            10 => add_glider(framebuffer, x, y),
            11 => add_lwss(framebuffer, x, y),
            12 => add_mwss(framebuffer, x, y),
            13 => add_hwss(framebuffer, x, y),
            _ => {}, 
        }
    }
}


fn add_block(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (0, 0), (1, 0),
        (0, 1), (1, 1),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_beehive(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (1, 0), (2, 0),
        (0, 1), (3, 1),
        (1, 2), (2, 2),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_loaf(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (1, 0), (2, 0),
        (0, 1), (3, 1),
        (1, 2), (3, 2),
        (2, 3),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_boat(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (0, 0), (1, 0),
        (0, 1), (2, 1),
        (1, 2),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_tub(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (1, 0),
        (0, 1), (2, 1),
        (1, 2),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_blinker(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (0, 0), (1, 0), (2, 0),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_toad(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (1, 0), (2, 0), (3, 0),
        (0, 1), (1, 1), (2, 1),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_beacon(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (0, 0), (1, 0),
        (0, 1),
        (3, 2),
        (2, 3), (3, 3),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_pulsar(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_pentadecathlon(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (0, 0), (1, 0), (3, 0), (4, 0),
        (0, 2), (1, 2), (3, 2), (4, 2),
        (1, 3), (3, 3),
        (1, 4), (3, 4),
        (0, 5), (1, 5), (3, 5), (4, 5),
        (0, 7), (1, 7), (3, 7), (4, 7),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_glider(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (1, 0),
        (2, 1),
        (0, 2), (1, 2), (2, 2),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_lwss(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (1, 0), (4, 0),
        (0, 1),
        (0, 2), (4, 2),
        (0, 3), (1, 3), (2, 3), (3, 3),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_mwss(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
        (0, 1), (5, 1),
        (0, 2), (5, 2),
        (0, 3), (1, 3), (2, 3), (3, 3), (4, 3),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}

fn add_hwss(framebuffer: &mut Framebuffer, x: usize, y: usize) {
    framebuffer.set_current_color(0xFFFFFF);
    let pattern = [
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0),
        (0, 1), (6, 1),
        (0, 2), (6, 2),
        (0, 3), (1, 3), (2, 3), (3, 3), (4, 3), (5, 3),
    ];
    for &(dx, dy) in pattern.iter() {
        framebuffer.point((x + dx) as isize, (y + dy) as isize);
    }
}


fn render(framebuffer: &mut Framebuffer) {
    // Clear the framebuffer
    framebuffer.set_background_color(0x333355);
    framebuffer.clear();

    // Draw some points
    framebuffer.set_current_color(0xFFDDDD);
    framebuffer.point(20, 40);
}

fn count_live_neighbors(framebuffer: &Framebuffer, x: isize, y: isize) -> usize {
    let mut count = 0;
    let deltas = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    for (dx, dy) in deltas.iter() {
        if let Some(color) = framebuffer.get_point(x + dx, y + dy) {
            if color == Color::new(255, 255, 255) {
                count += 1;
            }
        }
    }

    count
}

fn update_framebuffer(framebuffer: &mut Framebuffer) {
    let mut new_buffer = framebuffer.buffer.clone();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let live_neighbors = count_live_neighbors(&framebuffer, x as isize, y as isize);
            let index = y * framebuffer.width + x;
            let cell = framebuffer.buffer[index];

            if cell == Color::new(255, 255, 255) {
                if live_neighbors < 2 || live_neighbors > 3 {
                    new_buffer[index] = Color::new(0, 0, 0); // Muere
                }
            } else {
                if live_neighbors == 3 {
                    new_buffer[index] = Color::new(255, 255, 255); // Nace
                }
            }
        }
    }

    framebuffer.buffer = new_buffer;
}


fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    initialize_pattern(&mut framebuffer,10);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Actualizar framebuffer
        update_framebuffer(&mut framebuffer);

        // Actualizar la ventana con el contenido del framebuffer
        window
            .update_with_buffer(&framebuffer.as_u32_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
