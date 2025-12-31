use hecs::World;

use crate::{component::{BoxColor, Position}, entity::{self, create_box, create_box_spot, create_floor, create_player, create_wall, load_sounds}};

/// 关卡地图数据数组
/// 每个字符串代表一个关卡的布局
/// N: 空白, W: 墙, P: 玩家, .: 地板
/// BB: 蓝箱子, RB: 红箱子, BS: 蓝目标点, RS: 红目标点
pub const LEVELS: [&str; 30] = [
    // Level 1: 入门 - 单箱推动
    "
    N N W W W W W N
    W W W . . . W N
    W . P . BB . W N
    W . . . . BS W N
    W . . . . . W N
    W W W W W W W N
    ",
    // Level 2: 入门 - 双箱顺序
    "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . BB . . . W 
    W . P . . . . W
    W . . . . BS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ",
    // Level 3: 基础 - 转角技巧
    "
    N N W W W W W N
    W W W . . . W N
    W . P . BB . W N
    W . . . . . W N
    W W . BB . W W
    W . . . . BS W
    W . . . BS . W
    W W W W W W W
    ",
    // Level 4: 基础 - 走廊
    "
    W W W W W W W W
    W . . . . . . W
    W . BB . BB . W
    W . . P . . . W
    W . BS . BS . W
    W . . . . . . W
    W W W W W W W W
    ",
    // Level 5: 初级 - T字型
    "
    N W W W W W N
    W W . . . W W
    W . . BB . . W
    W . . BB . . W
    W W . BS . W W
    N W . BS . W N
    N W . P . W N
    N W W W W W N
    ",
    // Level 6: 初级 - 经典的 Microban 1
    "
    W W W W W N N
    W . . . W N N
    W . BS . W N N
    W W BB . W W W
    W . . BB . . W
    W . . P W . W
    W W BS . . . W
    N W W W W W W
    ",
    // Level 7: 初级 - 狭窄空间
    "
    N N W W W W N
    W W W . . W N
    W BS P BB . W N
    W W W . BB W N
    N W BS . . W N
    N W . . W W N
    N W W W W N N
    ",
    // Level 8: 初级 - 需要规划
    "
    W W W W W W W W
    W . . . . . . W
    W . P BB BB . W
    W . . . . . . W
    W . BS BS . . W
    W . . . . . . W
    W W W W W W W W
    ",
    // Level 9: 中级 - 障碍物
    "
    N W W W W W N
    W W . . . W W
    W . . BB . . W
    W . W BB W . W
    W . . P . . W
    W . BS BS . W
    W W . . . W W
    N W W W W W N
    ",
    // Level 10: 中级 - 经典的 U 型
    "
    N W W W W W N
    N W . . . W N
    N W . BB . W N
    W W . . . W W
    W . . P . . W
    W . BB . BB . W
    W BS . . . BS W
    W W W W W W W
    ",
    // Level 11
    "
    W W W W W W N
    W BS . . . W N
    W . BB P . W N
    W W W . . W W
    N W . BB . . W
    N W . . . BS W
    N W W W W W W
    ",
    // Level 12
    "
    N W W W W W N
    W W . . . W N
    W . . BB . W W
    W . W . W . W
    W . P . . . W
    W W . BB . W W
    N W BS BS W W N
    N W W W W W N N
    ",
    // Level 13
    "
    W W W W W W W
    W BS . . . BS W
    W . BB . BB . W
    W . . P . . W
    W . . . . . W
    W W W W W W W
    ",
    // Level 14
    "
    N N W W W N N
    N N W BS W N N
    W W W . W W W
    W . . BB . . W
    W . P . . . W
    W W W . W W W
    N N W BS W N N
    N N W W W N N
    ",
    // Level 15
    "
    W W W W W W W
    W . . . . . W
    W . BS BB BS . W
    W . . P . . W
    W . BS BB BS . W
    W . . . . . W
    W W W W W W W
    ",
    // Level 16: 进阶 - 拥挤
    "
    N W W W W N N
    W W . . W W N
    W . . BB . W N
    W . BB BS . W N
    W W P BS . W N
    N W . . W W N
    N W W W W N N
    ",
    // Level 17
    "
    W W W W W W N
    W . . . . W N
    W . BB BB . W N
    W . P . . W W
    W W . BS BS . W
    N W . . . . W
    N W W W W W W
    ",
    // Level 18
    "
    N N W W W W N
    W W W . . W N
    W . . . BB W N
    W . P BB . W N
    W W W BS BS W N
    N N W . . W N
    N N W W W W N
    ",
    // Level 19
    "
    N W W W W W N
    N W . . . W N
    W W . BB . W W
    W . . BS . . W
    W . BS P BS . W
    W . . BB . . W
    W W . . . W W
    N W W W W W N
    ",
    // Level 20
    "
    W W W W W W W
    W . BS BS BS . W
    W . . . . . W
    W . BB BB BB . W
    W . . P . . W
    W . . . . . W
    W W W W W W W
    ",
    // Level 21: 高级 - 复杂地形
    "
    N N W W W N N
    N W W . W W N
    W W . . . W W
    W . . BB . . W
    W . BB P BB . W
    W . . BB . . W
    W W . . . W W
    N W BS BS BS W N
    N W W W W W N
    ",
    // Level 22
    "
    W W W W W W W
    W BS . . . BS W
    W . . W . . W
    W . BB P BB . W
    W . . W . . W
    W BS . . . BS W
    W W W W W W W
    ",
    // Level 23
    "
    N W W W W W N
    W W . . . W W
    W . . BB . . W
    W . BB . BB . W
    W . . P . . W
    W . BS BS BS . W
    W W . . . W W
    N W W W W W N
    ",
    // Level 24
    "
    N N W W W N N
    N W W . W W N
    W W . . . W W
    W . BB . BB . W
    W . BS P BS . W
    W . BB . BB . W
    W W . . . W W
    N W W W W W N
    ",
    // Level 25
    "
    W W W W W W W
    W . . . . . W
    W . BS . BS . W
    W . . BB . . W
    W . BB P BB . W
    W . . . . . W
    W W W W W W W
    ",
    // Level 26
    "
    N N W W W N N
    N W W . W W N
    W W . . . W W
    W . . BB . . W
    W . BS P BS . W
    W . . BB . . W
    W W . . . W W
    N W W W W W N
    ",
    // Level 27
    "
    W W W W W W W
    W . . . . . W
    W . BS . BS . W
    W . BB . BB . W
    W . . P . . W
    W . . . . . W
    W W W W W W W
    ",
    // Level 28
    "
    N W W W W W N
    W W . . . W W
    W . . BB . . W
    W . BB P BB . W
    W . . BB . . W
    W . BS . BS . W
    W W . . . W W
    N W W W W W N
    ",
    // Level 29
    "
    W W W W W W W
    W . . . . . W
    W . BS . BS . W
    W . BB P BB . W
    W . BS . BS . W
    W . . . . . W
    W W W W W W W
    ",
    // Level 30: 最终挑战
    "
    N N W W W N N
    N W W . W W N
    W W . . . W W
    W . BB BB BB . W
    W . BS BS BS . W
    W . . P . . W
    W W . . . W W
    N W W W W W N
    "
];

/// 初始化指定关卡
/// 清除旧实体并加载新地图和资源
pub fn initialize_level(world: &mut World, ctx: &mut ggez::Context, level_index: usize) {
    if level_index < LEVELS.len() {
        // 创建核心游戏系统实体
        entity::create_gameplay(world);
        entity::create_time(world);
        entity::create_event_queue(world);
        entity::create_audio_store(world);
        // 加载地图
        load_map(world, LEVELS[level_index].to_string());
        // 加载音频资源
        load_sounds(world, ctx);
    }
}

/// 解析地图字符串并创建相应的实体
pub fn load_map(world: &mut World, map_string: String) {
    // 按行分割地图字符串
    let rows: Vec<&str> = map_string.trim().split('\n').map(|s| s.trim()).collect();
    for (y, row) in rows.iter().enumerate() {
        // 按空格分割每一行
        let columns: Vec<&str> = row.split(' ').collect();
        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
            };

            // 根据字符创建对应实体
            match *column {
                "." => {
                    create_floor(world, &position);
                }
                "W" => {
                    create_floor(world, &position);
                    create_wall(world, &position);
                }
                "P" => {
                    create_floor(world, &position);
                    create_player(world, &position);
                }
                "BB" => {
                    create_floor(world, &position);
                    create_box(world, &position, BoxColor::Blue);
                }
                "RB" => {
                    create_floor(world, &position);
                    create_box(world, &position, BoxColor::Red);
                }
                "BS" => {
                    create_floor(world, &position);
                    create_box_spot(world, &position, BoxColor::Blue);
                }
                "RS" => {
                    create_floor(world, &position);
                    create_box_spot(world, &position, BoxColor::Red);
                }
                "N" => (), // 空白区域，不做任何操作
                c => panic!("unrecognized map item {}", c), // 未知字符报错
            }
        }
    }
}
