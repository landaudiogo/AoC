use std::{
    char,
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
pub struct Matrix {
    pub inner: Vec<Vec<char>>,
    pub height: usize,
    pub width: usize,
    pub area: HashMap<char, u64>,
}

impl Matrix {
    pub fn new<B: BufRead>(mut buf: B) -> Self {
        let mut line = String::new();
        let mut inner: Vec<Vec<char>> = Vec::new();

        while let Ok(len) = buf.read_line(&mut line) {
            if len == 0 {
                break;
            }

            inner.push(line.trim_end().chars().collect());
            line.truncate(0);
        }

        let height = inner.len();
        let width = inner[0].len();
        Matrix {
            inner,
            height,
            width,
            area: HashMap::new(),
        }
    }

    pub fn get_neighbours(&self, pos: (usize, usize), c: char) -> Vec<(char, (usize, usize))> {
        let mut neighbours = Vec::new();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for direction in directions {
            if let Ok(pos) = self.get_relative(pos, direction) {
                if self.inner[pos.0][pos.1] == c {
                    neighbours.push((c, pos));
                }
            }
        }

        neighbours
    }

    pub fn get_fences(&self, pos: (usize, usize), c: char) -> Vec<(Orientation, (usize, usize))> {
        let mut fences = Vec::new();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for direction in directions {
            let orientation = Orientation::try_from(direction).unwrap();
            let fence_x = if direction.0 == 1 { 1 } else { 0 };
            let fence_y = if direction.1 == 1 { 1 } else { 0 };
            if let Ok(n) = self.get_relative(pos, direction) {
                if self.inner[n.0][n.1] != c {
                    let fence = (orientation, (pos.0 + fence_x, pos.1 + fence_y));
                    // println!("=== {} {:?} {:?}", c, pos, fence);
                    fences.push(fence);
                }
            } else {
                let fence = (orientation, (pos.0 + fence_x, pos.1 + fence_y));
                // println!("=== {} {:?} {:?}", c, pos, fence);
                fences.push(fence);
            }
        }

        fences
    }

    pub fn get_relative(
        &self,
        current: (usize, usize),
        offset: (i64, i64),
    ) -> Result<(usize, usize), ()> {
        let current = convert_pos_usize_to_i64(current);
        let pos = (current.0 + offset.0, current.1 + offset.1);
        if !self.valid_position(pos) {
            return Err(());
        }

        Ok(convert_pos_i64_to_usize(pos))
    }

    pub fn valid_position(&self, pos: (i64, i64)) -> bool {
        pos.0 >= 0 && pos.0 < self.height as i64 && pos.1 >= 0 && pos.1 < self.width as i64
    }
}

fn convert_pos_usize_to_i64(pos: (usize, usize)) -> (i64, i64) {
    (pos.0 as i64, pos.1 as i64)
}

fn convert_pos_i64_to_usize(pos: (i64, i64)) -> (usize, usize) {
    (pos.0 as usize, pos.1 as usize)
}

#[derive(Debug)]
pub struct Region {
    pub area: u64,
    pub perimeter: u64,
    pub fences: Vec<(Orientation, (usize, usize))>,
}

impl Region {
    pub fn count_sides(&mut self) -> u64 {
        let mut sides = 0;
        for orientation in [Orientation::Up, Orientation::Down] {
            let mut horizontal: Vec<(usize, usize)> = self
                .fences
                .iter()
                .filter(|fence| fence.0 == orientation)
                .map(|fence| fence.1)
                .collect();
            horizontal.sort();
            let mut prev: Option<(usize, usize)> = None;
            for fence in horizontal {
                if let Some(prev) = prev {
                    if !(prev.0 == fence.0 && fence.1 == prev.1 + 1) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }
                prev = Some(fence);
            }
        }

        for orientation in [Orientation::Left, Orientation::Right] {
            let mut vertical: Vec<(usize, usize)> = self
                .fences
                .iter()
                .filter(|fence| fence.0 == orientation)
                .map(|fence| (fence.1 .1, fence.1 .0))
                .collect();
            vertical.sort();
            let mut prev: Option<(usize, usize)> = None;
            for fence in vertical {
                if let Some(prev) = prev {
                    if !(prev.0 == fence.0 && fence.1 == prev.1 + 1) {
                        sides += 1;
                    }
                } else {
                    sides += 1;
                }
                prev = Some(fence);
            }
        }

        sides
    }
}

#[derive(Debug)]
pub struct Regions {
    plant_region_map: HashMap<(char, (usize, usize)), usize>,
    region_plants: HashMap<usize, Vec<(char, (usize, usize))>>,
    pub regions: HashMap<usize, Region>,
    next_id: usize,
}

impl Regions {
    pub fn new() -> Self {
        Regions {
            plant_region_map: HashMap::new(),
            region_plants: HashMap::new(),
            regions: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn merge_regions(&mut self, regions: HashSet<usize>) -> usize {
        if regions.len() == 1 {
            return regions.into_iter().next().unwrap();
        }

        let mut merged_regions = Region {
            area: 0,
            perimeter: 0,
            fences: Vec::new(),
        };
        let next_id = self.next_id;
        self.next_id += 1;
        let mut merged_regions_plants = Vec::new();
        for region_id in regions {
            let region = self.regions.remove(&region_id).unwrap();
            merged_regions.perimeter += region.perimeter;
            merged_regions.area += region.area;
            merged_regions.fences.extend(region.fences);
            let plants = self.region_plants.remove(&region_id).unwrap();
            for plant in plants.iter() {
                *self.plant_region_map.get_mut(plant).unwrap() = next_id;
            }
            merged_regions_plants.extend(plants);
        }
        self.region_plants.insert(next_id, merged_regions_plants);
        self.regions.insert(next_id, merged_regions);
        next_id
    }

    pub fn add_to_region(
        &mut self,
        plant: (char, (usize, usize)),
        region_id: usize,
        perimeter: u64,
    ) {
        self.region_plants.get_mut(&region_id).unwrap().push(plant);
        self.plant_region_map.insert(plant, region_id);
        let region = self.regions.get_mut(&region_id).unwrap();
        region.area += 1;
        region.perimeter += perimeter;
    }

    pub fn add_to_region_w_fences(
        &mut self,
        plant: (char, (usize, usize)),
        region_id: usize,
        fences: Vec<(Orientation, (usize, usize))>,
    ) {
        self.region_plants.get_mut(&region_id).unwrap().push(plant);
        self.plant_region_map.insert(plant, region_id);
        let region = self.regions.get_mut(&region_id).unwrap();
        region.area += 1;
        region.perimeter += fences.len() as u64;
        region.fences.extend(fences);
    }

    pub fn add_to_new_region_w_fences(
        &mut self,
        plant: (char, (usize, usize)),
        fences: Vec<(Orientation, (usize, usize))>,
    ) {
        let next_id = self.next_id;
        self.next_id += 1;
        self.regions.insert(
            next_id,
            Region {
                perimeter: fences.len() as u64,
                area: 1,
                fences,
            },
        );
        self.region_plants.insert(next_id, vec![plant]);
        self.plant_region_map.insert(plant, next_id);
    }

    pub fn add_to_new_region(&mut self, plant: (char, (usize, usize)), perimeter: u64) {
        let next_id = self.next_id;
        self.next_id += 1;
        self.regions.insert(
            next_id,
            Region {
                perimeter,
                area: 1,
                fences: Vec::new(),
            },
        );
        self.region_plants.insert(next_id, vec![plant]);
        self.plant_region_map.insert(plant, next_id);
    }

    pub fn get_regions(&self, plants: &[(char, (usize, usize))]) -> HashSet<usize> {
        plants
            .into_iter()
            .map(|plant| self.plant_region_map.get(plant).map(|region| *region))
            .flatten()
            .collect()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<(i64, i64)> for Orientation {
    type Error = ();
    fn try_from(value: (i64, i64)) -> Result<Self, Self::Error> {
        match value {
            (0, 1) => Ok(Self::Right),
            (0, -1) => Ok(Self::Left),
            (1, 0) => Ok(Self::Down),
            (-1, 0) => Ok(Self::Up),
            _ => Err(()),
        }
    }
}
