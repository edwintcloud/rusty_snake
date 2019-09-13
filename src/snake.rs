use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::draw_block;

/// SNAKE_COLOR is the color of the snake.
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0]; // rgba

/// Direction contains the different directions a snake can traverse.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Direction METHODS -|
impl Direction {
    /// Get the opposite of the current direction.
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

/// Block represents a draw block.
#[derive(Clone, Debug)]
struct Block {
    x: i32,
    y: i32,
}

/// Snake represents our snake.
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

// Snake METHODS -|
impl Snake {
    /// Draw the current Snake to the screen by adding
    /// all of its Blocks to the graphics buffer.
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    /// Get the head of the current Snake as a tuple of (x, y);
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    /// Advance the current Snake foward in its current Direction.
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // ensure dir is a valid direction
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }
        // get the current head position
        let (last_x, last_y): (i32, i32) = self.head_position();

        // create new block in the appropriate Direction
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        // push the new block to the front of our Snake
        self.body.push_front(new_block);

        // remove the last block (so it appears we are moving)
        let removed_block = self.body.pop_back().unwrap();

        // update the tail
        self.tail = Some(removed_block);
    }

    /// Get the Direction of the current Snake.
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    /// Get the next head position of the current Snake
    /// based on its current Direction.
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();
        let mut cur_direction = self.direction;

        match dir {
            Some(d) => cur_direction = d,
            None => (),
        }

        match cur_direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    /// Add the tail block back the body of the current Snake.
    pub fn restore_tail(&mut self) {
        let tail_block = self.tail.clone().unwrap();
        self.body.push_back(tail_block)
    }

    /// Check to see if current Snack has run into itself (collision).
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut acc = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            acc += 1;

            if acc == self.body.len() - 1 {
                break;
            }
        }
        false
    }
}

// Snake RELATED FUNCTIONS -|
impl Snake {
    /// Create a new Snake at position {x}, {y}.
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block { x: x + 2, y });

        body.push_back(Block { x: x + 1, y });

        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }
}
