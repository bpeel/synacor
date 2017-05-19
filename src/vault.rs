#[derive(Copy, Clone)]
enum Room {
    Add,
    Subtract,
    Multiply,
    Number(i32)
}

impl Room {
    fn name(self) -> char {
        match self {
            Room::Add => '+',
            Room::Subtract => '-',
            Room::Multiply => '*',
            _ => panic!()
        }
    }
}

const WIDTH: usize = 4;
const HEIGHT: usize = 4;

const ROOMS: [Room; WIDTH * HEIGHT] = [
    Room::Multiply, Room::Number(8), Room::Subtract, Room::Number(1),
    Room::Number(4), Room::Multiply, Room::Number(11), Room::Multiply,
    Room::Add, Room::Number(4), Room::Subtract, Room::Number(18),
    Room::Number(0), Room::Subtract, Room::Number(9), Room::Multiply,
];

#[derive(Copy, Clone)]
struct State {
    x: i8,
    y: i8,
    direction: i8
}

fn apply_op(a: i32, op: Room, b: i32) -> i32 {
    match op {
        Room::Add => a + b,
        Room::Multiply => a * b,
        Room::Subtract => a - b,
        Room::Number(_) => panic!()
    }
}

fn print_solution(queue: &Vec<State>)  {
    let mut weight = 22;
    let mut last_op = Room::Add;

    for state in queue {
        let direction_name = match state.direction {
            0 => "north",
            1 => "south",
            2 => "west",
            3 => "east",
            -1 => "(end)",
            n => panic!("unknown direction {}", n)
        };

        print!("{}", direction_name);

        match ROOMS[state.x as usize + state.y as usize * WIDTH] {
            Room::Number(n) => {
                weight = apply_op(weight, last_op, n);
                print!(" // {} {} = {}", last_op.name(), n, weight);
            },
            ref op => last_op = *op
        }

        println!();
    }
}

fn try_solution(queue: &Vec<State>) -> bool {
    let mut weight = 22;
    let mut last_op = Room::Add;

    for state in queue {
        match ROOMS[state.x as usize + state.y as usize * WIDTH] {
            Room::Number(n) => {
                weight = apply_op(weight, last_op, n)
            },
            ref op => last_op = *op
        }
    }

    weight == 30
}

fn solve(max_depth: usize) -> bool {
    let mut queue = Vec::<State>::new();

    queue.push(State { x: 0, y: 3, direction: -1 });

    while queue.len() > 0 {
        let state = *queue.last().unwrap();

        if state.x == 3 && state.y == 0 {
            if try_solution(&queue) {
                print_solution(&queue);
                return true;
            }
            queue.pop();
            continue;
        }

        queue.pop();

        if queue.len() + 2 <= max_depth {
            for next_direction in state.direction + 1 .. 4 {
                let (nx, ny) = match next_direction {
                    0 => {
                        if state.y <= 0 {
                            continue
                        }
                        (state.x, state.y - 1)
                    },
                    1 => {
                        if state.y as usize >= HEIGHT - 1 {
                            continue
                        }
                        (state.x, state.y + 1)
                    },
                    2 => {
                        if state.x <= 0 {
                            continue
                        }
                        (state.x - 1, state.y)
                    },
                    3 => {
                        if state.x as usize >= WIDTH - 1 {
                            continue
                        }
                        (state.x + 1, state.y)
                    },
                    _ => panic!()
                };
                queue.push(State {
                    x: state.x,
                    y: state.y,
                    direction: next_direction
                });
                queue.push(State {
                    x: nx,
                    y: ny,
                    direction: -1
                });
                break
            }
        }
    }

    false
}

fn main() {
    for max_depth in 2.. {
        if solve(max_depth) {
            break;
        }
    }
}
