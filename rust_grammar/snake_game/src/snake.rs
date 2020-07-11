use std::collections::LinkedList; //LinkedList관련 라이브러리
use piston_window::{Context, G2d}; 
use piston_window::types::Color;

use draw::draw_block;

//Snake의 Color 지정
const SNAKE_COLOR : Color = [0.00, 0.00, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
//Move direction에 대한 열거형 상수 생성
pub enum Direction{
    Up,
    Down,
    Left, 
    Rigth,
}

//Direction에 대한 trait 생성
impl Direction { 
    //역상황에 대한 Control Setting
    pub fn opposite(&self) -> Direction{
        match *self{
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left, 
        }
    }
}

#[derive(Debug, Clone)]
struct Block{
    x : i32,
    y : i32,
}

pub struct Snake{
    direction : Direction,
    body : LinkedList<Block>,
    tail : Option<Block>,
}

impl Snake{
    //새 게임에 대한 설정
    pub fn new(x : i32, y : i32) -> Snake{
        let mut body : LinkedList<Block> = LinkedList::new();
        body.push_back(Block{x : x + 2, y});
        body.push_back(Block{x : x + 1, y});
        body.push_back(Block{x, y});
        Snake{
            direction : Direction::Rigth,
            body,
            tail : None,
        }
    }

    pub fn draw(&self, con : &Context, g : &mut G2d){
        for block in &self.body{
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    pub fn head_position(&self) -> (i32, i32){
        //front()
        // -> provides a reference to the front element of the list or none if there is not in now
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir : Option<Direction>){
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }
        let (last_x, last_y) : (i32, i32) = self.head_position();

        let new_block = match self.direction{
            Direction::Up => Block{
                x : last_x,
                y : last_y - 1,
            }
            Direction::Down => Block{
                x : last_x,
                y : last_y + 1,
            }
            Direction::Left => Block{
                x : last_x - 1,
                y : last_y,
            }
            Direction::Rigth => Block{
                x : last_x + 1,
                y : last_y,
            }
        };
        
        //이동 -> LinkedList인 Body의 front에 Push, Body의 back에서 pop가 이루어짐
        self.body.push_front(new Block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direciotn{
        self.direction
    }

    pub fn next_head(&self, dir : Option<Direction>) _> (i32, i32) {
        let (head_x, head_y) : (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir{
            Some(d) => moving_dir = d,
            None => (),
        }
        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x -1, head_y),
            Direction::Rigth => (head_x + 1, head_y),
        }

    }

}