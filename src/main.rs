extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

const MAP_HEIGHT: usize = 48;
const MAP_WIDTH: usize = 64;
const TILE_SIZE: f64 = 10.0;

const COLOR_BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const COLOR_RED: [f32;4] = [1.0, 0.0, 0.0, 1.0];
const COLOR_GREEN: [f32;4] = [0.0, 1.0, 0.0, 1.0];
const COLOR_YELLOW: [f32;4] = [1.0,1.0,0.0,1.0];

const LENGTH_OF_TURN: f64 = 0.5; 

type Map = Vec<Vec<i32>>;

enum Direction {
    Up, Down, Left, Right
}

struct Snake {
    speed: f64,
    x: i32, 
    y: i32,
    size: i32,
    direction: Direction,
}

impl Snake {
    fn set_direction(&mut self, d: Direction) {
        self.direction = d;
    }
}

//finds a spot on the map for some delicious red square food
fn find_empty_spot(map: &Map) -> (i32, i32) {
   
    let mut x: i32;
    let mut y: i32;

    loop {
        x = rand::thread_rng().gen_range(0, MAP_WIDTH as i32 - 1);
        y = rand::thread_rng().gen_range(0, MAP_HEIGHT as i32 - 1);

        if map[x as usize][y as usize] == 0{
            break;
        }
    }

    (x, y)
}

//this is the basic game loop. It returns true on the case that the snake ate food or moved, or false if a move didn't happen
fn take_turn(map: &mut Map, snake: &mut Snake) -> bool{

    let (next_x, next_y) = match snake.direction {
        Direction::Up => (snake.x, snake.y + 1),
        Direction::Down => (snake.x, snake.y - 1),
        Direction::Left => (snake.x - 1, snake.y),
        Direction::Right => (snake.x+ 1, snake.y)
    };

    if next_x >= MAP_WIDTH as i32 || next_y >= MAP_HEIGHT as i32 || next_x < 0 || next_y < 0 {
        return false;
    }

    return match map[next_x as usize][next_y as usize] {
        -1 => {
            //eat and move
            snake.size = snake.size + 1;

            map[next_x as usize][next_y as usize] = snake.size;
            snake.x = next_x;
            snake.y = next_y;
            snake.speed = snake.speed + 5.0;

            let (food_x, food_y) = find_empty_spot(map);
            map[food_x as usize][food_y as usize] = -1; 

            true
        },
        0 => {
            //move
            for x in 0..MAP_WIDTH {
                for y in 0..MAP_HEIGHT {
                    if map[x as usize][y as usize] > 0 {
                        map[x as usize][y as usize] = map[x as usize][y as usize] - 1;
                    }             
                }
            }

            map[next_x as usize][next_y as usize] = snake.size;
            snake.x = next_x;
            snake.y = next_y;
            true
        },
        _ => {
            //dead
            false
        }
    }
 
}

fn main() {

    //initialize the map
    let mut map = vec![vec![0; MAP_HEIGHT]; MAP_WIDTH];

    //add a red square to nom on
    map[5][5] = -1;

    //initialize the snake
    let mut snake = Snake{
        speed: 10.0,
        x: 0,
        y: 0,
        size: 3,
        direction:Direction::Up
    };
 
    //setup the PistonWindow
    let mut window: PistonWindow = WindowSettings::new("piston snake!", [640, 480])
        .vsync(true).exit_on_esc(true).build().unwrap();

    let mut current_turn_time: f64 = 0.0;

    //The big event loop
    while let Some(event) = window.next() {

        //handle our update events 
        if let Some(u) = event.update_args() {

            //Values are arbitrary. This could have been smarter 
            current_turn_time = current_turn_time + u.dt * snake.speed;

            if current_turn_time >= LENGTH_OF_TURN {
                if take_turn(&mut map, &mut snake) == false {
                    break;
                }

                current_turn_time = 0.0;
            }
        }

        //handle our input events
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                //(0,0) is top left and not bottom right
                Key::Down => snake.set_direction(Direction::Up), 
                Key::Up => snake.set_direction(Direction::Down),
                Key::Left => snake.set_direction(Direction::Left),
                Key::Right => snake.set_direction(Direction::Right),
                _ => (),
            }
        }
        
        //how our drawing events
        window.draw_2d(&event, |context, graphics, _device| {
            
            clear([1.0; 4], graphics);

            //draw the game
            for x in 0..MAP_WIDTH {
                for y in 0..MAP_HEIGHT {
                    let f64_x = x as f64 * TILE_SIZE;
                    let f64_y = y as f64 * TILE_SIZE;
                    let f64_x2 = TILE_SIZE;
                    let f64_y2 = TILE_SIZE;
        
                    let tile_color = if map[x as usize][y as usize] == -1 {
                        COLOR_RED
                    }else if x as i32 == snake.x && y as i32 == snake.y {
                        COLOR_GREEN
                    }else if map[x as usize][y as usize] > 0 {
                        COLOR_YELLOW
                    }else {
                        COLOR_BLACK
                    };
                
                    rectangle(tile_color, 
                                [f64_x, f64_y, f64_x2, f64_y2],
                                context.transform, 
                                graphics);                    
                }
            }
        });
    }
}