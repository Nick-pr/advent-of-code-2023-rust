use std::ops::Range;

#[derive(Debug)]
pub struct Almanac {
    seeds: Box<[u64]>,
    seed_to_soil: AlmanacMapping,
    soil_to_fertilizer: AlmanacMapping,
    fertilizer_to_water: AlmanacMapping,
    water_to_light: AlmanacMapping,
    light_to_temperature: AlmanacMapping,
    tempature_to_humidity: AlmanacMapping,
    humidity_to_location: AlmanacMapping,
}

impl Almanac {
    pub fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.get(seed);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let tempature = self.light_to_temperature.get(light);
        let humidity = self.tempature_to_humidity.get(tempature);
        return self.humidity_to_location.get(humidity);
    }
    pub fn seeds(&self) -> impl Iterator<Item = &u64> {
        return self.seeds.iter();
    }
}
impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        let mut blocks = input.split("\n\n");

        let seed_block = blocks.next().unwrap();
        let seeds: Box<[u64]> = seed_block[6..]
            .split_whitespace()
            .map(|seed| seed.parse::<u64>().unwrap())
            .collect();

        let range_mapping = |block: &str| -> Vec<(u64, u64, u64)> {
            block
                .lines()
                .skip(1)
                .map(|x| {
                    let res: Vec<u64> = x
                        .splitn(3, " ")
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect();
                    (res[0], res[1], res[2])
                })
                .collect()
        };

        Almanac {
            seeds,
            seed_to_soil: AlmanacMapping::from(range_mapping(blocks.next().unwrap())),
            soil_to_fertilizer: AlmanacMapping::from(range_mapping(blocks.next().unwrap())),
            fertilizer_to_water: AlmanacMapping::from(range_mapping(blocks.next().unwrap())),
            water_to_light: AlmanacMapping::from(range_mapping(blocks.next().unwrap())),
            light_to_temperature: AlmanacMapping::from(range_mapping(blocks.next().unwrap())),
            tempature_to_humidity: AlmanacMapping::from(range_mapping(blocks.next().unwrap())),
            humidity_to_location: AlmanacMapping::from(range_mapping(blocks.next().unwrap())),
        }
    }
}

#[derive(Debug)]
pub struct AlmanacMapping(Vec<(Range<u64>, u64)>);

impl AlmanacMapping {
    pub fn get(&self, input: u64) -> u64 {
        return match self.0.iter().find(|(range, _)| range.contains(&input)) {
            None => input,
            Some((range, dest)) => dest + (input - range.start),
        };
    }
}

impl<const N: usize> From<[(u64, u64, u64); N]> for AlmanacMapping {
    fn from(values: [(u64, u64, u64); N]) -> Self {
        let mut mappings: Vec<(Range<u64>, u64)> = vec![];

        for value in values {
            let (dest, start, len) = value;
            mappings.push((start..start + len, dest));
        }
        return AlmanacMapping(mappings);
    }
}

impl From<Vec<(u64, u64, u64)>> for AlmanacMapping {
    fn from(values: Vec<(u64, u64, u64)>) -> Self {
        let mut mappings: Vec<(Range<u64>, u64)> = vec![];

        for value in values {
            let (dest, start, len) = value;
            mappings.push((start..start + len, dest));
        }

        return AlmanacMapping(mappings);
    }
}

#[cfg(test)]
mod tests {
    use super::AlmanacMapping;

    #[test]
    fn test_almanac_mapping() {
        let almanac_mapping = AlmanacMapping::from([(50, 98, 2), (52, 50, 48)]);
        assert_eq!(almanac_mapping.get(98), 50);
        assert_eq!(almanac_mapping.get(99), 51);
        assert_eq!(almanac_mapping.get(100), 100);
        assert_eq!(almanac_mapping.get(60), 62);
    }
}
