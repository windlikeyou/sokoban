use std::path;

use ggez::{GameResult, conf, event};
use hecs::{ World};

use crate::{map::initialize_level, tool::Game};

mod entity;
mod component;
mod constants;
mod map;
mod tool;
mod systems;

/// 游戏入口函数
/// 初始化游戏上下文、窗口设置和事件循环
fn main() -> GameResult {
    // 创建游戏上下文构建器
    let context_handler = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        // 设置窗口标题
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban"))
        // 设置窗口尺寸
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        // 添加资源路径（图片、音频等）
        .add_resource_path(path::PathBuf::from("./resources"));

    // 构建上下文和事件循环
    let (mut context, event_loop) = context_handler.build()?;
    // 创建游戏实例
    let game = Game::new(&mut context)?;
    // 运行游戏主循环
    event::run(context, event_loop, game)
}
