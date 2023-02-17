extern crate term_size;
extern crate matrix;

use colored::Colorize;

fn main() {
    let (width, height) = term_size::dimensions().unwrap();
    let mut world_map: Vec<Vec<i32>> = create_empty_map(width, height);
    for (_i, row) in world_map.iter_mut().enumerate() {
        for (_j, world_tile) in row.iter_mut().enumerate() {
            print!("{}", format!("{}", world_tile).on_green());
        }
        println!();
    }
}

fn create_empty_map(width: usize, height: usize) -> Vec<Vec<i32>> {
    println!("Width: {} Height: {}", width, height);
    let world_map: Vec<Vec<i32>> = vec![vec![0; width]; height - 2];
    return world_map;
}
