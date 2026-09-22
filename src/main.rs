#![allow(dead_code, unused_variables, unused_mut)]

/// Entry point for Chapter 5.1: Defining and Instantiating Structs
///
/// Structs let the programmer define a custom data type. This file models a
/// small game world while introducing named-field structs, field shorthand,
/// struct update syntax, tuple structs, unit-like structs, and ownership of
/// struct data.
fn main() {
    grouping_character_data();

    field_init();
    struct_update();

    tuple_structs();
    unit_like_struct();
    moving_owned_data_into_struct();
}

/// A named-field struct describing a game character.
struct Character {
    name: String,
    health: u32,
    level: u32,
    is_npc: bool,
}

/// A tuple struct for a location in the game's map grid.
struct MapPosition(i32, i32);

/// A tuple struct for a pixel offset on the player's screen.
struct ScreenOffset(i32, i32);

/// A unit-like struct that marks one set of game rules.
struct HardcoreMode;

/// # Grouping Character Data
/// - A struct definition creates a new type from related pieces of data.
/// - The field names should be descriptive as always.
/// - Creating an instance of the struct requires a value for each field.
fn grouping_character_data() {
    println!("\n{:=>80}", "");
    println!("grouping_character_data();\n");

    let mut hero = Character {
        name: String::from("Iines"),
        health: 100,
        level: 1,
        is_npc: false,
    };

    println!("{} begins with {} health.", hero.name, hero.health);

    // Fields can be updated, but the entire struct binding must be mutable.
    hero.health = 75;
    println!("After a trap, {} has {} health.", hero.name, hero.health);

    // A mutable field inside an immutable struct is not possible.
    // let hero_b = Character {
    //     name: String::from("Aku"),
    //     health: 42,
    //     level: 0,
    //     is_npc: false,
    // };
    // hero_b.health = 75; // Uncomment to see the error.
}

/// # Field Init Shorthand
/// - A function can receive values, then use them to construct a struct.
/// - If a variable name matches a field name, `name,` means `name: name,`.
/// - Shorthand removes repetition without hiding where the value comes from.
fn field_init() {
    println!("\n{:=>80}", "");
    println!("field_init()\n");

    let guide = recruit_character("Leenu", 100, 3);

    println!(
        "{} is a level {} non-player character: {}.",
        guide.name, guide.level, guide.is_npc
    );
}

/// # Struct Update Syntax
/// - `..existing_value` uses the fields not explicitly named from
///   `existing_value`.
/// - This is useful when making a variation of an existing value.
/// - The expression moves non-Copy fields, such as `String`, into the new
///   struct value.
fn struct_update() {
    println!("\n{:=>80}", "");
    println!("creating_a_variant_with_struct_update_syntax()\n");

    let village_guard = Character {
        name: String::from("Liinu"),
        health: 100,
        level: 4,
        is_npc: true,
    };

    // Every Character needs values for all four fields. Here we provide a new
    // `level` and write `..village_guard` to take every remaining field
    // (`name`, `health`, and `is_npc`) from village_guard.
    //
    // This is equivalent to writing each of those fields explicitly. The
    // `..village_guard` expression must be last because it fills in all fields
    // that have not already been written.
    //
    // The guard becomes an elite version with only the level changed.
    let elite_guard = Character {
        level: 8,
        ..village_guard
    };

    println!(
        "{} is now level {} with {} health.",
        elite_guard.name, elite_guard.level, elite_guard.health
    );

    // `name` moved into `elite_guard`, so the original value cannot be used
    // as a complete Character anymore.
    // println!("{}", village_guard.name); // Uncomment to see the error.
    //
    // `health` is a u32 and `is_npc` is a bool. Both implement Copy, so their
    // individual values are still available.
    println!("The original guard was an NPC: {}", village_guard.is_npc);
}

/// # Tuple Structs
/// - Tuple structs give a name to a group of values without naming each field.
/// - Their fields are accessed with numeric indices.
/// - Naming the type prevents values with the same representation from being
///   accidentally mixed up and of course lets us use them as types.
fn tuple_structs() {
    println!("\n{:=>80}", "");
    println!("tuple_structs()\n");

    let starting_position = MapPosition(12, 7);
    let camera_offset = ScreenOffset(12, 7);

    println!(
        "Spawn at map coordinates ({}, {}).",
        starting_position.0, starting_position.1
    );
    println!(
        "Draw the camera {} pixels right and {} pixels down.",
        camera_offset.0, camera_offset.1
    );

    // Both values contain two i32s, but they describe different concepts.
    // let position: MapPosition = camera_offset; // Uncomment to see the error.
}

/// # Unit-Like Structs
/// - A unit-like struct has a name but no fields.
/// - It is useful as a distinct type when a program needs no stored data.
/// - We will see traits later; they can give this marker type its own behavior.
fn unit_like_struct() {
    println!("\n{:=>80}", "");
    println!("unit_like_struct()\n");

    let game_rules = HardcoreMode;
    println!("HardcoreMode exists even though it stores no data.");
}

/// # Moving Owned Data into a Struct
/// - `Character` owns its `String` name, just as it owns its numeric fields.
/// - Passing a `String` into a struct moves ownership to the new struct value.
/// - A struct can store references instead, but that requires lifetime
///   annotations to prove the referenced values live long enough.
fn moving_owned_data_into_struct() {
    println!("\n{:=>80}", "");
    println!("moving_owned_data_into_struct()\n");

    let name = String::from("Tiinu");
    let character = Character {
        name,
        health: 100,
        level: 2,
        is_npc: true,
    };

    println!(
        "{} joins the party at level {}.",
        character.name, character.level
    );

    // The String now belongs to `character`, not the `name` variable.
    // println!("{name}"); // Uncomment to see the use-after-move error.

    // References in a struct need a lifetime annotation. Lifetimes will be
    // discussed later during the course. Uncomment to see the error.
    // struct TemporaryCharacter {
    //     name: &str,
    // }
}

/// Creates a non-player character from given data and a default value to
/// `is_npc`.
///
/// Accepting `&str` lets callers pass a string literal, a `&String`, or a
/// slice of a String. The Character needs to own its name, so `String::from`
/// creates owned data for the struct.
///
/// Here the variable names health and level correspond to the fields in
/// Character struct.
fn recruit_character(name: &str, health: u32, level: u32) -> Character {
    Character {
        name: String::from(name),
        health,
        level,
        is_npc: true,
    }
}
