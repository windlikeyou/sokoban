use std::collections::HashMap;

use ggez::input::keyboard::KeyCode;
use hecs::{Entity, World};

use crate::{component::{EventQueue, GamePlay, GameplayState, Immovable, Moveable, Player, Position}, constants::{MAP_HEIGHT, MAP_WIDTH}, systems::events::{EntityMoved, Event}};

/// 重复的输入系统示例（未使用）
/// 仅用于演示直接修改位置的简单方式
pub fn input_system_duplicate(world: &World, ctx: &mut ggez::Context) {
    for (_, (position, _player)) in world.query::<(&mut Position, &Player)>().iter() {
        if ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            position.y -= 1;
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            position.y += 1;
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Left) {
            position.x -= 1;
        }
        if ctx.keyboard.is_key_just_pressed(KeyCode::Right) {
            position.x += 1;
        }
    }
}

/// 输入打印系统（未使用）
/// 仅用于调试按键状态
pub fn run_input_print(world: &World, ctx: &mut ggez::Context) {
    if ctx.keyboard.is_key_pressed(KeyCode::Up) {
        println!("Up");
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Down) {
        println!("Down");
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Left) {
        println!("Left");
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Right) {
        println!("Right");
    }
}

/// 核心输入处理系统
/// 处理玩家的键盘输入，计算移动逻辑，并处理推箱子碰撞
pub fn run_input(world: &World, ctx: &mut ggez::Context) {
    let mut to_move: Vec<(Entity, KeyCode)> = Vec::new();
    let mut events = Vec::new();

    // 检查游戏状态，如果不是 Playing，则不处理移动输入
    {
        let mut query = world.query::<&GamePlay>();
        if let Some(gameplay) = query.iter().next().map(|(_, g)| g) {
            if gameplay.state != GameplayState::Playing {
                return;
            }
        }
    }

    // 收集所有可移动实体（箱子）和不可移动实体（墙）的位置信息
    let mov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Moveable)>()
        .iter()
        .map(|t| ((t.1.0.x, t.1.0.y), t.0))
        .collect::<HashMap<_, _>>();
    let immov: HashMap<(u8, u8), Entity> = world
        .query::<(&Position, &Immovable)>()
        .iter()
        .map(|t| ((t.1.0.x, t.1.0.y), t.0))
        .collect::<HashMap<_, _>>();
        
    // 遍历所有玩家实体（通常只有一个）
    for (_, (position, _player)) in world.query::<(&mut Position, &Player)>().iter() {
        // 获取按下的方向键
        let key = if ctx.keyboard.is_key_just_pressed(KeyCode::Up) {
            KeyCode::Up
        } else if ctx.keyboard.is_key_just_pressed(KeyCode::Down) {
            KeyCode::Down
        } else if ctx.keyboard.is_key_just_pressed(KeyCode::Left) {
            KeyCode::Left
        } else if ctx.keyboard.is_key_just_pressed(KeyCode::Right) {
            KeyCode::Right
        } else {
            continue; // 没有按键按下
        };

        // 计算移动方向和边界
        let (start, end, is_x) = match key {
            KeyCode::Up => (position.y, 0, false),
            KeyCode::Down => (position.y, MAP_HEIGHT - 1, false),
            KeyCode::Left => (position.x, 0, true),
            KeyCode::Right => (position.x, MAP_WIDTH - 1, true),
            _ => continue,
        };

        // 生成检测路径上的坐标序列
        let range = if start < end {
            (start..=end).collect::<Vec<_>>()
        } else {
            (end..=start).rev().collect::<Vec<_>>()
        };

        // 沿移动方向检测碰撞
        for x_or_y in range {
            let pos = if is_x {
                (x_or_y, position.y)
            } else {
                (position.x, x_or_y)
            };

            // 检查是否有可移动实体（箱子）
            match mov.get(&pos) {
                Some(entity) => to_move.push((*entity, key)),
                None => {
                    // 没有可移动的实体，检查是否有不可移动的实体（墙）
                    match immov.get(&pos) {
                        Some(_id) => {
                            // 撞墙了，清空移动列表（推不动），触发撞墙音效事件
                            to_move.clear();
                            events.push(Event::PlayerHitObstacle {});
                            break;
                        },
                        None => break, // 空地，可以移动
                    }
                }
            }
        }
    }

    // 如果有实体需要移动，增加步数计数
     if !to_move.is_empty() {
        let mut query = world.query::<&mut GamePlay>();
        let gameplay = query.iter().next().unwrap().1;
        gameplay.move_count += 1;
    }

    // 执行实际的移动操作
    for (entity, key) in to_move {
        let mut position = world.get::<&mut Position>(entity).unwrap();

        match key {
            KeyCode::Up => position.y -= 1,
            KeyCode::Down => position.y += 1,
            KeyCode::Left => position.x -= 1,
            KeyCode::Right => position.x += 1,
            _ => (),
        }

        // 触发实体移动事件（用于播放音效等）
        events.push(Event::EntityMoved(EntityMoved { entity }));
    }

    // 将生成的事件添加到全局事件队列
    {
        let mut query = world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.append(&mut events);
    }
}
