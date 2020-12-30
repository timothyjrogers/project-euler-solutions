use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let fname = "data.txt";
    let f = File::open(fname).expect("Unable to open data file");
    let reader = BufReader::new(f);
    let vec: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut grid: Vec<Vec<u32>> = vec![];
    for v in vec {
        let mut row: Vec<u32> = vec![];
        for c in v.split(" ") {
            row.push(c.parse::<u32>().unwrap());
        }
        grid.push(row);
    }

    let mut max = 0;
    let col_size = grid.len();
    let row_size = grid[0].len();
    for i in 0..col_size {
        for j in 0..row_size {
            //east
            if j < row_size - 3 {
                let cur = grid[i][j] * grid[i][j + 1] * grid[i][j + 2] * grid[i][j + 3];
                if cur > max { max = cur };
            }
            //south
            if i < col_size - 3 {
                let cur = grid[i][j] * grid[i+1][j] * grid[i+2][j] * grid[i+3][j];
                if cur > max { max = cur };
            }
            //southeast
            if j < row_size - 3 && i < col_size - 3 {
                let cur = grid[i][j] * grid[i+1][j+1] * grid[i+2][j+2] * grid[i+3][j+3];
                if cur > max { max = cur };
            }
            //southwest
            if j > 2 && i < col_size - 3 {
                let cur = grid[i][j] * grid[i+1][j-1] * grid[i+2][j-2] * grid[i+3][j-3];
                if cur > max { max = cur };
            }
        }
    }
    println!("{}", max);
}