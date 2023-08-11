use piston_window::{
    clear, rectangle, types::Color, Button, Context, G2d, Glyphs, PistonWindow, PressEvent,
    UpdateEvent, WindowSettings,
};

use crate::tetris_game::game::Game;

/// 背景颜色
const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [191.0, 179.0, 179.0, 1.0];

fn to_coord(game_coord: u32) -> f64 {
    game_coord as f64
}

// 绘制边框
fn draw_border(
    screen_left: u32,
    width: u32,
    block_size: u32,
    height: u32,
    con: &Context,
    g: &mut G2d,
) {
    // draw game box
    // top
    rectangle(
        BORDER_COLOR,
        [
            to_coord(screen_left),
            to_coord(0),
            to_coord(width * block_size),
            to_coord(2),
        ],
        con.transform,
        g,
    );
    // bottom
    rectangle(
        BORDER_COLOR,
        [
            to_coord(screen_left),
            to_coord(height * block_size - 2),
            to_coord(width * block_size),
            to_coord(2),
        ],
        con.transform,
        g,
    );
    // left
    rectangle(
        BORDER_COLOR,
        [
            to_coord(screen_left),
            to_coord(0),
            to_coord(2),
            to_coord(height * block_size),
        ],
        con.transform,
        g,
    );
    rectangle(
        BORDER_COLOR,
        [
            to_coord(screen_left + width * block_size),
            to_coord(0),
            to_coord(2),
            to_coord(height * block_size),
        ],
        con.transform,
        g,
    );
}

pub struct TetrisWindow {
    width: u32,
    height: u32,
    screen_left: u32,
    screen_right: u32,
    block_size: u32,
    windows: PistonWindow,
    game: Game,
    glyphs: Glyphs,
}

impl TetrisWindow {
    pub fn new(width: u32, height: u32) -> TetrisWindow {
        let screen_left: u32 = 200;
        let screen_right: u32 = 200;
        let block_size: u32 = 20;
        let mut windows: PistonWindow = WindowSettings::new(
            "tetris",
            [
                (width * block_size + screen_left + screen_right),
                height * block_size,
            ],
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

        let glyphs = windows
            .load_font("/home/wangcheng/ext/tetris/src/AlibabaPuHuiTi-2-45-Light.ttf")
            .unwrap();

        TetrisWindow {
            width,
            height,
            block_size, // 单个方块宽高为20
            windows,
            screen_left,
            screen_right,
            game: Game::new(),
            glyphs,
        }
    }

    pub fn loop_window(mut self) {
        while let Some(event) = self.windows.next() {
            if let Some(Button::Keyboard(key)) = event.press_args() {
                self.game.key_pressed(key);
            }

            self.windows.draw_2d(&event, |c, g, device| {
                clear(BACK_COLOR, g);
                draw_border(
                    self.screen_left,
                    self.width,
                    self.block_size,
                    self.height,
                    &c,
                    g,
                );
                self.game.draw_game_data(
                    &c,
                    g,
                    self.block_size,
                    self.screen_left,
                    &mut self.glyphs,
                    device,
                );
            });

            event.update(|arg| {
                // update game..
                self.game.update(arg.dt);
            });
        }
    }
}
