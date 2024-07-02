use crate::prelude::*;
pub fn spawn_player(esc: &mut World, pos: Point) {
    esc.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 20,
            max: 20,
        },
    ));
}

fn goblin() -> (i32, String, FontCharType) {
    (1, "Goblin".to_owned(), to_cp437('g'))
}

fn orc() -> (i32, String, FontCharType) {
    (2, "Orc".to_owned(), to_cp437('o'))
}

pub fn spawn_monster(ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    pos: Point) {

    let (hp, name, glyph) = match rng.roll_dice(1, 10){
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: glyph,
        },
        MovingRandomly {},
        Health{current: hp, max: hp},
        Name(name)
    ));
}
