use std::collections::HashMap;

use hecs::World;

use crate::component::{Box, BoxSpot, GamePlay, GameplayState, Immovable, Position, Wall};

/// 核心游戏逻辑系统
/// 检查胜利条件和失败条件（死锁）
pub fn run_gameplay_state(world: &World) {
    let mut query = world.query::<&mut GamePlay>();
    let gameplay = query.iter().next().unwrap().1;

    // 如果已经赢了或输了，就不再重复检查
    if gameplay.state == GameplayState::Won || gameplay.state == GameplayState::Lost {
        return;
    }

    // 1. 检查胜利条件
    // 获取所有箱子的位置
    let mut query = world.query::<(&Position, &Box)>();
    let boxes_by_position: HashMap<(u8, u8), &Box> = query
        .iter()
        .map(|(_, t)| ((t.0.x, t.0.y), t.1))
        .collect::<HashMap<_, _>>();

    // 统计不在正确目标点上的箱子数量
    let boxes_out_of_position: usize = world
        .query::<(&Position, &BoxSpot)>()
        .iter()
        .map(|(_, (position, box_spot))| {
            // 检查该目标点位置上是否有箱子
            if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                // 检查箱子颜色是否匹配
                if box_spot.color == the_box.color {
                    0 // 匹配，计数 0
                } else {
                    1 // 颜色不匹配，计数 1
                }
            } else {
                1 // 没有箱子，计数 1
            }
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    // 如果所有目标点都正确匹配了箱子，游戏胜利
    if boxes_out_of_position == 0 {
        gameplay.state = GameplayState::Won;
        return;
    }

    // 2. 检查失败条件（简单的死角检测）
    // 如果任意一个箱子进入了非目标的角落，游戏失败
    // 角落定义：(上是墙 && 左是墙) || (上 && 右) || (下 && 左) || (下 && 右)
    
    // 获取所有墙的位置
    let walls: HashMap<(u8, u8), bool> = world
        .query::<(&Position, &Wall)>()
        .iter()
        .map(|(_, (pos, _))| ((pos.x, pos.y), true))
        .collect();

    // 获取所有目标点位置
    let box_spots: HashMap<(u8, u8), bool> = world
        .query::<(&Position, &BoxSpot)>()
        .iter()
        .map(|(_, (pos, _))| ((pos.x, pos.y), true))
        .collect();

    // 检查每个箱子
    for (_, (pos, _)) in world.query::<(&Position, &Box)>().iter() {
        // 如果箱子已经在目标点上，暂时不算死（即使在角落也是胜利的一部分，或者中间状态）
        // 这里简化：只要在任意 BoxSpot 上，就不判死。
        if box_spots.contains_key(&(pos.x, pos.y)) {
            continue;
        }

        let is_wall = |x, y| walls.contains_key(&(x, y));

        // 检查四周是否有墙
        let up = is_wall(pos.x, pos.y - 1);
        let down = is_wall(pos.x, pos.y + 1);
        let left = is_wall(pos.x - 1, pos.y);
        let right = is_wall(pos.x + 1, pos.y);

        // 如果形成了死角，判定游戏失败
        if (up && left) || (up && right) || (down && left) || (down && right) {
            gameplay.state = GameplayState::Lost;
            return;
        }
    }
}
