use std::time::Instant;

use rand::prelude::*;
use raylib::prelude::*;
use sorting::algorithms::bubble::bubble_sort;
use sorting::algorithms::merge::merge_sort;

fn main() {
    let mut state = State::new(512, 1024, 512);

    let (mut handle, thread) = init()
        .size(state.width, state.height)
        .title("Sorting Algorithms")
        .resizable()
        .build();

    while !handle.window_should_close() {
        // Change scaling if window resized
        if handle.is_window_resized() {
            state.update_size(handle.get_screen_width(), handle.get_screen_height())
        }

        let mut draw = handle.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);

        // Draw array to screen
        let mut x = 0;
        for value in &state.array {
            draw.draw_rectangle(
                x,
                state.height - (*value as i32 * state.line_mul),
                state.line_width,
                state.height,
                Color::WHITE,
            );
            x += state.line_width
        }

        // GUI
        let shuffle = draw.gui_button(
            Rectangle::new(4., 4., 120., 24.),
            Some(rstr!("SHUFFLE ARRAY")),
        );
        let bubble = draw.gui_button(
            Rectangle::new(4., 30., 120., 24.),
            Some(rstr!("BUBBLE SORT")),
        );
        let merge = draw.gui_button(
            Rectangle::new(124., 30., 120., 24.),
            Some(rstr!("MERGE SORT")),
        );

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
    size: u32,
    width: i32,
    height: i32,
    line_width: i32,
    line_mul: i32,
}

impl State {
    fn new(size: u32, width: i32, height: i32) -> State {
        let mut array = (1..=size).collect::<Vec<u32>>();
        array.shuffle(&mut thread_rng());
        State {
            array,
            size,
            width,
            height,
            line_width: width / size as i32,
            line_mul: height / size as i32,
        }
    }

    fn shuffle(&mut self) {
        self.array.shuffle(&mut thread_rng());
    }

    fn update_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
        self.line_width = width / self.size as i32;
        self.line_mul = height / self.size as i32;
    }
}
