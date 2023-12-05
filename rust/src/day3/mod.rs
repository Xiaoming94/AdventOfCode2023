use std::collections::BTreeMap;

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

fn is_symbol_closeby(x: &i32, y: &i32, schematic: &SchematicType) -> bool
{
    let dxdys = vec![(1,0), (0,1), (1,1), (-1,0), (0,-1), (-1,-1), (-1,1), (1,-1)];
    dxdys.iter().any(
        |(dx,dy)| {
            if let Some(entry) = schematic.get(&Pos::from((x + dx, y + dy))) {
                match entry {
                    SchematicData::Symbol(_) => true,
                    _ => false
                }

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
    let schematic_copy = schematic.clone();
    for (Pos {x, y}, schm) in schematic {
        match schm {
            SchematicData::Digit (val) => {
                digit_reader.read(val);
                if is_symbol_closeby(x, y, schematic_copy)
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

pub fn find_partnumbers(schematic: &str) -> Vec<u32>
{
    let parsed_schematic = parse_schematic(schematic);
    find_partnumbers_helper(&parsed_schematic)
}