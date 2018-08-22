extern crate rand;
extern crate image;
extern crate imageproc;

use std::env;
use rand::prelude::*;
use image::{RgbImage, Rgb};
use imageproc::drawing::draw_line_segment_mut;

type Coor = (usize, usize);

#[derive(Clone, Debug)]
struct Square {
    visited: bool,
    destinations: Vec<Coor>,
}

impl Square {
    pub fn new() -> Square {
        Square { visited: false, destinations: Vec::new()}
    }
    pub fn visit(&mut self) {
        self.visited = true;
    }
    pub fn add_dest(&mut self, dest: Coor) {
        self.destinations.push(dest);
    }
    pub fn dest(&self) -> &Vec<Coor> {
        &self.destinations
    }
    pub fn visited(&self) -> bool {
        self.visited
    }
}

#[derive(Debug)]
struct Maze {
    squares: Vec<Square>,
    width: usize,
    height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        assert!(width > 0);
        assert!(height > 0);
        Maze { squares: vec![Square::new(); width * height], width, height }
    }

    fn at(&self, (x, y): Coor) -> &Square {
        &self.squares[x + y*self.width]
    }

    fn at_mut(&mut self, (x, y): Coor) -> &mut Square {
        &mut self.squares[x + y*self.width]
    }

    pub fn generate(&mut self) {
        let mut rng = thread_rng();
        let mut stack = vec![(0, 0)];
        
        while stack.len() > 0 {
            let top = stack[stack.len()-1];
            self.at_mut(top).visit();

            let adj: Vec<Coor> = self.adjacent_squares(top).into_iter().filter(|&i| !self.at(i).visited()).collect();
            if adj.len() == 0 {
                 stack.pop();
                 continue;
            }
            
            let next = adj[rng.gen_range(0, adj.len())];
            self.at_mut(top).add_dest(next);
            self.at_mut(next).add_dest(top);

            stack.push(next);
        }
    }

    fn adjacent_squares(&self, (x, y): Coor) -> Vec<Coor> {
        let mut res = Vec::new();
        if x > 0 { res.push( (x-1, y) )}
        if x < self.width-1 { res.push( (x+1, y) )}
        if y > 0 { res.push( (x, y-1) )}
        if y < self.height-1 { res.push( (x, y+1) )}
        res
    }

    pub fn render(&self) -> RgbImage {
        let inner_square_side = 3;
        let square_side = 1+inner_square_side;

        let white = Rgb([255, 255, 255]);
        let black = Rgb([0, 0, 0]);

        let mut out = RgbImage::from_pixel(self.width as u32 * square_side + 1, self.height as u32 * square_side + 1, white);
        for x in 0..self.width {
            for y in 0..self.height {
                let square = self.at((x,y));
                let xf = x as f32;
                let yf = y as f32;
                let ssf = square_side as f32;
                if y == 0 || !square.dest().contains(&(x, y-1)) {
                    draw_line_segment_mut(
                        &mut out,
                        (xf * ssf, yf * ssf),
                        ((xf+1.)*ssf, yf*ssf),
                        black);
                }
                if y == self.height-1 || !square.dest().contains(&(x, y+1)) {
                    draw_line_segment_mut(
                        &mut out,
                        (xf * ssf, (yf+1.) * ssf),
                        ((xf+1.)*ssf, (yf+1.)*ssf),
                        black);
                }
                if x == self.width-1 || !square.dest().contains(&(x+1, y)) {
                    draw_line_segment_mut(
                        &mut out,
                        ((xf+1.) * ssf, yf * ssf),
                        ((xf+1.)*ssf, (yf+1.)*ssf),
                        black);
                }
                if x == 0 || !square.dest().contains(&(x-1, y)) {
                    draw_line_segment_mut(
                        &mut out,
                        (xf * ssf, yf * ssf),
                        (xf*ssf, (yf+1.)*ssf),
                        black);
                }
            }
        }
        out
    }
}


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
    maze.render().save("out.png").expect("Couldn't write image.");
}
