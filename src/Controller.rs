use crate::MineGrid::{MineGrid, print_grid};
use piston::GenericEvent;
use std::ops::Deref;
use std::usize::MAX;

const MAXSIZE:i32 = 10;
const DX:[i32;8] = [0,0,1,1,1,-1,-1,-1];
const DY:[i32;8] = [1,-1,0,1,-1,0,1,-1];

pub struct MineGridController{
    mine_grid:MineGrid,
    cursor_pos:[f64;2],
    vis:[[i32;MAXSIZE as usize ];MAXSIZE as usize],
    state:State,
    cnt:i32,
}

enum  State{
    WIN,
    LOSE,
    PENDING,
}

impl MineGridController{
    pub fn new(minegrid:MineGrid) -> MineGridController{
        MineGridController{
            mine_grid:minegrid,
            cursor_pos:[0.0;2],
            vis:[[0;MAXSIZE as usize];MAXSIZE as usize],
            state:State::PENDING,
            cnt:100,
        }
    }
    pub fn solve_event<E:GenericEvent>(& mut self,position:[f64;2],size:f64,e:&E){
        use piston::input::{Button,Key,MouseButton};
        if let Some(now_cursor) = e.mouse_cursor_args(){
            self.cursor_pos = now_cursor;
        }
        if let Some(Button::Mouse((MouseButton::Left))) = e.press_args(){
            let x = self.cursor_pos[0] - position[0];
            let y = self.cursor_pos[1] - position[1];
            if x>=0.0 && y>=0.0 && x< size && y< size{
                let cell_x = (x/size *10.0) as usize;
                let cell_y = (y/size *10.0) as usize;
                println!("click {},{}",cell_x,cell_y);
                self.dfs(cell_x,cell_y);
            }
            if self.cnt==10{
                self.state = State::WIN;
            }
            print_grid(self.vis,MAXSIZE as usize);
        }

    }
    fn dfs(& mut self,x:usize,y:usize){
        self.vis[x][y] = 1;
        self.cnt-=1;
        if self.mine_grid.grid[x][y]==1{
            self.state=State::LOSE;
            return ();
        }
        if self.mine_grid.answer[x][y]!=0{
            return ();
        }
        for i in 0..8{
            let nx = DX[i]+x as i32;
            let ny = DY[i]+y as i32;
            if nx >= 0&& ny>=0 && nx <MAXSIZE && ny < MAXSIZE &&self.mine_grid.answer[nx as usize][ny as usize]!=-1&&self.vis[nx as usize][ny as usize]==0{
                self.dfs(nx as usize,ny as usize);
            }
        }
    }
    pub fn visit(&self,x:usize,y:usize) -> bool{
        if self.vis[x][y]==1 {return true;}
        false
    }
    pub fn get_char(&self,x:usize,y:usize) ->char{
        let x = self.mine_grid.answer[x][y];
         match  x {
             -1 => '*' ,
             0 => ' ',
             _ => char::from( x as u8 + '0' as u8),
         }
    }
    pub fn check_state(&self) -> i32{
        match self.state {
            State::PENDING => 0,
            State::LOSE => -1,
            State::WIN => 1,
        }
    }
}
