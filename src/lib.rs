use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn hello(){
    alert("Hello Mum!");
}

// u32 -> 2**32
// i32 -> +- 2**31
#[wasm_bindgen]
pub struct  Game{
    width : u32,
    height : u32,
    cells : Vec<u8>,
    seed : u32
}

#[wasm_bindgen]
impl Game {
    pub fn new(width: u32, height: u32, seed: u32) -> Game{
        return Game {
            width,
            height,
            cells: vec![0; (width*height) as usize],
            seed
        }
    }

    /*
    pub fn rand(&mut self, N: u32){
        // Set N-number of random cells to 1

        //let mut rng = rand::thread_rng();
        //let mut c = self.cells.clone(); //copy cell array
        for i in 0..N{
            //let r = rng.gen_range(0..(self.width * self.height));
            let mut r :[u8; 4];
            getrandom::getrandom(&mut r);
            self.cells[r as usize] = 1;
        }
    }
    */
    pub fn semi_rand(&mut self, _n : u32){
        // Set N-number of semi-random cells to 1
        //let mut rng = rand::thread_rng();
        for i in 0..(self.width * self.height){
            //let r = rng.gen_range(0..(self.width * self.height));
            if (i % _n) * (i % self.seed) == 0{
                self.cells[i as usize] = 1;
            }
            else{
                self.cells[i as usize] = 0;
            }
        }
    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Game {
    // ...

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u8{
        self.cells.as_ptr()
    }
}

/// Simulation logic
#[wasm_bindgen]
impl Game {
    pub fn tick(&mut self){
        let conv: [[i64; 2]; 8] = [[0, 1], [0, -1], [1, 0], [-1, 0], [-1, -1], [-1, 1], [1, -1], [1, 1]];
        for i in 0..self.width{
            for j in 0..self.height{
                let mut _sum = 0;
                for c in conv{
                    let _v = index((i as i64 + c[0]) as u32 % self.height, (j as i64 + c[1]) as u32 % self.width, self.width);
                    _sum += self.cells[_v];
                }
                let _idx = index(i, j, self.width);
                //let _top = index(i, (j - 1) % self.height, self.width);
                //let _bot = index(i, (j + 1) % self.height, self.width);
                //let _left = index((i - 1) % self.width, j, self.width);
                //let _right = index((i + 1) % self.width, j, self.width);
                //let s = self.cells[_top] + self.cells[_bot] + self.cells[_left] + self.cells[_right];
                if (_sum > 3) | (_sum < 2){
                    self.cells[_idx] = 0;
                }
                else{
                    if (self.cells[_idx] == 0) && (_sum == 2){
                        self.cells[_idx] = 0;
                        continue;
                    }
                    self.cells[_idx] = 1;
                }
            }
        }
    }
}

// index a grid as matrix
// 0,0 top left
// N, N bottom right
#[wasm_bindgen]
pub fn index(i : u32, j : u32, num_cols: u32) -> usize{
    return (i * num_cols + j) as usize;
}

