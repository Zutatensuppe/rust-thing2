// LEVEL GENERATOR
//
//
// Start Area ->   room  --> room --> room -->    -> End Area (with nexus in it?) ?
//
//
//

use std::collections::HashMap;

use super::level::{Level, Room, Tile};

const ROOM_MIN_WIDTH: usize = 5;
const ROOM_MIN_HEIGHT: usize = 5;
const ROOM_MAX_WIDTH: usize = 15;
const ROOM_MAX_HEIGHT: usize = 15;

const GROUND: char = 'c';

pub fn add_random_room(level: &mut Level) -> bool {
    let room = Room {
        x: fastrand::usize(0..level.width + 1),
        y: fastrand::usize(0..level.height + 1),
        w: fastrand::usize(ROOM_MIN_WIDTH..ROOM_MAX_WIDTH + 1),
        h: fastrand::usize(ROOM_MIN_HEIGHT..ROOM_MAX_HEIGHT + 1),
        connections: HashMap::new(),
    };
    add_room(level, room)
}

pub fn add_room(level: &mut Level, room: Room) -> bool {
    if room.x + room.w > level.width {
        println!("LVL out of bounds");
        return false;
    }
    if room.y + room.h > level.height {
        println!("LVL out of bounds");
        return false;
    }
    for x in 0..room.w {
        for y in 0..room.h {
            if level.tiles[(room.y + y) * level.width + (room.x + x)].ch != ' ' {
                println!("SPACE already taken");
                return false;
            }
        }
    }

    for x in 0..room.w {
        for y in 0..room.h {
            let ch: char = if y == 0 || x == 0 || x == room.w - 1 || y == room.h - 1 {
                'a' // WALL
            } else {
                GROUND // GROUND
            };
            level.tiles[(room.y + y) * level.width + (room.x + x)] = Tile::from_char(ch);
        }
    }
    level.rooms.push(room);
    true
}

fn surround(level: &mut Level, x: usize, y: usize, ch: char) {
    let xstart = if x > 0 { x - 1 } else { x };
    let ystart = if y > 0 { y - 1 } else { y };
    let xend = if x < level.width - 1 { x + 1 } else { x };
    let yend = if y < level.height - 1 { y + 1 } else { y };

    for x in xstart..xend + 1 {
        for y in ystart..yend + 1 {
            if level.tiles[y * level.width + x].ch == ' ' {
                level.tiles[y * level.width + x] = Tile::from_char(ch);
            }
        }
    }
}

fn connect_rooms(level: &mut Level, i: usize, j: usize, ch: char, ch_wall: char) {
    if level.rooms[i].connections.keys().any(|&k| k == j) {
        println!("rooms {} and {} are already connected", i, j);
        return;
    }
    let center = level.rooms[i].center();
    let end_center = level.rooms[j].center();
    let start_x: usize = center.x.floor() as usize;
    let start_y: usize = center.y.floor() as usize;
    let end_x: usize = end_center.x.floor() as usize;
    let end_y: usize = end_center.y.floor() as usize;

    let mut y = start_y;
    let mut x = start_x;

    while x != end_x {
        level.tiles[y * level.width + x] = Tile::from_char(ch);
        surround(level, x, y, ch_wall);
        if x < end_x {
            x += 1;
        } else {
            x -= 1;
        }
    }
    while y != end_y {
        level.tiles[y * level.width + x] = Tile::from_char(ch);
        surround(level, x, y, ch_wall);
        if y < end_y {
            y += 1;
        } else {
            y -= 1;
        }
    }

    set_rooms_connected(level, i, j)
}

fn set_rooms_connected(level: &mut Level, i: usize, j: usize) {
    level.rooms[i].connections.insert(j, true);
    level.rooms[j].connections.insert(i, true);
}

fn are_rooms_connected(
    level: &Level,
    i: usize,
    j: usize,
    checked: &mut HashMap<usize, bool>,
) -> bool {
    for key in level.rooms[i].connections.keys() {
        if *key == j {
            return true;
        }
        if checked.keys().any(|k| k == key) {
            continue;
        }
        checked.insert(*key, true);
        if are_rooms_connected(level, *key, j, checked) {
            return true;
        }
    }
    return false;
}

pub fn generate_level(width: usize, height: usize, rooms: usize) -> Level {
    // start at a random position?
    // start in a random corner?
    println!("generating LVL");

    let mut level = Level {
        width,
        height,
        tiles: vec![],
        rooms: vec![],
    };

    // fill empty level
    for _ in 0..width * height {
        level.tiles.push(Tile::from_char(' '))
    }

    while level.rooms.len() < rooms {
        if add_random_room(&mut level) {
            println!("ROOM added")
        }
    }

    for i in 0..level.rooms.len() {
        // every room is connected to itself
        level.rooms[i].connections.insert(i, true);

        // connect room to another room
        let mut closest_idx = None;
        let mut closest_dist = None;
        let center = level.rooms[i].center();
        for j in 0..level.rooms.len() {
            if i == j {
                continue;
            }

            let dist = center.distance(level.rooms[j].center());
            if closest_dist.is_none() || closest_dist.unwrap() > dist {
                closest_dist = Some(dist);
                closest_idx = Some(j);
            }
        }

        if let Some(closest_idx) = closest_idx {
            connect_rooms(&mut level, i, closest_idx, GROUND, 'b');
        } else {
            // should not arrive here..
            panic!("no room found to connect to");
        }
    }

    for i in 0..level.rooms.len() {
        let mut checked = HashMap::new();
        if are_rooms_connected(&level, i, 0, &mut checked) {
            println!("room ALREADY connected to 0... {}", i);
            // room already connected to starting room
            continue;
        }

        // connect room to a closest one that is already connected to 0

        // connect room to another room
        let mut closest_idx = None;
        let mut closest_dist = None;
        let center = level.rooms[i].center();
        for j in 0..level.rooms.len() {
            if i == j {
                continue;
            }
            let mut checked = HashMap::new();
            if !are_rooms_connected(&level, j, 0, &mut checked) {
                // other room also has no connection to room 0
                continue;
            }

            let dist = center.distance(level.rooms[j].center());
            if closest_dist.is_none() || closest_dist.unwrap() > dist {
                closest_dist = Some(dist);
                closest_idx = Some(j);
            }
        }

        if let Some(closest_idx) = closest_idx {
            println!("room not connected yet to 0... {}", i);
            // println!("room not connected yet to 0... {}", i)
            connect_rooms(&mut level, i, closest_idx, GROUND, 'b');
        } else {
            // should not arrive here..
            panic!("no room found to connect to");
        }
    }

    level
}
