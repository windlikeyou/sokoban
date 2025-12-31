use crate::component::{
    AudioStore, Box, BoxColor, BoxSpot, EventQueue, GamePlay, Immovable, Moveable, Player, Position, Renderable, Time, Wall
};
use ggez::audio::Source;
use hecs::{Entity, World};

pub fn create_wall(world: &mut World, position: &Position) -> Entity {
    world.spawn((
        Position { z: 10, ..*position },
        Renderable::new_static("/images/wall.png"),
        Wall {},
        Immovable {},
    ))
}
pub fn create_floor(world: &mut World, position: &Position) -> Entity {
    world.spawn((
        Position { z: 5, ..*position },
        Renderable::new_static("/images/floor.png"),
    ))
}

pub fn create_box(world: &mut World, position: &Position, color: BoxColor) -> Entity {
    world.spawn((
        Position { z: 10, ..*position },
        Renderable::new_animated(vec![
            format!("/images/box_{}_1.png", color),
            format!("/images/box_{}_2.png", color),
        ]),
        Box { color },
        Moveable {},
    ))
}

pub fn create_box_spot(world: &mut World, position: &Position, color: BoxColor) -> Entity {
    world.spawn((
        Position { z: 9, ..*position },
        Renderable::new_static(&format!("/images/box_spot_{}.png", color)),
        BoxSpot { color },
    ))
}

pub fn create_player(world: &mut World, position: &Position) -> Entity {
    world.spawn((
        Position { z: 10, ..*position },
        Renderable::new_animated(vec![
            "/images/player_1.png".to_string(),
            "/images/player_2.png".to_string(),
            "/images/player_3.png".to_string(),
        ]),
        Player {},
        Moveable {},
    ))
}

pub fn create_gameplay(world: &mut World) -> Entity {
    world.spawn((GamePlay::default(),))
}

pub fn create_time(world:&mut World)-> Entity{
    world.spawn((Time::default(),))
}

pub fn create_event_queue(world: &mut World) -> Entity {
    world.spawn((EventQueue::default(),))
}

pub fn create_audio_store(world: &mut World) -> Entity {
    world.spawn((AudioStore::default(),))
}

pub fn load_sounds(world: &mut World,ctx: &mut ggez::Context){
    let mut audio_store = world.query::<&mut AudioStore>();
    let audio_store = audio_store.iter().next().unwrap().1;
    let sounds = ["correct", "incorrect", "wall"];

    for sound in sounds.iter() {
        let sound_name = sound.to_string();
        let sound_path = format!("/sounds/{}.wav", sound_name);
        let sound_source = Source::new(ctx, sound_path).expect(&format!("failed to load sound {}", sound_name));
        audio_store.sounds.insert(sound_name, std::boxed::Box::new(sound_source));
    }
}