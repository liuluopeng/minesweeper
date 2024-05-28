mod utils;

use rand::prelude::SliceRandom;
use rand::thread_rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Grid {
    Safe0 = 0,
    Safe1 = 1,
    Safe2 = 2,
    Safe3 = 3,
    Safe4 = 4,
    Safe5 = 5,
    Safe6 = 6,
    Safe7 = 7,
    Safe8 = 8,
    Mine = 9,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum SafeGrid {
    Safe0,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    Close = 1,
    Flag = 2,
    Open = 3,
    Boom = 4,
}

// 模式: 开图安全按钮.
// 玩法: 自己绘图玩法.
#[wasm_bindgen]
pub struct GameMap {
    width: usize,
    height: usize,
    map1d: Vec<Grid>,
    status1d: Vec<Status>, // 状态: 格子打开  格子关闭   格子标记
    total_mine: usize,
}
#[wasm_bindgen]
impl GameMap {
    pub fn new(width: usize, height: usize, total_mine: usize, fist_step: bool) -> Self {
        let quick_find = vec![
            Grid::Safe0,
            Grid::Safe1,
            Grid::Safe2,
            Grid::Safe3,
            Grid::Safe4,
            Grid::Safe5,
            Grid::Safe6,
            Grid::Safe7,
            Grid::Safe8,
        ];
        let area = width * height;
        let mut map1d = vec![Grid::Safe0; area];

        // 在地图中随便埋入地雷
        let mut random_index_list: Vec<usize> = (0..width * height).collect();
        let mut rng = thread_rng();
        random_index_list.shuffle(&mut rng);
        for &index in &random_index_list[..total_mine] {
            map1d[index] = Grid::Mine;
        }

        // 遍历 标上数字
        for i in 0..height {
            for j in 0..width {
                let index = i * width + j;

                if map1d[index] != Grid::Mine {
                    let counter = GameMap::surr_mine(&map1d, i, j, width, height);
                    // 统计周围的雷
                    println!("{:?} {:?} 周围: {:?} ", i, j, counter);

                    // [0 1 2 3 4 5 6 7 8 ]
                    // [safe0 ...    safe8]
                    map1d[index] = quick_find[counter];

                    match counter {
                        _ => {}
                    }
                }
            }
        }

        let mut status_list = vec![Status::Close; area];

        GameMap {
            width: width,
            height: height,
            map1d: map1d,
            status1d: status_list,
            total_mine: total_mine,
        }
    }

    pub fn new2(width: usize, height: usize, mine_list: Vec<usize>) -> Self {
        let quick_find = vec![
            Grid::Safe0,
            Grid::Safe1,
            Grid::Safe2,
            Grid::Safe3,
            Grid::Safe4,
            Grid::Safe5,
            Grid::Safe6,
            Grid::Safe7,
            Grid::Safe8,
        ];
        let area = width * height;
        let mut map1d = vec![Grid::Safe0; area];

        let mut total_mine = 0;

        // 在地图中
        for i in 0..width * height {
            if mine_list[i] == 1 {
                map1d[i] = Grid::Mine;
                total_mine += 1;
            }
        }

        // 遍历 标上数字
        for i in 0..height {
            for j in 0..width {
                let index = i * width + j;

                if map1d[index] != Grid::Mine {
                    let counter = GameMap::surr_mine(&map1d, i, j, width, height);
                    // 统计周围的雷
                    println!("{:?} {:?} 周围: {:?} ", i, j, counter);

                    // [0 1 2 3 4 5 6 7 8 ]
                    // [safe0 ...    safe8]
                    map1d[index] = quick_find[counter];

                    match counter {
                        _ => {}
                    }
                }
            }
        }

        let mut status_list = vec![Status::Close; area];

        GameMap {
            width: width,
            height: height,
            map1d: map1d,
            status1d: status_list,
            total_mine: total_mine,
        }
    }

    pub fn peek(&self) -> String {
        let mut msg = format!(
            "{:?}  {:?} \n {:?}",
            self.width, self.height, self.total_mine
        );

        msg
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn still_alive(&self) -> bool {
        let mut res = true;

        let d1 = self.get_bottom_1d();
        let d2 = self.get_surface_1d();

        for k in 0..self.width * self.height {
            if d2[k] == Status::Open && d1[k] == Grid::Mine {
                return false;
            }
        }

        true
    }

    pub fn show_all_mine(&mut self) {
        for k in 0..self.width * self.height {
            if self.map1d[k] == Grid::Mine {
                self.status1d[k] = Status::Open;
            }
        }
    }

    //
    pub fn count_flag(&self) -> usize {
        let mut count = 0;

        for &i in &self.status1d {
            if i == Status::Flag {
                count += 1;
            }
        }

        count
    }

    pub fn can_I_win(&self) -> bool {
        //
        for i in 0..self.width * self.height {
            if self.status1d[i] == Status::Close {
                return false;
            }

            if self.map1d[i] == Grid::Mine && self.status1d[i] != Status::Flag {
                return false;
            }
        }

        true
    }

    pub fn get_bottom_1d(&self) -> Vec<Grid> {
        self.map1d.clone()
    }

    pub fn get_surface_1d(&self) -> Vec<Status> {
        self.status1d.clone()
    }

    pub fn get_hover_1d(&self) -> Vec<String> {
        let mut res = vec![];

        let d1 = self.get_bottom_1d();
        let d2 = self.get_surface_1d();

        for k in 0..self.width * self.height {
            let mut elem = String::new();
            if d2[k] == Status::Close {
                elem = String::from("🎭");
            } else if d2[k] == Status::Open {
                if d1[k] == Grid::Mine {
                    elem = String::from("💣");
                } else {
                    let quick = vec!["0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣"];
                    let quick = vec![" ", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣"];
                    elem = String::from(quick[d1[k] as usize]);
                }
            } else if d2[k] == Status::Flag {
                elem = String::from("🚩");
            }
            res.push(elem);
        }

        res
    }

    // 是否打开
    pub fn opened(&self, row: usize, col: usize) -> bool {
        self.status1d[self.get_index(row, col)] == Status::Open
    }

    // 把周围一圈找到. 遇到数字停止
    fn get_all_safe_around(&self, row: usize, col: usize) -> Vec<usize> {
        let mut res = vec![];

        let mut memo_visited = vec![false; self.height * self.width];

        // res.push(self.get_index(row, col));

        GameMap::find(&self, row, col, &mut res, &mut memo_visited);

        // 现在 加一圈
        for inner_0_index in res.clone() {
            let (i, j) = self.get_i_j(inner_0_index);

            for delta_i in [-1, 0, 1].iter().cloned() {
                for delta_j in [-1, 0, 1].iter().cloned() {
                    // 排除自己
                    if delta_i == 0 && delta_j == 0 {
                        continue;
                    }

                    // 排除边角
                    if i == 0 && delta_i == -1 {
                        continue;
                    }
                    if j == 0 && delta_j == -1 {
                        continue;
                    }
                    if i == self.height - 1 && delta_i == 1 {
                        continue;
                    }
                    if j == self.width - 1 && delta_j == 1 {
                        continue;
                    }

                    let index = (i as i32 + delta_i) * self.width as i32 + (j as i32 + delta_j);
                    let index = index as usize;

                    if memo_visited[index] == false && self.map1d[index] != Grid::Safe0 {
                        res.push(index);
                        memo_visited[index] = true;
                    }
                }
            }
        }

        res.sort();
        res
    }
    fn find(&self, row: usize, col: usize, res: &mut Vec<usize>, memo_visited: &mut Vec<bool>) {
        let j = col;
        let i = row;
        let width = self.width;
        let height = self.height;

        for delta_i in [-1, 0, 1].iter().cloned() {
            for delta_j in [-1, 0, 1].iter().cloned() {
                // 排除自己
                if delta_i == 0 && delta_j == 0 {
                    continue;
                }

                // 排除边角
                if i == 0 && delta_i == -1 {
                    continue;
                }
                if j == 0 && delta_j == -1 {
                    continue;
                }
                if i == height - 1 && delta_i == 1 {
                    continue;
                }
                if j == width - 1 && delta_j == 1 {
                    continue;
                }

                if delta_i != 0 && delta_j != 0 {
                    continue; // 存在对角线
                }

                let index = (i as i32 + delta_i) * width as i32 + (j as i32 + delta_j);
                let index = index as usize;

                if memo_visited[index] == false && self.map1d[index] == Grid::Safe0 {
                    res.push(index);
                    memo_visited[index] = true;

                    //find
                    let (next_row, next_col) = self.get_i_j(index);
                    self.find(next_row, next_col, res, memo_visited)
                }
            }
        }
    }

    // 统计周围几个地雷
    fn surr_mine(map1d: &Vec<Grid>, i: usize, j: usize, width: usize, height: usize) -> usize {
        let mut counter = 0;

        for delta_i in [-1, 0, 1].iter().cloned() {
            for delta_j in [-1, 0, 1].iter().cloned() {
                // 排除自己
                if delta_i == 0 && delta_j == 0 {
                    continue;
                }

                // 排除边角
                if i == 0 && delta_i == -1 {
                    continue;
                }
                if j == 0 && delta_j == -1 {
                    continue;
                }
                if i == height - 1 && delta_i == 1 {
                    continue;
                }
                if j == width - 1 && delta_j == 1 {
                    continue;
                }

                let index = (i as i32 + delta_i) * width as i32 + (j as i32 + delta_j);
                let index = index as usize;
                if map1d[index] == Grid::Mine {
                    counter += 1;
                }
            }
        }

        counter
    }

    pub fn get_value(&self, row: usize, col: usize) -> String {
        let v = self.map1d[self.get_index(row, col)];
        match v {
            Grid::Mine => "💣".to_string(),
            Grid::Safe0 => String::from("0️⃣"),
            Grid::Safe1 => String::from("1️⃣"),
            Grid::Safe2 => String::from("2️⃣"),
            Grid::Safe3 => String::from("3️⃣"),
            Grid::Safe4 => String::from("4️⃣"),
            Grid::Safe5 => String::from("5️⃣"),
            Grid::Safe6 => String::from("6️⃣"),
            Grid::Safe7 => String::from("7️⃣"),
            Grid::Safe8 => String::from("8️⃣"),
        }
    }
    pub fn get_answer(&self) -> Vec<String> {
        let mut res = vec![];

        for k in 0..self.width * self.height {
            let (i, j) = self.get_i_j(k);

            res.push(self.get_value(i, j));
        }

        res
    }
    pub fn get_status(&self, row: usize, col: usize) -> String {
        let v = self.status1d[self.get_index(row, col)];
        match v {
            Status::Open => String::from("开"),
            Status::Close => String::from("关"),
            _ => String::from("mm"),
        }
    }

    fn get_i_j(&self, index: usize) -> (usize, usize) {
        let i = index / self.width;
        let j = index % self.width;

        (i, j)
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    fn print_by_line(&self) {
        for line in self.map1d.as_slice().chunks(self.width) {
            for &grid in line {
                let mut char = "";
                if grid == Grid::Mine {
                    char = "💣";
                } else {
                    char = "🚩️";
                }
                print!("{}", char);
            }
            println!();
        }
    }

    // 点击.  can be continued
    pub fn foot_click(&mut self, i: usize, j: usize) -> Vec<usize> {
        let index = self.get_index(i, j);

        // 1. it is a mine -> game over
        if self.map1d[index] == Grid::Mine {
            self.status1d[index] = Status::Open;
            // self.status1d[index] = Status::Boom;

            return vec![index];
        } else if self.map1d[index] == Grid::Safe0 {
            // 开拓   回溯

            self.status1d[index] = Status::Open;
            let index_list = self.get_all_safe_around(i, j);

            for &k in &index_list {
                self.status1d[k] = Status::Open;
            }

            return index_list;
        } else {
            self.status1d[index] = Status::Open;

            return vec![index];
        }

        // 2. it is safe, but there are mines near map[i][j] -> show mine counter

        // 3. it is safe here, and it is safe around here. -> open up map util meet mine counter
    }

    // 点击 标记有地雷  或者 取消标记
    pub fn flag_click(&mut self, i: usize, j: usize) {
        let index = self.get_index(i, j);

        if self.status1d[index] == Status::Close {
            self.status1d[index] = Status::Flag;
        } else if self.status1d[index] == Status::Flag {
            self.status1d[index] = Status::Close;
        }
    }

    // Numbers - wasm-bindgen指南 https://rustwasm.wasmdev.cn/docs/wasm-bindgen/reference/types/numbers.html
    pub fn double_click(&mut self, row: usize, col: usize) -> Option<String> {
        let mut res: Option<String> = None;

        let index = self.get_index(row, col);
        let target_counter = self.map1d[index] as usize;

        // count flags
        let mut flags = 0;

        let mut round8_index_list = vec![];

        for delta_i in [-1, 0, 1].iter().cloned() {
            for delta_j in [-1, 0, 1].iter().cloned() {
                // 排除自己
                if delta_i == 0 && delta_j == 0 {
                    continue;
                }
                // 排除边角
                if row == 0 && delta_i == -1 {
                    continue;
                }
                if col == 0 && delta_j == -1 {
                    continue;
                }
                if row == self.height - 1 && delta_i == 1 {
                    continue;
                }
                if col == self.width - 1 && delta_j == 1 {
                    continue;
                }

                let index = (row as i32 + delta_i) * self.width as i32 + (col as i32 + delta_j);
                let index = index as usize;

                if self.status1d[index] == Status::Flag {
                    flags += 1;
                } else {
                    round8_index_list.push(index);
                }
            }
        }

        if target_counter > flags {
            res = Some(String::from("标记不足"))
        } else if target_counter < flags {
            res = Some(String::from("标记过多"))
        } else {
            // 把周围一圈全都打开.遇到0自动打开.

            for &index in &round8_index_list {
                self.status1d[index] = Status::Open;

                if self.map1d[index] == Grid::Safe0 {
                    self.status1d[index] = Status::Open;
                    let index_list =
                        self.get_all_safe_around(index / self.width, index % self.width);

                    for &k in &index_list {
                        self.status1d[k] = Status::Open;
                    }
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::GameMap;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        let mut map = GameMap::new(10, 10, 55, true);
        map.print_by_line();
    }
}
