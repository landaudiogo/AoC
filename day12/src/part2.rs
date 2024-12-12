use std::io::BufRead;

use crate::common::{Matrix, Region, Regions};

pub fn run<B: BufRead>(buf: B) -> u64 {
    let matrix = Matrix::new(buf);
    let mut regions = Regions::new();

    for i in 0..matrix.height {
        for j in 0..matrix.width {
            let plant = (matrix.inner[i][j], (i, j));
            let fences = matrix.get_fences(plant.1, plant.0);
            let neighbours = matrix.get_neighbours(plant.1, plant.0);
            let neighbour_regions = regions.get_regions(neighbours.as_slice());
            match neighbour_regions.len() {
                0 => {
                    regions.add_to_new_region_w_fences(plant, fences);
                }
                _ => {
                    let region = regions.merge_regions(neighbour_regions);
                    regions.add_to_region_w_fences(plant, region, fences);
                }
            };
        }
    }

    let mut total = 0;
    for (_, mut region) in regions.regions {
        total += region.area * region.count_sides();
    }
    total
}
