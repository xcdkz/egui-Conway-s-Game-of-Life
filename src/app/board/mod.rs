use std::collections::HashSet;
use eframe::egui::{Color32, Rect, Shape, vec2, Rounding};
use std::{time::Duration};
use std::fs;
use instant::Instant;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos(pub i32, pub i32);

pub struct Board {
    pub fps: u32,
    pub speed: u128,
    cells: HashSet<Pos>,
    b_size: i32,
    last_frame_time: Instant,
    cell_size: f32,
}

impl Board {
    pub fn new() -> Self {
        Self {
            fps: 2,
            speed: Board::fps_to_speed(2.0),
            cells: HashSet::new(),
            b_size: 100,
            last_frame_time: Instant::now(),
            cell_size: 5.0
        }
    }

    pub fn neighbours(&self, p: &Pos) -> usize {
        self.cells.iter()
            .filter(|l| (l.0-p.0).abs() <= 1 && (l.1-p.1).abs() <= 1 && !(*l == p))
            .count()
    }

    pub fn fps_to_speed(fps: f32) -> u128 {
        Duration::new(0, (1000000000.0 / fps) as u32).as_millis()
    }

    pub fn update(&mut self) {
        let duration_since_last_frame = Instant::now().duration_since(self.last_frame_time);
        if duration_since_last_frame.as_millis().lt(&self.speed) {
            return;
        }
        let mut n_cells = HashSet::new();
        for col in 0..self.b_size {
            for row in 0..self.b_size {
                let n = self.neighbours(&Pos(col as i32, row as i32));
                if (n == 2 && self.cells.contains(&Pos(col as i32, row as i32))) || n == 3 {
                    n_cells.insert(Pos(col, row));
                }
            }
        }
        self.last_frame_time = Instant::now();
        self.cells = n_cells;
    }

    fn find_min(&self) -> (i32, i32) {
        let mut min_x = -1;
        let mut min_y = -1;
        for el in &self.cells {
            if min_x == -1 || el.0 < min_x {
                min_x = el.0;
            }
            if min_y == -1 || el.1 < min_y {
                min_y = el.1;
            }
        }
        (min_x, min_y)
    }

    fn find_max(&self) -> (i32, i32) {
        let mut max_x = -1;
        let mut max_y = -1;
        for el in &self.cells {
            if el.0 > max_x {
                max_x = el.0;
            }
            if el.1 > max_y {
                max_y = el.1;
            }
        }
        (max_x, max_y)
    }

    pub fn center_cells(&mut self) {
        let (min_x, min_y) = self.find_min();
        let (max_x, max_y) = self.find_max();
        let mut elems_c = HashSet::new();
        for el in &self.cells {
            elems_c.insert(Pos(self.b_size/2-(max_x-min_x)/2 + el.0, self.b_size/2-(max_y-min_y)/2 + el.1));
        }
        self.cells = elems_c;
    }

    pub fn generate_cells(&self, shapes: &mut Vec<Shape>, rect: Rect) {
        for c in &self.cells {
            shapes.push(Shape::rect_filled(
                Rect {
                    min: rect.min
                        + vec2(self.cell_size as f32 * c.0 as f32, self.cell_size as f32 * c.1 as f32),
                    max: rect.min
                        + vec2(self.cell_size as f32 * (c.0+1) as f32, self.cell_size as f32 * (c.1+1) as f32)
                },
                Rounding::none(),
                Color32::BLACK
            ));
        }
    }

    pub fn generate_from_file(&mut self, f: &str) {
        let contents = fs::read_to_string(f)
            .expect("Error reading from file");

        let lines: Vec<&str> = contents.split('\n').collect();

        let mut x = HashSet::new();
        for l in 0..lines.len() {
           for (i, c) in lines[l].chars().enumerate() {
               if c == '#' {
                   x.insert(Pos(i as i32, l as i32));
               }
           }
        }
        self.cells = x;
    }
}
