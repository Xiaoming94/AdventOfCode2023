use std::collections::{HashMap, BTreeMap};

type SeedsVec = Vec<u64>;

type SourceDestMap = BTreeMap<u64,(u64,u64)>;

type AlmanacMap = HashMap<AlmanacPath, SourceDestMap>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
enum LocationType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humidity,
    Location
}

impl LocationType {
    pub fn next(&self) -> Self {
        match self {
            Self::Seed => Self::Soil,
            Self::Soil => Self::Fertilizer,
            Self::Fertilizer => Self::Water,
            Self::Water => Self::Light,
            Self::Light => Self::Temp,
            Self::Temp => Self::Humidity,
            Self::Humidity => Self::Location,
            Self::Location => Self::Location,
        }
    }
}

impl From<&str> for LocationType {
    fn from(args: &str) -> Self {
        match args {
            "seed" => Self::Seed,
            "soil" => Self::Soil,
            "fertilizer" => Self::Fertilizer,
            "water" => Self::Water,
            "light" => Self::Light,
            "temperature" => Self::Temp,
            "humidity" => Self::Humidity,
            "location" => Self::Location,
            _ => Self::Seed //Should be unreachable
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct AlmanacPath {
    from: LocationType,
    to: LocationType,
}

impl AlmanacPath {
    pub fn new() -> Self {
        AlmanacPath {
            from: LocationType::Seed,
            to: LocationType::Soil,
        }
    }

    pub fn construct_next(&self) -> Self {
        AlmanacPath {
            from: self.to,
            to: self.to.next(),
        }
    }
}

impl From<&str> for AlmanacPath {
    fn from(args: &str) -> Self {
        if let Some((path_name,_)) = args.split_once(" ") {
            if let Some((from,to)) = path_name.split_once("-to-") {
                Self::from((LocationType::from(from), LocationType::from(to)))
            } else {
                Self::new()
            }
        } else {
            Self::new()
        }
    }
}

impl From <(LocationType, LocationType)> for AlmanacPath {
    fn from(from_to_path: (LocationType, LocationType)) -> Self {
        let (from, to) = from_to_path;
        AlmanacPath {
            from, to
        }
    }
}


fn construct_path_graph(section: &str) -> (AlmanacPath, SourceDestMap) {
    if let Some((section_name, section_ranges)) = section.split_once("\n") {
        let path = AlmanacPath::from(section_name);
        let source_dest_map = section_ranges
            .split("\n")
            .fold(BTreeMap::new(), move |mut sd_map, current_range| {
                let vranges: Vec<u64> = current_range
                    .split_whitespace()
                    .map(|number| number.parse::<u64>().unwrap())
                    .collect();
                let (dest_start, source_start, range) = (
                    vranges[0],
                    vranges[1],
                    vranges[2],
                );
                sd_map.insert(source_start, (dest_start, range));
                sd_map
            });
            
        

        (path, source_dest_map)
    } else {
        (AlmanacPath::new(), BTreeMap::new())
    }
}

fn construct_maps(table_ranges: &str) -> AlmanacMap {
    table_ranges
        .split("\n\n")
        .map(construct_path_graph)
        .collect::<AlmanacMap>()
}

fn parse_seeds(seeds_str: &str) -> SeedsVec {
    if let Some((_, seednums)) = seeds_str.split_once(":") {
        seednums
            .split_whitespace()
            .map(|seedn| seedn.parse::<u64>().unwrap())
            .collect()
    } else {
        Vec::new()
    }
}

fn find_location(seed: u64, the_map: &AlmanacMap) -> u64 {
    let mut current_path = AlmanacPath::new();
    let mut current_location_val = seed;
    while(current_path != AlmanacPath {from: LocationType::Location, to: LocationType::Location}) {
        if let Some(ranges) = the_map.get(&current_path) {
            for (source_start, (dest_start, range)) in ranges{
                if (*source_start..(source_start+range)).contains(&current_location_val) {
                    let diff = current_location_val - source_start;
                    current_location_val = dest_start + diff;
                    break;
                }
            }
        }
        current_path = current_path.construct_next();
    }
    current_location_val
}

pub fn find_lowest_location(input_almanac: &str) -> u64 {
    println!("==== ENTERING FUNCTION ====");
    if let Some((seeds, table_ranges)) = input_almanac.split_once("\n\n") {
        let seeds_numbers: SeedsVec = parse_seeds(seeds);
        let tables = construct_maps(table_ranges);
        seeds_numbers
            .into_iter()
            .map(|seed| find_location(seed, &tables))
            .min()
            .unwrap()
    } else {
        1
    }
}