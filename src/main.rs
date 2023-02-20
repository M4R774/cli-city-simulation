extern crate term_size;
extern crate matrix;

use colored::Colorize;
use rand::seq::SliceRandom;
use rand::Rng;

// 0 = on_green, grassfield
// 1 = on_yellow, farm
// 2 = on_red, house
// 3 = on_gray, road
// 4 = on_blue, river
// 5 = on_brown, harvested farm

#[derive(Copy, Clone)]
struct Coordinates {
    x: usize, 
    y: usize,
}

fn main() {
    let (width, height) = term_size::dimensions().unwrap();
    let mut world_map: Vec<Vec<usize>> = create_empty_map(width, height);
    add_river_to_the_map(&mut world_map);
    let farm_coordinates = find_good_place_for_farm(&mut world_map);
    build_road_out_of_city(&mut world_map, farm_coordinates);
    draw_map(world_map);
}

fn build_road_out_of_city(world_map: &mut Vec<Vec<usize>>, farm_coordinates: Coordinates) {
    let exit_ramp_coordinates = Coordinates {
        x: world_map[0].len() / 2,
        y: world_map.len() - 1,
    };
    let mut road_cursor: Coordinates = exit_ramp_coordinates;
    while road_cursor.x != farm_coordinates.x || road_cursor.y != farm_coordinates.y {
        if world_map[road_cursor.y][road_cursor.x] != 2 {
            world_map[road_cursor.y][road_cursor.x] = 3;
        }
        if road_cursor.y > farm_coordinates.y {
            road_cursor.y -= 1
        } else if road_cursor.y < farm_coordinates.y {
            road_cursor.y += 1
        } else if road_cursor.x < farm_coordinates.x {
            road_cursor.x += 1
        } else if road_cursor.x > farm_coordinates.x {
            road_cursor.x -= 1
        }
    }
}

fn add_river_to_the_map(world_map: &mut Vec<Vec<usize>>) {
    let mut rng = rand::thread_rng();
    let river_start_x: usize = rng.gen_range(0..world_map[0].len() - 1);
    let mut river_cursor = Coordinates {x: river_start_x, y: 0};
    while river_cursor.y < world_map.len() {
        add_river_tile(&mut river_cursor, world_map, &mut rng);
    }
}

fn add_river_tile(river_cursor: &mut Coordinates, world_map: &mut Vec<Vec<usize>>, rng: &mut rand::rngs::ThreadRng) {
    if river_cursor.x < world_map[0].len() && river_cursor.y < world_map.len() {
        world_map[river_cursor.y][river_cursor.x] = 4;
    }
    let directions = ["West", "South", "East"];
    let flow_direction = directions.choose(rng);
    match flow_direction {
        Some(direction) => {
            // TODO: Better checking not to go out of bounds
            if *direction == "West" && river_cursor.x > 0 {
                river_cursor.x -= 1;
            } else if *direction == "South" {
                river_cursor.y += 1;
            } else if *direction == "East" {
                river_cursor.x += 1;
            }
        }
        None => {}
    }
}

fn find_good_place_for_farm(world_map: &mut Vec<Vec<usize>>) -> Coordinates {
    let mut good_places: Vec<Coordinates> = vec![];
    // TODO: Collect good places to vector and choose 1 randomly
    let radius: usize = 2;
    for (y, row) in world_map.iter().enumerate() {
        for (x, _world_tile) in row.iter().enumerate() {
            if empty_space_on_radius(radius, world_map, x, y) {
                let good_position: Coordinates = Coordinates { x, y };
                good_places.push(good_position);
            }
        }
    }

    match place_farm(good_places, radius, world_map) {
        Ok(value) => value,
        Err(value) => return value,
    }
}

fn place_farm(good_places: Vec<Coordinates>, radius: usize, world_map: &mut Vec<Vec<usize>>) -> Result<Coordinates, Coordinates> {
    let good_place = good_places.choose(&mut rand::thread_rng());
    Ok(match good_place {
        Some(coordinates) => {
            set_tile_on_radius(radius, world_map, coordinates.x, coordinates.y, 1);
            world_map[coordinates.y][coordinates.x] = 2;
            return Err(coordinates.clone());
        }
        None => { 
            println!("No good places found...");
            return Err(Coordinates {x: 0, y: 0});
        }
    })
}

fn set_tile_on_radius(radius: usize, world_map: &mut Vec<Vec<usize>>, center_x: usize, center_y: usize, tile_type: usize) {
    for y in center_y - radius..center_y + radius + 1 {
        for x in center_x - radius..center_x + radius + 1 {
            if world_map[y][x] == 0 {  // TODO: Add checking world map size
                world_map[y][x] = tile_type;
            }
        }
    }
}

fn empty_space_on_radius(radius: usize, world_map: &Vec<Vec<usize>>, center_x: usize, center_y: usize) -> bool {
    if radius > center_x || radius > center_y || world_map.len() < center_y + radius + 1 || world_map[0].len() < center_x + radius + 1 {
        return false;
    }
    for y in center_y - radius..center_y + radius + 1 {
        for x in center_x - radius..center_x + radius + 1 {
            if world_map[y][x] != 0 {
                return false;
            }
        }
    }
    return true;
}

fn draw_map(mut world_map: Vec<Vec<usize>>) {
    for (_i, row) in world_map.iter_mut().enumerate() {
        for (_j, world_tile) in row.iter_mut().enumerate() {
            if *world_tile == 0 {
                print!("{}", format!("{}", world_tile).on_green());
            } else if *world_tile == 1 {
                print!("{}", format!("{}", world_tile).on_yellow());
            } else if *world_tile == 2 {
                print!("{}", format!("{}", world_tile).on_red());
            } else if *world_tile == 3 {
                print!("{}", format!("{}", world_tile).on_bright_black());
            } else if *world_tile == 4 {
                print!("{}", format!("{}", world_tile).on_blue());
            } 
        }
        println!();
    }
}

fn create_empty_map(width: usize, height: usize) -> Vec<Vec<usize>> {
    println!("Width: {} Height: {}", width, height - 3);
    let world_map: Vec<Vec<usize>> = vec![vec![0; width]; height - 3];
    return world_map;
}
