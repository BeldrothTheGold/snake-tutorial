use piston_window::{types::Color, Context, G2d};
use std::collections::LinkedList;

use crate::draw::Block;

const SNAKE_COLOR: Color = [0.0, 0.8, 0.0, 1.0];

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

pub struct Snake {
    direction: Direction,
    len: u32,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(head_x: u32, head_y: u32, len: u32, direction: Direction) -> Snake {
        let mut body = LinkedList::new();
        body.push_back(Block::new(head_x, head_y));
        Snake {
            direction,
            len,
            body,
            tail: None,
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            block.draw(SNAKE_COLOR, con, g);
        }
    }
    pub fn head(&self) -> &Block {
        self.body.front().unwrap()
    }
    pub fn move_fwd(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }
        // we need to add a new head to the body to simulate moving forward
        self.body.push_front(self.next_head(None));

        // if the length of the snake == the length of the body
        // we want to remove the last block in the linked list
        if self.body.len() > self.len as usize {
            self.tail = self.body.pop_back();
        }
    }
    pub fn next_head(&self, dir: Option<Direction>) -> Block {
        let mut direction = self.direction;
        if let Some(d) = dir {
            direction = d;
        }
        let head = self.head();
        match direction {
            Direction::Up => Block {
                x: head.x,
                y: head.y - 1,
            },
            Direction::Down => Block {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Block {
                x: head.x - 1,
                y: head.y,
            },
            Direction::Right => Block {
                x: head.x + 1,
                y: head.y,
            },
        }
    }
    pub fn direction(&self) -> Direction {
        self.direction.clone()
    }
    pub fn set_direction(&mut self, dir: Direction)  {
        self.direction = dir;
    }
    pub fn grow(&mut self) {
        self.len += 1;
    }
    pub fn check_collide_tail(&self, block: &Block) -> bool {
        let head = self.head();
        for bl in &self.body {
            if bl == head {
                continue;
            }
            if bl == block {
                return true;
            }
        }
        return false;
    }
}
