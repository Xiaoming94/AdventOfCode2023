use std::{collections::HashMap, arch::x86_64::__cpuid};

type Pos = (u32, u32);

#[derive(Debug, Eq, PartialEq)]
enum SchematicData {
    Dot,
    Symbol,
    Digit (u32)
}

struct DigitReader {
    value: u32,
    part_number_found: bool,
}

impl DigitReader {
    pub fn new () -> Self
    {
        DigitReader {
            value: 0,
            part_number_found: false
        }
    }
    
    pub fn read(&mut self, val: &u32)
    {
        self.value += self.value * 10 + val;
    }

    pub fn part_number_found(&mut self) {
        self.part_number_found = true;
    }

    pub fn has_found_part_number(&self) -> bool {
        self.part_number_found
    }
    pub fn get_digit(&self) -> u32 {
        self.value
    }

    pub fn reset(&mut self) {
        self.value = 0;
        self.part_number_found = false;
    }
    
}

type SchematicType = HashMap<Pos, SchematicData>;

fn read_schematic_symbol(sym: char) -> SchematicData
{
    match sym {
        '.'                     => SchematicData::Dot,
        sym if sym.is_numeric() => SchematicData::Digit(sym.to_digit(10).unwrap()),
        _                       => SchematicData::Symbol
    }
}

fn parse_schematic(schematic: &str) -> HashMap<Pos, SchematicData>
{
    let mut x = 0u32;
    schematic.chars().map(
        |c| {
                let schematic_entry = ((x, 0u32), read_schematic_symbol(c));
                x += 1;
                return schematic_entry;
            }).collect::<SchematicType>()
}

fn is_symbol_closeby(x: &u32, y: &u32, schematic: &SchematicType) -> bool
{
    false
}

fn find_partnumbers_helper(schematic: &SchematicType) -> Vec<u32>
{
    let mut digit_reader = DigitReader::new();
    let mut part_numbers = Vec::<u32>::new();
    let schematic_copy = schematic.clone();
    for ((x,y), schm) in schematic {
        match schm {
            SchematicData::Digit (val) => {
                digit_reader.read(val);
                if is_symbol_closeby(x,y,schematic_copy)
                {
                    digit_reader.part_number_found();
                }
            }
            _ => {
                if digit_reader.has_found_part_number() {
                    part_numbers.push(digit_reader.get_digit());
                }
                digit_reader.reset();
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