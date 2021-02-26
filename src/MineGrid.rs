
use rand::{Rng, thread_rng};



const  GRIDMAX  :usize = 10;
pub struct MineGrid{
    pub grid:[[i32;GRIDMAX];GRIDMAX],
    pub answer: [[i32;GRIDMAX];GRIDMAX],
}

impl MineGrid {

    pub fn new() -> MineGrid {
        let mut grid = [[0;GRIDMAX];GRIDMAX]; 
        let mut rng = thread_rng();
        let n = 10;
        let mut cnt = 0;
        while cnt <n {
            let x1:usize = rng.gen_range(0..10);
            let y1:usize = rng.gen_range(0..10);
            if grid[x1][y1]!=0 {
                continue;
            }
            cnt +=1;
            grid[x1][y1] = 1;
        }
        MineGrid{
            grid,
            answer:solve(grid,10),
        }
    }

}


pub fn print_grid(g:[[i32;GRIDMAX];GRIDMAX],n : usize){
    println!("***********************************");
    for i in 0..n{
        for j in 0..n{
            print!("{} ",g[i][j]);
        }
        println!();
    }
    println!("***********************************");
}

pub fn solve(g:[[i32;GRIDMAX];GRIDMAX],n : usize) -> [[i32;GRIDMAX];GRIDMAX]{
    let mut number = [[0;GRIDMAX];GRIDMAX]; 
    let dx :[i32;8] = [0,0,1,1,1,-1,-1,-1];
    let dy :[i32;8] = [1,-1,0,1,-1,0,1,-1];
    for i in 0..n{
        for j in 0..n{
            for k in 0..5{
                if g[i][j]==1{
                    number[i][j]=-1;
                    continue;
                }
                let nx = i as i32 +(dx[k]);
                let ny = j as i32 +(dy[k]);
                if nx>=0&&nx<(GRIDMAX as i32)&&ny>=0&&ny<(GRIDMAX as i32)&&g[nx as usize ][ny as usize]==1{
                    number[i][j]+=1;
                }
            }
        }
    }
    number
}