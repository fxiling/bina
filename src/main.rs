#[derive(Clone, PartialEq, Debug)]
struct Path {
    field: Field,
    direction: Direction,
    parent: Option<Box<Path>>,
}

#[derive(Clone, PartialEq, Debug)]
struct Field {
    position: (usize, usize),
    field_type: FieldType,
}

#[derive(Clone, PartialEq, Debug)]
enum FieldType {
    Road,
    Building,
    Dump,
    Car,
    PathNtoN,
    PathNtoE,
    PathWtoW,
    PathWtoN,
    PathEtoE,
    PathEtoS,
    PathStoS,
    PathStoW,
}

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    North,
    West,
    East,
    South,
}

//fn clear() {
//    let _ = std::process::Command::new("clear").status();
//}

fn read_file() -> Result<(Field, Vec<Vec<Field>>), std::io::Error> {
    let mut car = None;
    let mut field: Vec<Vec<Field>> = Vec::new();
    let mut y = 0;
    let file = std::fs::read_to_string("city.txt")?;
    for line in file.split("\n").collect::<Vec<&str>>() {
        let mut row: Vec<Field> = Vec::new();
        let mut x = 0;
        for char in line.chars() {
            let field = Field {
                position: (x, y),
                field_type: match char {
                    '.' => FieldType::Road,
                    'X' => FieldType::Building,
                    '@' => FieldType::Dump,
                    '%' => FieldType::Car,
                    _ => FieldType::Building,
                },
            };
            if field.field_type == FieldType::Car {
                car = Some(field.clone());
            };
            row.push(field);
            x += 1;
        }
        for i in x..100 {
            row.push(Field {
                position: (i, y),
                field_type: FieldType::Building,
            });
        }
        field.push(row);
        y += 1;
    }
    for i in y..100 {
        let mut row: Vec<Field> = Vec::new();
        for j in 0..100 {
            row.push(Field {
                position: (j, i),
                field_type: FieldType::Building,
            });
        }
        field.push(row);
    }
    Ok((car.expect("Invalid map!"), field))
}

fn can_move(current: &Field, field: &Vec<Vec<Field>>, direction: Direction) -> bool {
    match direction {
        Direction::North => {
            if current.position.1 != 0
                && field[current.position.1 - 1][current.position.0].field_type
                    != FieldType::Building
            {
                true
            } else {
                false
            }
        }
        Direction::West => {
            if current.position.0 != 0
                && field[current.position.1][current.position.0 - 1].field_type
                    != FieldType::Building
            {
                true
            } else {
                false
            }
        }
        Direction::East => {
            if current.position.0 != 99
                && field[current.position.1][current.position.0 + 1].field_type
                    != FieldType::Building
            {
                true
            } else {
                false
            }
        }
        Direction::South => {
            if current.position.1 != 99
                && field[current.position.1 + 1][current.position.0].field_type
                    != FieldType::Building
            {
                true
            } else {
                false
            }
        }
    }
}

fn bfs(map: Vec<Vec<Field>>, start: Field) -> Result<Vec<Path>, String> {
    let direction = if start.position.0 != 0
        && map[start.position.1][start.position.0 - 1].field_type == FieldType::Road
    {
        Direction::West
    } else if start.position.1 != 0
        && map[start.position.1 - 1][start.position.0].field_type == FieldType::Road
    {
        Direction::North
    } else if start.position.0 != 99
        && map[start.position.1][start.position.0 + 1].field_type == FieldType::Road
    {
        Direction::East
    } else if start.position.1 != 99
        && map[start.position.1 + 1][start.position.0].field_type == FieldType::Road
    {
        Direction::South
    } else {
        return Err(String::from("Invalid map!"));
    };
    let mut result = None;
    let mut queue: Vec<Path> = Vec::new();
    let mut checked: Vec<Path> = Vec::new();
    queue.push(Path {
        field: start.clone(),
        direction: direction.clone(),
        parent: None,
    });
    checked.push(Path {
        field: start,
        direction: direction,
        parent: None,
    });
    while !queue.is_empty() {
        //println!("c: {checked:?}");
        let current = queue.remove(0);
        //print!("q: [");
        //for x in queue.clone() {
        //    //print!("{:?} {:?}", x.field.position, x.direction);
        //}
        //println!("]");
        if current.field.field_type == FieldType::Dump {
            //println!("-------------------");
            result = Some(current);
            break;
        }
        match current.direction {
            Direction::North => {
                if can_move(&current.field, &map, Direction::North) {
                    let path = Path {
                        field: map[current.field.position.1 - 1][current.field.position.0].clone(),
                        direction: Direction::North,
                        parent: Some(Box::new(current.clone())),
                    };
                    if !checked.contains(&path) {
                        queue.push(path.clone());
                        checked.push(path);
                    }
                }
                if can_move(&current.field, &map, Direction::East) {
                    let path = Path {
                        field: map[current.field.position.1][current.field.position.0 + 1].clone(),
                        direction: Direction::East,
                        parent: Some(Box::new(current.clone())),
                    };
                    if !checked.contains(&path) {
                        queue.push(path.clone());
                        checked.push(path);
                    }
                }
            }
            Direction::West => {
                if can_move(&current.field, &map, Direction::North) {
                    let path = Path {
                        field: map[current.field.position.1 - 1][current.field.position.0].clone(),
                        direction: Direction::North,
                        parent: Some(Box::new(current.clone())),
                    };
                    if !checked.contains(&path) {
                        queue.push(path.clone());
                        checked.push(path);
                    }
                }
                if can_move(&current.field, &map, Direction::West) {
                    let path = Path {
                        field: map[current.field.position.1][current.field.position.0 - 1].clone(),
                        direction: Direction::West,
                        parent: Some(Box::new(current.clone())),
                    };
                    if !checked.contains(&path) {
                        queue.push(path.clone());
                        checked.push(path);
                    }
                }
            }
            Direction::East => {
                if can_move(&current.field, &map, Direction::East) {
                    let path = Path {
                        field: map[current.field.position.1][current.field.position.0 + 1].clone(),
                        direction: Direction::East,
                        parent: Some(Box::new(current.clone())),
                    };
                    if !checked.contains(&path) {
                        queue.push(path.clone());
                        checked.push(path);
                    }
                }
                if can_move(&current.field, &map, Direction::South) {
                    let path = Path {
                        field: map[current.field.position.1 + 1][current.field.position.0].clone(),
                        direction: Direction::South,
                        parent: Some(Box::new(current.clone())),
                    };
                    if !checked.contains(&path) {
                        queue.push(path.clone());
                        checked.push(path);
                    }
                }
            }
            Direction::South => {
                if can_move(&current.field, &map, Direction::West) {
                    let path = Path {
                        field: map[current.field.position.1][current.field.position.0 - 1].clone(),
                        direction: Direction::West,
                        parent: Some(Box::new(current.clone())),
                    };
                    if !checked.contains(&path) {
                        queue.push(path.clone());
                        checked.push(path);
                    }
                }
                if can_move(&current.field, &map, Direction::South) {
                    let path = Path {
                        field: map[current.field.position.1 + 1][current.field.position.0].clone(),
                        direction: Direction::South,
                        parent: Some(Box::new(current)),
                    };
                    if !checked.contains(&path) {
                        queue.push(path.clone());
                        checked.push(path);
                    }
                }
            }
        }
    }
    //println!("{result:?}");
    let mut resulting_path = Vec::new();
    match result {
        Some(element) => {
            resulting_path.push(element.clone());
            let mut current = element;
            while current.clone().parent != None {
                for elem in checked.clone() {
                    if Some(Box::new(elem.clone())) == current.clone().parent {
                        current = elem;
                        resulting_path.push(current.clone());
                    }
                }
            }
            resulting_path.reverse();
            Ok(resulting_path)
        }
        None => Err(String::from("Unable to complete!")),
    }
}

impl Path {
    fn get_child(&self, path: &Vec<Path>) -> Option<Path> {
        for x in path {
            if x.parent == Some(Box::new(self.clone())) {
                return Some(x.clone());
            }
        }
        None
    }
}

fn main() {
    let (start, mut field) = match read_file() {
        Ok(x) => x,
        Err(_) => return,
    };
    //println!("{start:?}");
    let path = bfs(field.clone(), start).unwrap();
    //println!("----------------");
    for step in path.clone() {
        field[step.field.position.1][step.field.position.0].field_type = match step.get_child(&path)
        {
            Some(child) => match child.direction {
                Direction::North => match step.direction {
                    Direction::West => FieldType::PathWtoN,
                    _ => FieldType::PathNtoN,
                },
                Direction::West => match step.direction {
                    Direction::South => FieldType::PathStoW,
                    _ => FieldType::PathWtoW,
                },
                Direction::East => match step.direction {
                    Direction::North => FieldType::PathNtoE,
                    _ => FieldType::PathEtoE,
                },
                Direction::South => match step.direction {
                    Direction::East => FieldType::PathEtoS,
                    _ => FieldType::PathStoS,
                },
            },
            None => match step.direction {
                Direction::North => FieldType::PathNtoN,
                Direction::West => FieldType::PathWtoW,
                Direction::East => FieldType::PathEtoE,
                Direction::South => FieldType::PathStoS,
            },
        }
    }
    //clear();
    for line in field {
        for x in line {
            match x.field_type {
                FieldType::Road => print!(". "),
                FieldType::Building => print!("X "),
                FieldType::Car => print!("% "),
                FieldType::Dump => print!("@ "),
                FieldType::PathNtoN | FieldType::PathStoS => print!("│ "),
                FieldType::PathWtoW | FieldType::PathEtoE => print!("──"),
                FieldType::PathNtoE => print!("┌─"),
                FieldType::PathWtoN => print!("└─"),
                FieldType::PathEtoS => print!("┐ "),
                FieldType::PathStoW => print!("┘ "),
            }
        }
        print!("\n");
    }
    //print!("\x1b[4;5Hp")
}
