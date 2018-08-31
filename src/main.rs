extern crate image;
extern crate imageproc;
extern crate rand;

mod maze;
mod square;

use maze::Maze;
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let mut width = 100;
    if let Some(arg) = args.next() {
        width = arg.parse::<usize>().expect("Invalid width value.");
    }
    let mut height = width;
    if let Some(arg) = args.next() {
        height = arg.parse::<usize>().expect("Invalid height value.");
    }

    let mut maze = Maze::new(width, height);
    maze.generate();
    maze.render()
        .save("out.png")
        .expect("Couldn't write image.");
}
