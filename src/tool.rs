use ggez::{
    input::{keyboard::KeyCode, mouse::MouseButton}, GameResult, event,
};
use hecs::{ World};

use crate::{component::{GameplayState, GamePlay, Time}, map::{initialize_level, LEVELS}, systems::{gameplay::run_gameplay_state, input::run_input, rendering::run_rendering}};

/// 游戏主结构体，维护游戏世界和当前关卡状态
pub struct Game {
    /// ECS 世界，存储所有实体和组件
    pub world: World,
    /// 当前关卡索引
    pub current_level: usize,
}

impl Game {
    /// 创建新的游戏实例
    /// 初始化 ECS 世界并加载第一个关卡
    pub fn new(ctx: &mut ggez::Context) -> GameResult<Game> {
        let mut world = World::new();
        initialize_level(&mut world, ctx, 0);
        Ok(Game {
            world,
            current_level: 0,
        })
    }
}

/// 实现 ggez 的 EventHandler trait，处理游戏循环
impl event::EventHandler<ggez::GameError> for Game {
    /// 更新游戏状态（每帧调用）
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        {
            // 运行输入处理系统
            run_input(&self.world, ctx);
        }
        {
            // 运行游戏逻辑系统（移动、状态检查等）
            run_gameplay_state(&self.world);
        }
        {
            // 更新时间组件
            let mut query = self.world.query::<&mut Time>();
            let time = query.iter().next().unwrap().1;
            time.delta += ctx.time.delta();
        }

        // 处理全局状态转换（下一关/重试）
        // 这里需要获取 GamePlay 状态，但因为 run_gameplay_state 可能刚修改了它
        let mut next_action = None; // 0: nothing, 1: next level, 2: restart

        {
            let mut query = self.world.query::<&GamePlay>();
            if let Some(gameplay) = query.iter().next().map(|(_, g)| g) {
                // 如果游戏胜利，按回车键进入下一关
                if gameplay.state == GameplayState::Won {
                    if ctx.keyboard.is_key_just_pressed(KeyCode::Return) {
                        next_action = Some(1);
                    }
                // 如果游戏失败（死锁），按 R 键重试
                } else if gameplay.state == GameplayState::Lost {
                    if ctx.keyboard.is_key_just_pressed(KeyCode::R) {
                        next_action = Some(2);
                    }
                }
            }
        }

        // 执行状态转换操作
        if let Some(action) = next_action {
            match action {
                1 => {
                    // 进入下一关
                    self.current_level += 1;
                    if self.current_level >= LEVELS.len() {
                        self.current_level = 0; // 循环回到第一关
                    }
                    self.world.clear();
                    initialize_level(&mut self.world, ctx, self.current_level);
                }
                2 => {
                    // 重新开始当前关卡
                    self.world.clear();
                    initialize_level(&mut self.world, ctx, self.current_level);
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// 绘制游戏画面（每帧调用）
    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        {
            // 运行渲染系统
            run_rendering(&self.world, ctx, self.current_level);
        }
        Ok(())
    }

    /// 处理鼠标点击事件
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        // 检查是否点击了关卡列表区域
        // 渲染位置参考 rendering.rs:
        // Level Select: 525.0, 160.0
        // Level i: 540.0, 190.0 + i * 30.0
        // 文本高度约 20.0
        
        let start_x = 540.0;
        let end_x = 700.0; // 假设宽度足够覆盖文本
        let start_y_base = 190.0;
        let item_height = 30.0;
        let text_height = 20.0;

        // 计算显示的关卡范围（滚动窗口），与 rendering.rs 保持一致
        let visible_count = 12;
        let start_index = if self.current_level > 5 {
            self.current_level - 5
        } else {
            0
        };
        let start_index = if start_index + visible_count > LEVELS.len() {
            LEVELS.len().saturating_sub(visible_count)
        } else {
            start_index
        };
        let end_index = std::cmp::min(start_index + visible_count, LEVELS.len());

        for i in start_index..end_index {
            let display_index = i - start_index;
            let item_y = start_y_base + (display_index as f32 * item_height);
            
            if x >= start_x && x <= end_x && y >= item_y && y <= item_y + text_height {
                // 点击了第 i 关
                if self.current_level != i {
                    self.current_level = i;
                    self.world.clear();
                    initialize_level(&mut self.world, ctx, self.current_level);
                }
                break;
            }
        }

        Ok(())
    }
}
