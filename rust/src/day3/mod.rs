use std::collections::{BTreeMap, HashMap};

use self::{types::*, digitreader::DigitReader};

mod types;
mod digitreader;

fn read_schematic_symbol(sym: char) -> SchematicData
{
    match sym {
        '.'                     => SchematicData::Dot,
        sym if sym.is_numeric() => SchematicData::Digit(sym.to_digit(10).unwrap()),
        _                       => SchematicData::Symbol (sym)
    }
}

fn parse_schematic(schematic: &str) -> SchematicType
{
    let mut y = 0;
    schematic.split("\n")
             .map(|line| line.to_owned() + ".")
             .fold( BTreeMap::new(), |mut schem_map, schematic_line| { 
                let mut x = 0;
                schematic_line.chars()
                              .for_each( |c| {
                    schem_map.insert(Pos::new(x, y), read_schematic_symbol(c));
                    x += 1;
                });
                y += 1;
                schem_map
            })
}

const OFFSETS: [(i32,i32);8] = [(1,0), (0,1), (1,1), (-1,0), (0,-1), (-1,-1), (-1,1), (1,-1)];

fn is_symbol_closeby(pos: &Pos, schematic: &SchematicType) -> bool
{
    OFFSETS.iter().any(
        |(dx,dy)| {
            if let Some(SchematicData::Symbol(_)) = schematic.get(&pos.added_pos(*dx, *dy)) {
                true
            } else {
                false
            }
        }
    )
}

fn find_partnumbers_helper(schematic: &SchematicType) -> Vec<u32>
{
    let mut digit_reader = DigitReader::new();
    let mut part_numbers = Vec::<u32>::new();
    for (pos, schm) in schematic {
        match schm {
            SchematicData::Digit (val) => {
                digit_reader.read(val);
                if is_symbol_closeby(pos, schematic)
                {
                    digit_reader.part_number_found();
                }
            }
            _ => {
                if let Some(read_value) = digit_reader.get_digit() {
                    if digit_reader.has_found_part_number() {
                        part_numbers.push(read_value);
                    }
                    digit_reader.reset();
                }
            }
        };
    } 
    part_numbers
}

// TURING MACHINE SOLUTION...
fn read_digits_tm(pos_to_read: Vec<Pos>, schematic: &SchematicType) -> Vec<u32>
{
    type PosToReadType = HashMap<Pos,bool>;
    let mut position_is_read: PosToReadType = pos_to_read.iter().map(|pos| (*pos, false)).collect();
    let mut digits = Vec::new();
    let mut digit_reader = DigitReader::new();
    for mut pos in pos_to_read {
        if let Some(false) = position_is_read.get(&pos) {
            while let Some(SchematicData::Digit(_)) = schematic.get(&pos.right()) {
                position_is_read.entry(pos).and_modify(|b| *b = true ).or_insert(true);
                pos = pos.right();
            }
            while let Some(SchematicData::Digit(val)) = schematic.get(&pos) {
                digit_reader.read(val);
                position_is_read.entry(pos).and_modify(|b| *b = true ).or_insert(true);
                pos = pos.left();
            }
            if let Some(read_digit) = digit_reader.get_digit() {
                digits.push(read_digit);
                digit_reader.reset();
            }
        }
    }
    digits
}

fn find_adjacent_numbers(pos: &Pos, schematic: &SchematicType) -> Vec<u32>
{
    let is_digit = |dx: i32, dy: i32| -> bool {
        if let Some(SchematicData::Digit(_)) = schematic.get(&pos.added_pos(dx, dy)) {
            true
        } else {
            false
        }
    };

    let pos_to_read: Vec<Pos> = OFFSETS.iter().filter(|(dx, dy)| is_digit(*dx, *dy))
                                       .map(|(dx, dy)| (pos.added_pos(*dx, *dy)))
                                       .collect();
    
    read_digits_tm(pos_to_read, schematic)
}

fn find_gear_ratios_helper(schematic: &SchematicType) -> Vec<u32>
{
    let is_asterix = move |schem: &SchematicData| -> bool {
        if let SchematicData::Symbol('*') = schem {
            true
        } else {
            false
        }
    };

    schematic.iter().filter(|(_,schem)| is_asterix(*schem))
                    .map(|(pos,_)| find_adjacent_numbers(pos, schematic))
                    .filter(|adjacent_nums| adjacent_nums.len() == 2)
                    .map(|gear_numbers| gear_numbers.iter().product())
                    .collect()
}

pub fn find_partnumbers(schematic: &str) -> Vec<u32>
{
    let parsed_schematic = parse_schematic(schematic);
    find_partnumbers_helper(&parsed_schematic)
}

pub fn find_gear_ratios(schematic: &str) -> Vec<u32> {
    let parsed_schematic = parse_schematic(schematic);
    find_gear_ratios_helper(&parsed_schematic)
}