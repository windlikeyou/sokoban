use std::collections::HashMap;

use hecs::{Entity, World};

use crate::component::{AudioStore, Box, BoxSpot, EventQueue, Position};

/// 实体移动事件数据
#[derive(Debug)]
pub struct EntityMoved {
    pub entity: Entity
}

/// 箱子放置事件数据（暂时未使用）
#[allow(dead_code)]
#[derive(Debug)]
pub struct BoxPlacedOnSpot {
    pub is_correct_spot: bool
}

/// 游戏事件枚举
#[derive(Debug)]
pub enum Event {
    // 玩家撞墙事件
    PlayerHitObstacle,
    // 实体移动事件
    EntityMoved(EntityMoved),
    // 箱子放置在目标点事件
    BoxPlacedOnSpot(BoxPlacedOnSpot)
}

/// 事件处理系统
/// 处理队列中的事件，触发音效或生成新事件
pub fn run_process_events(world: &mut World,ctx: &mut ggez::Context){
    // 1. 取出并清空当前所有事件
    let events = {
        let mut query = world.query::<&mut EventQueue>();
        let events = query.iter().next().unwrap().1.events.drain(..).collect::<Vec<_>>();

        events
    };

    let mut new_events = Vec::new();
    // 获取所有目标点位置，用于检查箱子是否归位
    let mut query = world.query::<(&Position,&BoxSpot)>();
    let box_spots_by_position:HashMap<(u8,u8), &BoxSpot> = query.iter().map(|(_,t)|((t.0.x,t.0.y),t.1)).collect::<HashMap<_,_>>();
    
    // 获取音频存储组件
    let mut audio_query = world.query::<&mut AudioStore>();
    let audio_store = audio_query.iter().next().unwrap().1;

    // 2. 遍历处理每个事件
    for event in events {
        println!("New Event: {:?}", event);
        match event {
            Event::PlayerHitObstacle => {
                // 撞墙，播放音效
                audio_store.play(ctx, "wall");
            },
            Event::EntityMoved(EntityMoved {entity}) => {
                // 实体移动后，检查是否是箱子移动到了目标点
                if let Ok(the_box)= world.get::<&Box>(entity) {
                    if let Ok(the_position) = world.get::<&Position>(entity) {
                        if let Some(box_spot) = box_spots_by_position.get(&(the_position.x,the_position.y)) {
                            // 生成箱子归位事件
                            new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                is_correct_spot: box_spot.color == the_box.color
                            }));
                        }
                    }
                }
            },
            Event::BoxPlacedOnSpot(BoxPlacedOnSpot {is_correct_spot}) => {
                // 箱子归位，播放对应音效
                let sound_name = if is_correct_spot {
                    "correct"
                } else {
                    "incorrect"
                };
                audio_store.play(ctx, sound_name);
            }
        }
    }

    // 3. 将新生成的事件加入队列（等待下一帧处理）
    {
        let mut query = world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.append(&mut new_events);
    }
}
