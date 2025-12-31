use std::{collections::HashMap, time::Duration};

use ggez::graphics::{self, Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment};
use glam::Vec2;
use hecs::{Entity, World};
use itertools::Itertools;

use crate::{component::{GamePlay, GameplayState, Position, Renderable, RenderableKind, Time}, constants::TITLE_WIDTH, entity, map::LEVELS};

/// 核心渲染系统
/// 负责绘制所有游戏实体、UI 和状态提示
pub fn run_rendering(world: &World, ctx: &mut ggez::Context, current_level_index: usize) {
    // 创建画布，设置灰色背景
    let mut canvas =
        graphics::Canvas::from_frame(ctx, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));
        
    // 获取时间组件，用于动画计算
    let mut query = world.query::<&Time>();
    let time = query.iter().next().unwrap().1;

    // 1. 收集所有需要渲染的实体
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();
    
    // 按 Z 轴排序，确保正确的遮挡关系（例如箱子在目标点上面）
    rendering_data.sort_by_key(|k| k.1.0.z);

    // 2. 准备渲染批次
    for (_, (position, renderable)) in rendering_data.iter() {
        let image = get_image( renderable, time.delta);
        let x = position.x as f32 * TITLE_WIDTH;
        let y = position.y as f32 * TITLE_WIDTH;
        let z = position.z;
        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        
        // 按 Z 轴和图片路径分组，以便批量绘制
        rendering_batches.entry(z).or_default().entry(image).or_default().push(draw_params);
    }

    // 3. 执行批量绘制
    for (_z, group) in rendering_batches
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
    {
        for (image_path, draw_params) in group {
            let image = Image::from_path(ctx, image_path).unwrap();
            let mut mesh_batch = graphics::InstanceArray::new(ctx, Some(image));

            for draw_param in draw_params.iter() {
                mesh_batch.push(*draw_param);
            }

            canvas.draw(&mesh_batch, graphics::DrawParam::new());
        }
    }

    // 4. 绘制 UI 信息（步数、FPS）
    let mut query = world.query::<&GamePlay>();
    let gameplay = query.iter().next().unwrap().1;
    let fps = format!("FPS: {:.0}", ctx.time.fps());
    
    // 在右侧显示状态信息
    draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0, Color::new(0.0, 0.0, 0.0, 1.0));
    draw_text(&mut canvas, &format!("Moves: {}", gameplay.move_count), 525.0, 100.0, Color::new(0.0, 0.0, 0.0, 1.0));
    draw_text(&mut canvas, &fps, 525.0, 120.0, Color::new(0.0, 0.0, 0.0, 1.0));

    // 绘制关卡列表
    draw_text(&mut canvas, "Level Select:", 525.0, 160.0, Color::new(0.0, 0.0, 0.0, 1.0));
    
    // 计算显示的关卡范围（滚动窗口）
    // 保持当前关卡在列表中间位置
    let visible_count = 12;
    let start_index = if current_level_index > 5 {
        current_level_index - 5
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
        let y = 190.0 + (display_index as f32 * 30.0);
        let color = if i == current_level_index {
            Color::new(0.0, 0.0, 1.0, 1.0) // 选中蓝色
        } else {
            Color::new(0.4, 0.4, 0.4, 1.0) // 未选中灰色
        };
        draw_text(&mut canvas, &format!("Level {}", i + 1), 540.0, y, color);
    }

    // 5. 绘制游戏结束状态提示
    match gameplay.state {
        GameplayState::Won => {
            let win_text = "You Won!\nPress Enter to Next Level";
            draw_center_text(&mut canvas, ctx, win_text, Color::from([0.0, 0.8, 0.0, 1.0]));
        }
        GameplayState::Lost => {
            let lost_text = "Game Over!\nPress R to Restart";
            draw_center_text(&mut canvas, ctx, lost_text, Color::from([0.8, 0.0, 0.0, 1.0]));
        }
        _ => {}
    }

    // 提交绘制命令
    canvas.finish(ctx).expect("expected to present");
}

/// 绘制普通文本
pub fn draw_text(canvas: &mut Canvas,text_str: &str, x: f32, y: f32, color: Color){
    let text = Text::new(TextFragment {
        text: text_str.to_string(),
        color: Some(color),
        scale: Some(PxScale::from(20.0)),
        ..Default::default()
    });

    canvas.draw(&text, Vec2::new(x, y));
}

/// 绘制屏幕居中文本（用于胜利/失败提示）
pub fn draw_center_text(canvas: &mut Canvas, ctx: &ggez::Context, text_str: &str, color: Color) {
    let mut text = Text::new(TextFragment {
        text: text_str.to_string(),
        color: Some(color),
        scale: Some(PxScale::from(48.0)),
        ..Default::default()
    });
    // 获取窗口大小
    let (w, h) = ctx.gfx.drawable_size();
    text.set_bounds(Vec2::new(w, h));
    
    // 计算文本尺寸以居中
    let text_dims = text.measure(ctx).unwrap();
    let x = (w - text_dims.x) / 2.0;
    let y = (h - text_dims.y) / 2.0;
    canvas.draw(&text, Vec2::new(x, y));
}

/// 获取当前需要渲染的图片路径（支持动画）
pub fn get_image(renderable: &Renderable,delta: Duration) -> String{
    let path_index= match renderable.kind() {
        RenderableKind::Static => {
            0
        }
        RenderableKind::Animated => {
            // 每 250ms 切换一帧
            ((delta.as_millis()% 1000)/250) as usize
        }
    };

    renderable.path(path_index)
}
