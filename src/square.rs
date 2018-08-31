use maze::Coor;

#[derive(Clone, Debug)]
pub struct Square {
    visited: bool,
    destinations: Vec<Coor>,
}

impl Square {
    pub fn new() -> Square {
        Square {
            visited: false,
            destinations: Vec::new(),
        }
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
