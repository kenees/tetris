use crate::tetris_tetris::tetris::{create_i, Direction, Tetris};
use piston_window::{
    rectangle, text, types::Color, Context, G2d, GfxDevice, Glyphs, Key, Transformed,
};
use rand::distributions::{Distribution, Uniform};
/// 移动周期，每过多长时间移动一次方块
const MOVING_PERIOD: f64 = 0.8; //0.8;
/// 方块颜色
const BLOCK_COLOR: Color = [255.0, 255.0, 255.0, 1.0];
const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn to_coord(game_coord: u32) -> f64 {
    game_coord as f64
}

#[derive(Debug)]
pub struct Game {
    // 分数
    score: f64,
    // 行数
    lines: f64,
    // 方块
    tetris: Tetris,
    //
    // pause
    game_pause: bool,
    game_over: bool,
    waiting_time: f64,

    // 下落中
    falling: bool,
    falling_tetris: [[i32; 2]; 4],
    rotate: bool,

    // 窗口所有格子数据
    // vec![[i32; 10]; 20]
    all_tetris: Vec<[i32; 10]>,
    // 渲染需要结合all_tetris 和 falling_tetris
}

impl Game {
    pub fn new() -> Self {
        Game {
            score: 0.0,
            lines: 0.0,
            tetris: Tetris::new(),

            game_pause: false,
            game_over: false,
            waiting_time: 0.0,

            falling: true,
            rotate: false,
            falling_tetris: create_i(),
            all_tetris: vec![
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // 暂停， 结束
        if self.game_pause || self.game_over {
            return;
        }

        self.waiting_time += delta_time;

        if self.rotate {
            // 需要旋转
            self.rotate = false;
            let d = self.tetris.rotate(self.falling_tetris);
            if self.check_if_tetris_alive(d) {
                self.falling_tetris = d;
            }
        }

        if self.waiting_time > MOVING_PERIOD {
            // needs update
            println!("{}", "update");
            self.waiting_time = 0.0;
            self.update_tetris(None);
        }
    }

    fn update_tetris(&mut self, dir: Option<Direction>) {
        if self.falling {
            // move to next
            let _d = self.tetris.move_forward(self.falling_tetris, dir);
            if self.check_if_tetris_alive(_d) {
                // 可以移动
                println!("可以移动: {:?}", _d);
                self.falling_tetris = _d;
            } else {
                // 不可以移动了， 结束了
                // 将数据合并到 all_tetris
                self.falling = false;
                let d = self.merge_tetris();
                self.all_tetris = d;

                println!("不可以移动");
            }
        } else {
            println!("create new ..");
            // 未存在下落中的方块
            // 计分检查
            self.check_score();
            // check game over
            if self.is_game_over() {
                self.game_over = true;
            };
            // 创建一个新的
            let mut rng = rand::thread_rng();
            let die = Uniform::from(0..8);
            let throw = die.sample(&mut rng);
            self.falling_tetris = self.tetris.create(throw);
            self.falling = true;
            // println!("{:?}", md);
        }
    }

    fn check_if_tetris_alive(&self, _d: [[i32; 2]; 4]) -> bool {
        // 1. 调用 move_forward 获取移动后的数据，
        // 2. 根据移动后的数据判断是否可以移动，返回bool
        for i in 0.._d.len() {
            let x = _d[i][0];
            let y = _d[i][1];

            // 数据在范围内且该位置已经有方块了
            if x >= 0 && x < 20 && y >= 0 && y < 10 && self.all_tetris[x as usize][y as usize] == 1
            {
                println!(
                    "all_tetris[{}, {}]: {}",
                    x, y, self.all_tetris[x as usize][y as usize]
                );
                return false;
            }
            // 超出左右的边界 (在move_forward中直接处理了)
            if x >= 20 {
                println!("触底了。。");
                return false;
            }
        }
        return true;
    }

    pub fn merge_tetris(&self) -> Vec<[i32; 10]> {
        let mut data = self.all_tetris.clone();

        for m in self.falling_tetris {
            if m[0] < 0 || m[1] < 0 {
                continue;
            }
            data[m[0] as usize][m[1] as usize] = 1;
        }

        return data;
    }

    fn check_score(&mut self) {
        for i in 0..20 {
            let mut isOk = true;
            let data = self.all_tetris[i];
            for j in data {
                if j == 0 {
                    isOk = false;
                }
            }
            if isOk {
                println!("第{}行得分", i);
                self.all_tetris.remove(i);
                self.all_tetris.insert(0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
                self.score += 1.0;
            }
        }
        //Test
        // println!("第19行得分");
        // self.all_tetris.remove(19);
        // self.all_tetris.insert(0, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        // self.score += 1.0;
    }

    fn is_game_over(&self) -> bool {
        for i in self.falling_tetris {
            if i[0] < 0 {
                return true;
            }
        }
        return false;
    }

    pub fn draw_game_data(
        &mut self,
        c: &Context,
        g: &mut G2d,
        block_size: u32,
        screen_left: u32,
        glyphs: &mut Glyphs,
        device: &mut GfxDevice,
    ) {
        // 1, 合并数据
        let d = self.merge_tetris();

        // 2. 绘制所有方块
        for y in 0..20 {
            for x in 0..10 {
                if d[y][x] == 1 {
                    rectangle(
                        BLOCK_COLOR,
                        [
                            to_coord((x as u32) * block_size + screen_left),
                            to_coord((y as u32) * block_size),
                            to_coord(block_size),
                            to_coord(block_size),
                        ],
                        c.transform,
                        g,
                    );
                }
            }
        }

        // 3. 绘制分数
        let transform = &c.transform.trans((screen_left / 3).into(), 20.0);
        text::Text::new_color([0.0, 1.0, 0.0, 1.0], 12)
            .draw(
                &format!("分数：{}", self.score.to_string()),
                glyphs,
                &c.draw_state,
                *transform,
                g,
            )
            .unwrap();
        glyphs.factory.encoder.flush(device);

        // 4. 绘制下一个预测

        // 绘制game over
        if self.game_over {
            let transform2 = &c.transform.trans((screen_left).into(), 260.0);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                .draw("GAME OVER!", glyphs, &c.draw_state, *transform2, g)
                .unwrap();
            glyphs.factory.encoder.flush(device);
        }
    }

    // 字母需要通过ctrl + 才可激活
    pub fn key_pressed(&mut self, key: Key) {
        println!("key: {:?}", key);
        match key {
            Key::Left => {
                self.update_tetris(Some(Direction::Left));
            }
            Key::Right => {
                self.update_tetris(Some(Direction::Right));
            }
            Key::Down => {
                // self.update_tetris(Some(Direction::Down));
            }
            Key::Up => self.update_tetris(Some(Direction::Up)),
            Key::Space => {
                println!("key down space");
                self.game_pause = !self.game_pause;
            }
            Key::Z => {
                println!("key down space");
                self.rotate = true;
            }
            _ => {}
        };
    }
}
