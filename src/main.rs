mod tetris_game;
mod tetris_tetris;
mod tetris_window;

use tetris_window::window::TetrisWindow;

fn main() {
    println!("Hello, world!");

    // 定义窗口大小
    let (width, height) = (10.0, 20.0);

    let windows = TetrisWindow::new(width as u32, height as u32);

    windows.loop_window();
}
