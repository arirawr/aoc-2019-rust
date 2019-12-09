/* ----- Day 3 ----- */
use crate::aoc;

/* STRUCTS */
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
    Undefined
}

#[derive(Copy, Clone, Debug)]
struct DirectionInstruction {
    direction: Direction,
    distance: u32
}

// Coordinates can be negative
#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct TwoInts (u32,u32);

#[derive(Copy, Clone, Debug)]
enum LineSegment {
    LineSegmentHorizontal {
        y: i32,
        x_min: i32,
        x_max: i32
    },
    LineSegmentVertical {
        x: i32,
        y_min: i32,
        y_max: i32
    }
}

/* MAIN */
pub fn aoc_3_1() -> u32 {
    let data = parse_data();
    closest(find_matches(&data))
}

pub fn aoc_3_2() -> u32 {
    let data = parse_data();
    
    let steps: Vec<TwoInts> = find_matches(&data).iter().map(|c| {
        TwoInts(steps_until(c, &data[0]), steps_until(c, &data[1]))
    }).collect();

    steps.iter().map(|x| x.0 + x.1).min().unwrap()
}

/* HELPERS */

// Convert data from one large string into 2 vectors of DirectionInstructions
fn parse_data() -> Vec<Vec<DirectionInstruction>> {
    let string = aoc::util::read_input(3).unwrap();
    let mut data = Vec::new();
    for line in string.split("\n") {
        let directions = strings_to_directions(line.split(",").collect());
        data.push(directions);
    }
    data
}

fn strings_to_directions(input: Vec<&str>) -> Vec<DirectionInstruction> {
    let mut direction_set = Vec::new();
    for i in input {
        // Split strings (ie. "R324") and make a DirectionInstruction (ie. { "Right", 324 })
        let substrings = i.split_at(1);
        direction_set.push(DirectionInstruction {
            direction: match substrings.0 {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => Direction::Undefined
            },
            distance: substrings.1.parse().unwrap()
        });
    }
    direction_set
}

fn get_line_segments(directions: &Vec<DirectionInstruction>) -> Vec<LineSegment> {
    let mut segments = Vec::new();
    let mut current_coord = Coordinate { x: 0, y: 0 };

    for d in directions {
        let dist = d.distance as i32;
        match d.direction {
            Direction::Up => { 
                segments.push(
                    LineSegment::LineSegmentVertical {
                        x: current_coord.x,
                        y_min: current_coord.y,
                        y_max: (current_coord.y + dist)
                    }
                );
                current_coord.y += dist;
            }
            Direction::Down => { 
                segments.push(
                    LineSegment::LineSegmentVertical {
                        x: current_coord.x,
                        y_min: current_coord.y - dist,
                        y_max: current_coord.y
                    }
                );
                current_coord.y -= dist;
            }
            Direction::Right => { 
                segments.push(
                    LineSegment::LineSegmentHorizontal {
                        y: current_coord.y,
                        x_min: current_coord.x,
                        x_max: current_coord.x + dist
                    }
                );
                current_coord.x += dist;
            }
            Direction::Left => { 
                segments.push(
                    LineSegment::LineSegmentHorizontal {
                        y: current_coord.y,
                        x_min: current_coord.x - dist,
                        x_max: current_coord.x
                    }
                );
                current_coord.x -= dist;
            }
            _ => ()
        }
    }
    segments
}

fn steps_until(point: &Coordinate, directions: &Vec<DirectionInstruction>) -> u32 {
    let mut steps: u32 = 0;
    let mut current_coord = Coordinate { x: 0, y: 0 };

    for d in directions {
        let dist = d.distance;
        match d.direction {
            Direction::Up => { 
                for _ in 0..dist {
                    current_coord.y += 1;
                    steps += 1;
                    if current_coord.x == point.x && current_coord.y == point.y {
                        return steps
                    }
                } 
            },
            Direction::Down => { 
                for _ in 0..dist {
                    current_coord.y -= 1;
                    steps += 1;
                    if current_coord.x == point.x && current_coord.y == point.y {
                        return steps
                    }
                }
            },
            Direction::Right => { 
                for _ in 0..dist {
                    current_coord.x += 1;
                    steps += 1;
                    if current_coord.x == point.x && current_coord.y == point.y {
                        return steps
                    }
                }
            },
            Direction::Left => { 
                for _ in 0..dist {
                    current_coord.x -= 1;
                    steps += 1;
                    if current_coord.x == point.x && current_coord.y == point.y {
                        return steps
                    }
                }
            },
            _ => ()
        }
    }
    steps
}

fn find_matches(data: &Vec<Vec<DirectionInstruction>>) -> Vec<Coordinate> {
    let mut matches = Vec::new();
    for a in get_line_segments(&data[0]) {
        for b in get_line_segments(&data[1]) {
            match a {
                LineSegment::LineSegmentHorizontal { y, x_min, x_max } => {
                    match b {
                        LineSegment::LineSegmentVertical { x, y_min, y_max } => {
                            if y >= y_min && y <= y_max && x >= x_min && x <= x_max {
                                matches.push(Coordinate{x: x.abs(), y: y.abs()});
                            }
                        },
                        _ => ()
                    }
                },
                LineSegment::LineSegmentVertical { x, y_min, y_max } => {
                    match b {
                        LineSegment::LineSegmentHorizontal { y, x_min, x_max } => {
                            if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
                                matches.push(Coordinate{x: x, y: y});
                            }
                        },
                        _ => ()
                    }
                },
            }
        }
    }
    matches
}

fn closest(coords: Vec<Coordinate>) -> u32 {
    let mut min_val = coords[0].x.abs() + coords[0].y.abs();
    for c in coords {
        let next_val = c.x.abs() + c.y.abs();
        if min_val > next_val { min_val = next_val; }
    }
    min_val as u32
}
