use std::{collections::BTreeMap, cmp::Ordering};

#[derive(Debug, Eq, PartialEq)]
enum SchematicData {
    Dot,
    Symbol,
    Digit (u32)
}

#[derive(Debug, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos {
            x, y
        }
    }
}

impl From<(i32,i32)> for Pos {
    fn from(arg: (i32, i32)) -> Self {
       let (x,y) = arg;
       Pos::new(x, y)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        if (self.x, self.y) == (other.x, other.y) {
            Ordering::Equal 
        }
        else if self.y == other.y {
            if self.x > other.x {
                Ordering::Greater
            }
            else {
                Ordering::Less
            }
        } else {
            if self.y > other.y {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
        self.value = self.value * 10 + val;
    }

    pub fn part_number_found(&mut self) {
        self.part_number_found = true;
    }

    pub fn has_found_part_number(&self) -> bool {
        self.part_number_found
    }
    pub fn get_digit(&self) -> Option<u32> {
        if self.value > 0{
            Some(self.value)
        }
        else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.value = 0;
        self.part_number_found = false;
    } 
}

type SchematicType = BTreeMap<Pos, SchematicData>;

fn read_schematic_symbol(sym: char) -> SchematicData
{
    match sym {
        '.'                     => SchematicData::Dot,
        sym if sym.is_numeric() => SchematicData::Digit(sym.to_digit(10).unwrap()),
        _                       => SchematicData::Symbol
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
                    SchematicData::Symbol => true,
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