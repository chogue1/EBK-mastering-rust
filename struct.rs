// struct.rs
struct Character {
	strength: u8,
	dexterity: u8,
	consitution: u8,
	wisdom: u8,
	intelligence: u8,
	charisma: u8,
	name: String
}

fn main() {
	let char = Character { strength: 9, dexterity: 9, consitution: 9,
	wisdom: 9, intelligence: 9, charisma: 9,
	name: "Generic AD&D Hero".to_string() };

	println!("Character's name is {}, and his/her strength is {}",
	char.name, char.strength);
}

// Type            -- Description                  -- Possible values            |
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - |
// bool            -- Booleans                     -- true, false                |
// u8/u16/u32/u64  -- Fixed size unsigned integers -- Unsigned range determined  |
//                                                    by bit size                |
// i8/i16/i32/i64  -- Fixed size signed integers   -- Signed range determined by |
//                                                    bit size                   |
// f32/f64         -- Fixed size floats            -- Float range determined by  |
//                                                    bit size (IEEE-754)        |
// usize           -- Architecture-dependant       -- Depending on target        |
//                    unsigned integer                machine, usually 32 or 64  |
//                                                    bit value                  |
// isize           -- Architecture-dependant       -- Depending on target        |
//                    signed integer                  machine, usually 32 or 64  |
//                                                    bit value                  |
// str             -- String slice                 -- Unicode string             |
// [T; N]          -- Fixed-size arrays            -- N number of type T values  |
// &[T]            -- Slices                       -- References to values of    |
//                                                    type T                     |
// (T1, T2, ...)   -- Tuples                       -- Elements of types T1, T2 . |
// fn(T1, T2, ...) -- Functions                    -- Functions that take types  |
//                                                    T1, T2, ... as parameters, |
//                                                    returns value of type R    |
// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - | 
