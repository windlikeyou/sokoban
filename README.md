# rust-sokoban

一个用 Rust 编写的推箱子（Sokoban）小游戏，基于 [ggez](https://github.com/ggez/ggez) 构建渲染与事件循环，使用 [hecs](https://github.com/Ralith/hecs) 作为 ECS（Entity Component System）来组织游戏对象与系统逻辑。

关键词：Rust / Sokoban / 推箱子 / ggez / ECS / hecs / 2D Game

## 特性

- 30 个关卡，难度梯度上升
- 右侧关卡列表，支持鼠标点击跳转
- 胜利/失败提示
  - 胜利：提示后按回车进入下一关
  - 失败：提示后按 R 重新开始当前关卡
- 防止按键长按导致一次移动多格（使用“just pressed”触发）

## 运行

需要 Rust 工具链（支持 edition 2024）。

```bash
cd rust-sokoban
cargo run
```

## 操作说明

- 方向键：移动
- 回车：胜利后进入下一关
- R：失败后重新开始当前关卡
- 鼠标：点击右侧 `Level Select` 列表选择关卡

## 资源目录

资源文件位于 [resources](file:///d:/workspace/rust-demo/rust-sokoban/resources)：

- `resources/images`：精灵图
- `resources/sounds`：音效

## 项目结构

- [src/main.rs](file:///d:/workspace/rust-demo/rust-sokoban/src/main.rs)：窗口初始化与事件循环
- [src/tool.rs](file:///d:/workspace/rust-demo/rust-sokoban/src/tool.rs)：游戏主结构（关卡切换、鼠标选关）
- [src/map.rs](file:///d:/workspace/rust-demo/rust-sokoban/src/map.rs)：关卡数据与地图解析
- [src/component.rs](file:///d:/workspace/rust-demo/rust-sokoban/src/component.rs)：ECS 组件与状态
- [src/systems](file:///d:/workspace/rust-demo/rust-sokoban/src/systems)：输入/逻辑/渲染等系统

## 构建发布版本

```bash
cargo build --release
```

