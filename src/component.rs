use std::{collections::HashMap, fmt::Display, time::Duration};

use ggez::audio::{self, SoundSource};

use crate::systems::events::Event;

/// 位置组件，表示实体在网格中的坐标
#[derive(Clone,Copy,PartialEq,Eq,Hash)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

/// 渲染组件，包含渲染所需的资源路径
pub struct Renderable {
    pub paths: Vec<String>,
}

/// 渲染类型枚举：静态图片或动画
pub enum RenderableKind {
    Static,
    Animated,
}

/// 墙组件（标记组件）
pub struct Wall {}

/// 玩家组件（标记组件）
pub struct Player {}

/// 箱子组件，包含颜色信息
pub struct Box {
    pub color: BoxColor
}

/// 目标点组件，包含颜色信息
pub struct BoxSpot {
    pub color: BoxColor
}

/// 可移动组件（标记组件）
pub struct Moveable {}

/// 不可移动组件（标记组件）
pub struct Immovable {}

/// 游戏状态枚举
#[derive(Default, PartialEq, Eq)]
pub enum GameplayState {
    #[default]
    Playing, // 游戏中
    Won,     // 胜利
    Lost,    // 失败
}

/// 游戏核心状态组件，记录当前状态和步数
#[derive(Default)]
pub struct GamePlay {
    pub state: GameplayState,
    pub move_count: u32
}

impl Display for GameplayState {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
            GameplayState::Lost => "Lost",
        })?;
        Ok(())
    }
}

/// 箱子颜色枚举
#[derive(PartialEq)]
pub enum BoxColor {
    Red,
    Blue
}

impl Display for BoxColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BoxColor::Red => "red",
            BoxColor::Blue => "blue",
        })?;
        Ok(())
    }
}

impl Renderable {
    /// 创建静态渲染组件（单张图片）
    pub fn new_static(path: &str) -> Self{
        Self{
            paths: vec![path.to_string()],
        }
    }

    /// 创建动画渲染组件（多张图片）
    pub fn new_animated(paths: Vec<String>) -> Self{
        Self{
            paths,
        }
    }

    /// 获取渲染类型
    pub fn kind(&self) -> RenderableKind{
        match self.paths.len() {
            0 => panic!("Invalid renderable kind"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    /// 获取指定索引的资源路径（用于动画循环）
    pub fn path(&self,path_index: usize) -> String {
        self.paths[path_index % self.paths.len()].clone()
    }
}

/// 时间组件，记录帧间隔
#[derive(Default)]
pub struct Time {
    pub delta: Duration
}

/// 事件队列组件，存储待处理的游戏事件
#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

/// 音频存储组件，管理游戏音效
#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, std::boxed::Box<audio::Source>>
}

impl AudioStore {
    /// 播放指定名称的音效
    pub fn play(&mut self,ctx: &mut ggez::Context, sound: &str){
        if let Some(source) = self.sounds.get_mut(sound) {
            if source.play_detached(ctx).is_ok() {
                println!("play sound: {}", sound);
            }
        }
    }
}
