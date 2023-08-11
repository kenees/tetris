#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug)]
pub struct Tetris {}

/// 块的类型有7中，
///     ----(I)，
///     _|_,(T)
///     方块, (O)
///     |___, ___|, (L, J)
///     ~|_, _|~, (Z, G)
/// 将所有方块的第一个坐标设置为旋转中心

pub fn create_i() -> [[i32; 2]; 4] {
    let a = [[-3, 5], [-3, 4], [-3, 6], [-3, 7]]; // [-3, 5] 旋转点
    return a;
}

fn create_z() -> [[i32; 2]; 4] {
    let a = [[-3, 5], [-4, 4], [-4, 5], [-3, 6]]; // [-3, 5] 旋转点
    return a;
}

fn create_z1() -> [[i32; 2]; 4] {
    let a = [[-3, 5], [-3, 4], [-4, 5], [-4, 6]]; // [-3, 5] 旋转点
    return a;
}

fn create_l() -> [[i32; 2]; 4] {
    let a = [[-3, 5], [-3, 4], [-4, 5], [-5, 5]]; // [-3, 5] 旋转点
    return a;
}

fn create_l1() -> [[i32; 2]; 4] {
    let a = [[-3, 5], [-4, 5], [-5, 5], [-3, 6]]; // [-3, 5] 旋转点
    return a;
}

fn create_t() -> [[i32; 2]; 4] {
    let a = [[-3, 5], [-3, 4], [-4, 5], [-3, 6]]; // [-3, 5] 旋转点
    return a;
}

fn create_g() -> [[i32; 2]; 4] {
    let a = [[-3, 5], [-3, 6], [-2, 5], [-2, 6]]; // [-3, 5] 旋转点
    return a;
}

impl Tetris {
    pub fn new() -> Tetris {
        Tetris {}
    }

    // 创建一个块
    pub fn create(&self, num: i32) -> [[i32; 2]; 4] {
        let tetris = match Some(num) {
            Some(1) => create_i(),
            Some(2) => create_z(),
            Some(3) => create_z1(),
            Some(4) => create_l(),
            Some(5) => create_l1(),
            Some(6) => create_t(),
            Some(7) => create_g(),
            _ => create_i(),
        };
        return tetris;
    }

    // 根据方向获取下一步的数据
    pub fn move_forward(&self, ele: [[i32; 2]; 4], dir: Option<Direction>) -> [[i32; 2]; 4] {
        // println!("{:?}, {:?}", ele, dir);
        match dir {
            Some(Direction::Up) => ele,
            Some(Direction::Down) => {
                let mut d = ele.clone();
                for i in 0..d.len() {
                    d[i][0] = d[i][0] + 1;
                }
                return d;
            }
            Some(Direction::Left) => {
                let mut d = ele.clone();
                for i in 0..d.len() {
                    if d[i][1] <= 0 {
                        // 到左边界了，返回原坐标
                        return ele.clone();
                    }
                    d[i][1] = d[i][1] - 1;
                }
                return d;
            }
            Some(Direction::Right) => {
                let mut d = ele.clone();
                for i in 0..d.len() {
                    if d[i][1] >= 9 {
                        // 到右边界了，返回原坐标
                        return ele.clone();
                    }
                    d[i][1] = d[i][1] + 1;
                }
                return d;
            }
            _ => {
                let mut d = ele.clone();
                for i in 0..d.len() {
                    d[i][0] = d[i][0] + 1;
                }
                return d;
            }
        }
    }

    // 旋转，并获取旋转后的数据
    pub fn rotate(&self, ele: [[i32; 2]; 4]) -> [[i32; 2]; 4] {
        let center = ele[0];

        // 如果旋转后超出范围需要规范到范围内(0 ~ 9)（左移或右移)
        let mut l_offset = 0;
        let mut r_offset = 0;
        let mut d = ele.clone();
        for i in 1..d.len() {
            let dx = d[i][0] - center[0];
            let dy = d[i][1] - center[1];
            d[i][0] = dy + center[0];
            d[i][1] = -dx + center[1];

            if d[i][1] < l_offset {
                l_offset = -d[i][1];
            }
            if 9 - d[i][1] < r_offset {
                r_offset = 9 - d[i][1]
            }
        }

        let offset: i32 = if l_offset != 0 {
            l_offset
        } else {
            if r_offset != 0 {
                r_offset
            } else {
                0
            }
        };

        if offset != 0 {
            for i in 0..d.len() {
                d[i][1] = d[i][1] + offset;
            }
        }
        return d;
    }
}
