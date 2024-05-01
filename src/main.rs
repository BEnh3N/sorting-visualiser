use std::time::Instant;

use rand::prelude::*;
use raylib::prelude::*;
use sorting::algorithms::bubble::bubble_sort;
use sorting::algorithms::merge::merge_sort;

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 512;
const LINE_WIDTH: i32 = 2;

const NUM_ITEMS: i32 = WIDTH / LINE_WIDTH;
const LINE_MUL: i32 = HEIGHT / NUM_ITEMS;

fn main() {
    let (mut handle, thread) = init()
        .size(WIDTH, HEIGHT)
        .title("Sorting Algorithms")
        .resizable()
        .build();

    let mut state = State::new(NUM_ITEMS as u32);

    while !handle.window_should_close() {
        let mut draw = handle.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);

        // Draw array to screen
        let mut x = 0;
        for value in &state.array {
            draw.draw_rectangle(x, HEIGHT - (*value as i32 * LINE_MUL), LINE_WIDTH, HEIGHT, Color::WHITE);
            x += LINE_WIDTH
        }

        // GUI
        let shuffle = draw.gui_button(Rectangle::new(4., 4., 120., 24.), rstr!("SHUFFLE ARRAY").into());
        let bubble = draw.gui_button(Rectangle::new(4., 30., 120., 24.), rstr!("BUBBLE SORT").into());
        let merge = draw.gui_button(Rectangle::new(124., 30., 120., 24.), rstr!("MERGE SORT").into());

        // Handle GUI button presses
        if shuffle {
            state.shuffle();
        }
        if bubble {
            let start_time = Instant::now();
            bubble_sort(&mut state.array);
            let elapsed = start_time.elapsed();
            println!("Bubble sort completed in {}μs", elapsed.as_micros())
        }
        if merge {
            let start_time = Instant::now();
            merge_sort(&mut state.array);
            let elapsed = start_time.elapsed();
            println!("Bubble sort completed in {}μs", elapsed.as_micros())
        }
    }
}

struct State {
    array: Vec<u32>,
}

impl State {
    fn new(size: u32) -> State {
        let mut array = (1..=size).collect::<Vec<u32>>();
        array.shuffle(&mut thread_rng());
        State {
            array,
        }
    }

    fn shuffle(&mut self) {
        self.array.shuffle(&mut thread_rng());
    }
}