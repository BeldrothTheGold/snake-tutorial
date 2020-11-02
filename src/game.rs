use crate::draw::{Block, Rectangle};
use crate::snake::{Direction, Snake};
use piston_window::{types::Color, *};
use rand::{thread_rng, Rng};

const BOARD_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const FOOD_COLOR: Color = [0.0, 0.0, 0.85, 1.0];
const GAME_OVER_COLOR: Color = [0.90, 0.0, 0.0, 0.8];

const MOVE_DELAY: f64 = 0.10;
const FOOD_DELAY: f64 = 0.5;
const RESTART_TIME: f64 = 0.5;

const SNAKE_START_X: u32 = 2;
const SNAKE_START_Y: u32 = 2;
const SNAKE_START_LEN: u32 = 5;
const SNAKE_START_DIR: Direction = Direction::Right;

pub struct Game {
    snake: Snake,
    food: Option<Block>,
    window_rect: Rectangle,
    game_rect: Rectangle,
    game_over: bool,
    pending_change: bool,
    waiting_time: f64,
    food_time: f64,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            snake: Snake::new(
                SNAKE_START_X,
                SNAKE_START_Y,
                SNAKE_START_LEN,
                SNAKE_START_DIR,
            ),
            food: None,
            window_rect: Rectangle::new(0, 0, width, height),
            game_rect: Rectangle::new(1, 1, width - 2, height - 2),
            game_over: false,
            pending_change: false,
            waiting_time: 0.0,
            food_time: FOOD_DELAY,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.game_over {
            if self.waiting_time >= RESTART_TIME {
                self.restart();
            }
            return;
        }

        if let None = self.food {
            self.food_time += delta_time;
            if self.food_time > FOOD_DELAY {
                self.new_food();
            }
        }

        if self.waiting_time > MOVE_DELAY {
            self.waiting_time = 0.0;
            self.update_snake(None);
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {

        //draw game over
        if self.game_over {
            self.window_rect.draw(GAME_OVER_COLOR, con, g);
        }

        //draw game board
        self.game_rect.draw(BOARD_COLOR, con, g);

        //draw food
        if let Some(food) = self.food {
            food.draw(FOOD_COLOR, con, g);
        }

        //draw snake
        self.snake.draw(con, g);

    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over || self.pending_change {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::W => Some(Direction::Up),
            Key::S => Some(Direction::Down),
            Key::A => Some(Direction::Left),
            Key::D => Some(Direction::Right),
            _ => None,
        };

        if let Some(d) = dir {
            if d == self.snake.direction().opposite() {
                return;
            } else {
                self.snake.set_direction(d);
                self.pending_change = true;
            }
        }
    }

    fn gen_food(&self) -> Block {
        let mut rng = thread_rng();

        let x: u32 = rng.gen_range(1, self.game_rect.width);
        let y: u32 = rng.gen_range(1, self.game_rect.height);

        let mut block = Block::new(x, y);

        while self.snake.check_collide_tail(&block) {
            block.x = rng.gen_range(1, self.game_rect.width);
            block.y = rng.gen_range(1, self.game_rect.height);
        }

        block
    }

    fn snake_alive(&self, dir: Option<Direction>) -> bool {
        let next_head = self.snake.next_head(dir);
        self.game_rect.contains(&next_head)
            && !self.snake.check_collide_tail(&next_head)
    }

    fn new_food(&mut self) {
        self.food_time = 0.0;
        self.food = Some(self.gen_food());
    }

    fn snake_eating(&self) -> bool {
        if let Some(food) = self.food {
            &food == self.snake.head()
        } else {
            false
        }
    }
    fn update_snake(&mut self, dir: Option<Direction>) {
        self.pending_change = false;
        if self.snake_alive(dir) {
            self.snake.move_fwd(dir);
            if self.snake_eating() {
                self.snake.grow();
                self.food = None;
            }
        } else {
            self.game_over = true;
        }
    }
    fn restart(&mut self) {
        self.snake = Snake::new(
            SNAKE_START_X,
            SNAKE_START_Y,
            SNAKE_START_LEN,
            SNAKE_START_DIR,
        );
        self.game_over = false;
        self.pending_change = false;
        self.waiting_time = 0.0;
        self.food = Some(self.gen_food());
    }
}
