use rand::Rng;
use std::io::Write;

use std::collections::HashMap;
use std::{ thread, time };

fn main() {
  print!("\x1B[?25l");
  let grid_size: i16 = 90;
  let mut grid = generate_grid(grid_size);
  draw_grid(grid_size, &grid);

  let mut generation = 0;
  loop {
    let mut next_grid: HashMap<(i16, i16), i16> = Default::default();
    let keys: Vec<&(i16, i16)> = grid.keys().collect();

    for key in keys {
      let val = grid[key];
      let (x, y) = key;
      let mut pop = 0;
      for coord in &near_cell(*x, *y) {
        if grid.contains_key(coord) {
          pop += grid[coord];
        }
      }

      if val == 1 && (pop == 2 || pop == 3) {
        next_grid.insert(*key, 1);
        for coord in &near_cell(*x, *y) {
          if !next_grid.contains_key(coord) {
            next_grid.insert(coord.clone(), 0);
          }
        }
      }

      if val == 0 && pop == 3 {
        next_grid.insert(key.clone(), 1);
        for coord in &near_cell(*x, *y) {
          if !next_grid.contains_key(coord) {
            next_grid.insert(coord.clone(), 0);
          }
        }
      }
    }

    grid = next_grid;
    generation += 1;
    print!("\x1B[2J\x1B[H");
    draw_grid(grid_size, &grid);
    std::io::stdout().flush().unwrap();
    let total_pop: i16 = grid.values().sum();
    println!("Gen: {generation}\nTotal pop: {total_pop}");
    thread::sleep(
         time::Duration::from_millis(100)
       );
  }
}

fn generate_grid(grid_size: i16) -> HashMap<(i16, i16), i16> {
  let mut grid: HashMap<(i16, i16), i16> = Default::default();

  let mut coords: (i16, i16) = (0, 0);
  for _ in 0..50 {
    coords.0 = rand::thread_rng().gen_range(grid_size/ 4..grid_size - (grid_size / 4));
    coords.1 = rand::thread_rng().gen_range(grid_size/ 4..grid_size - (grid_size / 4));

    grid.insert((coords.0, coords.1),     1);
    grid.insert((coords.0 + 1, coords.1), 1);
    grid.insert((coords.0, coords.1 + 1), 1);

    for crd in near_cell(coords.0, coords.1) {
      if !grid.contains_key(&crd) { grid.insert(crd, 0); }
    }
    for crd in near_cell(coords.0 + 1, coords.1) {
      if !grid.contains_key(&crd) { grid.insert(crd, 0); }
    }
    for crd in near_cell(coords.0, coords.1 + 1) {
      if !grid.contains_key(&crd) { grid.insert(crd, 0); }
    }
  }
  grid
}
fn near_cell(x: i16, y: i16) -> Vec<(i16, i16)> {
  let mut coords: Vec<(i16, i16)> = vec![];
  coords.push((x + 1, y));
  coords.push((x , y + 1));
  coords.push((x + 1, y + 1));
  coords.push((x - 1, y - 1));
  coords.push((x - 1, y));
  coords.push((x, y - 1));
  coords.push((x + 1, y - 1));
  coords.push((x - 1, y + 1));
  coords
}
fn draw_grid(grid_size: i16, grid: &HashMap<(i16, i16), i16>) {
  let mut frame = String::with_capacity((grid_size * (grid_size + 1)) as usize);
  for y in 0..grid_size {
    for x in 0..grid_size {
      let key = (x, y);
      if grid.get(&key).copied().unwrap_or(0) == 1 {
        frame.push('█');
      } else {
        frame.push(' ');
      }
    }
    frame.push('\n');
  }
  print!("\x1B[2J\x1B[H{frame}");
  std::io::stdout().flush().unwrap();
}
