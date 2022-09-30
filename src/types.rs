use crate::traits::Parse;
use derivative::Derivative;
use indexmap::IndexMap;
use serde::*;
pub use wedit_macros::Parse;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct World {
    pub version: i32,
    pub filetype: u8,
    pub sections: IndexMap<i32, Section>,
    #[derivative(Debug = "ignore")]
    pub rest: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub enum Section {
    Header(Header),
    Unknown(Vec<u8>),
}

#[repr(transparent)]
pub struct VersionReq(i32);

#[derive(Parse, Debug, Serialize)]
pub struct Header {
    pub world_name: String,
    pub world_seed: String,
    pub generator_version: i64,
    pub guid: [u8; 16],
    pub world_id: i32,
    pub world_bounds: Rect,
    pub world_height: i32,
    pub world_width: i32,
    pub game_mode: i32,
    pub drunk_world: bool,
    pub get_good_world: bool,
    pub expert_mode: bool,
    pub master_mode: bool,
    pub creation_time: i64,
    pub moon_type: u8,
    pub tree_x: [i32; 3],
    pub tree_style: [i32; 4],
    pub cave_back_x: [i32; 3],
    pub cave_back_style: [i32; 4],
    pub ice_back_style: i32,
    pub jungle_back_style: i32,
    pub hell_back_style: i32,
    pub spawn_tile_x: i32,
    pub spawn_tile_y: i32,
    pub world_surface: f64,
    pub rock_layer: f64,
    pub time: f64,
    pub day_time: bool,
    pub moon_phase: i32,
    pub blood_moon: bool,
    pub eclipse: bool,
    pub dungeon_x: i32,
    pub dungeon_y: i32,
    pub crimson: bool,
    pub killed_boss_eye_of_cthulu: bool,
    pub killed_boss_eater_of_worlds: bool,
    pub killed_boss_skeletron: bool,
    pub killed_boss_queen_bee: bool,
    pub killed_boss_the_destroyer: bool,
    pub killed_boss_the_twins: bool,
    pub killed_boss_skeletron_prime: bool,
    pub killed_any_hardmode_boss: bool,
    pub killed_boss_plantera: bool,
    pub killed_boss_golem: bool,
    pub killed_boss_slime_king: bool,
    pub saved_goblin_tinkerer: bool,
    pub saved_wizard: bool,
    pub saved_mechanic: bool,
}

// /// Bool	1	Either 0 for false, or 1 for true
/// Byte	1	Unsigned 0 .. 255
/// Int16	2	Signed -32,768 .. 32,767
/// Int32	4	Signed -2,147,483,648 .. 2,147,483,647
/// String	*	Pascal string. First byte contains the length of the string.
/// Single	4	Single-precision float
/// Double	8	Double-precision float
/// Rect	16	4 Int32s that form a rectangle (left, right, top, bottom)
pub enum Types {
    Bool,
    Byte,
    Int16,
    Int32,
    String,
    Single,
    Double,
    Rect,
}

#[repr(C)]
#[derive(Debug, Serialize)]
pub struct Rect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}
