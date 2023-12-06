#[cfg(test)]
mod day05 {
    use std::cmp;
    use std::ops::{Range, RangeInclusive};

    use itertools::Itertools;
    use regex::Regex;

    use crate::read_data_file;

    static SAMPLE: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    struct DestinationToSourceRange {
        destination_start: u64,
        source_start: u64,
        length: u64,
        convert_factor: i128
    }

    impl DestinationToSourceRange {
        fn source_range(&self) -> Range<u64> {
            return (self.source_start)..(self.source_start + self.length);
        }

        fn destination_range(&self) -> Range<u64> {
            return (self.destination_start)..(self.destination_start + self.length);
        }

        fn map_source(&self, source: &u64) -> Option<u64> {
            let source_range = self.source_range();
            if source_range.contains(source) {
                return Some((source.to_owned() as i128  + self.convert_factor) as u64);
            }

            return None;
        }

        fn intersect(&self, rng: &Range<u64>) -> (Vec<Range<u64>>, Vec<Range<u64>>) {
            let mut unmatched: Vec<Range<u64>> = Vec::new();
            let mut matched_and_translated: Vec<Range<u64>> = Vec::new();

            let self_rng = self.source_range();

            if rng.start < self.source_start {
                let end = cmp::min(rng.end, self.source_start);
                unmatched.push(rng.start..end);
            }

            if rng.end > self_rng.end {
                let start = cmp::max(rng.start, self_rng.start);
                unmatched.push(start..rng.end);
            }

            if self_rng.contains(&rng.start) || self_rng.contains(&(rng.end - 1)) {
                let start = cmp::max(rng.start, self_rng.start);
                let end = cmp::min(rng.end, self_rng.end);

                matched_and_translated.push((start as i128 + self.convert_factor) as u64..(end as i128+self.convert_factor) as u64)
            }

            return (unmatched, matched_and_translated);
        }
    }

    struct Section {
        destination_type: String,
        source_type: String,
        ranges: Vec<DestinationToSourceRange>
    }

    impl Section {
        fn map_source_to_destination(&self, source: &u64) -> u64 {
            return self.find_range_for_source(source)
                .map(|r| r.map_source(source).unwrap())
                .unwrap_or(source.to_owned());
        }

        fn find_range_for_source(&self, source: &u64) -> Option<&DestinationToSourceRange> {
            return self
                .ranges
                .iter()
                .find(|r| r.source_range().contains(source));
        }

        fn map_source_range_to_destination(&self, input: &Range<u64>) -> Vec<Range<u64>> {
            let mut unmatched = vec![input.to_owned()];
            let mut results: Vec<Range<u64>> = vec![];

            for r in &self.ranges {
                let mut new_unmatched: Vec<Range<u64>> = vec![];

                for existing_unmatched in &unmatched {
                    let (mut um, mut mt) = r.intersect(existing_unmatched);
                    new_unmatched.append(&mut um);
                    results.append(&mut mt);
                }

                unmatched = new_unmatched;
            }

            results.append(&mut unmatched);

            return results;
        }
    }

    struct FarmData {
        seeds: Vec<u64>,
        seed_to_soil: Section,
        soil_to_fertilizer: Section,
        fertilizer_to_water: Section,
        water_to_light: Section,
        light_to_temperature: Section,
        temperature_to_humidity: Section,
        humidity_to_location: Section
    }

    impl FarmData {
        fn seed_ranges(&self) -> Vec<Range<u64>> {
            return self.seeds
                .iter()
                .chunks(2)
                .into_iter()
                .map(|c| c.collect_vec())
                .map(|c| c[0].to_owned()..(c[0] + c[1]))
                .collect_vec();
        }
    }

    fn parse_file_into_areas(input: &str) -> Vec<&str> {
        return input
            .split("\n\n")
            .collect_vec();
    }

    fn parse_section(input: &str) -> Section {
        let lines = input.lines().collect_vec();
        let regex = Regex::new(r"^(.*)-to-(.*) map:$").unwrap();
        let caps = regex.captures(lines[0]).unwrap();
        let source = &caps[1];
        let destination = &caps[2];

        let ranges = lines.get(1..lines.len()).unwrap()
            .iter()
            .map(|l| {
                let vs = l.split(" ").collect_vec();
                let destination_start = vs[0].parse::<u64>().unwrap();
                let source_start = vs[1].parse::<u64>().unwrap();
                let length = vs[2].parse::<u64>().unwrap();
                let convert_factor = destination_start as i128 - source_start as i128;

                return DestinationToSourceRange {
                    destination_start,
                    source_start,
                    length,
                    convert_factor
                }
            })
            .collect_vec();

        return Section {
            source_type: source.to_owned(),
            destination_type: destination.to_owned(),
            ranges
        }
    }

    fn parse_seeds(input: &str) -> Vec<u64> {
        let seed_list = input.strip_prefix("seeds: ").unwrap();

        return seed_list
            .split(" ")
            .map(|v| v.parse::<u64>().unwrap())
            .collect_vec();
    }

    fn parse_farm_data(input: &str) -> FarmData {
        let areas = parse_file_into_areas(input);

        return FarmData {
            seeds: parse_seeds(areas[0]),
            seed_to_soil: parse_section(areas[1]),
            soil_to_fertilizer: parse_section(areas[2]),
            fertilizer_to_water: parse_section(areas[3]),
            water_to_light: parse_section(areas[4]),
            light_to_temperature: parse_section(areas[5]),
            temperature_to_humidity: parse_section(areas[6]),
            humidity_to_location: parse_section(areas[7]),
        }
    }

    fn min_location(farm_data: &FarmData) -> u64 {
        return farm_data.seeds
            .iter()
            .map(|seed| farm_data.seed_to_soil.map_source_to_destination(seed))
            .map(|soil| farm_data.soil_to_fertilizer.map_source_to_destination(&soil))
            .map(|fert| farm_data.fertilizer_to_water.map_source_to_destination(&fert))
            .map(|water| farm_data.water_to_light.map_source_to_destination(&water))
            .map(|light| farm_data.light_to_temperature.map_source_to_destination(&light))
            .map(|temp| farm_data.temperature_to_humidity.map_source_to_destination(&temp))
            .map(|hum| farm_data.humidity_to_location.map_source_to_destination(&hum))
            .min()
            .unwrap();
    }

    fn min_location_by_ranges(farm_data: &FarmData) -> u64 {
        let seed_ranges = farm_data.seed_ranges();

        return seed_ranges
            .iter()
            .flat_map(|seed| farm_data.seed_to_soil.map_source_range_to_destination(seed))
            .flat_map(|soil| farm_data.soil_to_fertilizer.map_source_range_to_destination(&soil))
            .flat_map(|fert| farm_data.fertilizer_to_water.map_source_range_to_destination(&fert))
            .flat_map(|water| farm_data.water_to_light.map_source_range_to_destination(&water))
            .flat_map(|light| farm_data.light_to_temperature.map_source_range_to_destination(&light))
            .flat_map(|temp| farm_data.temperature_to_humidity.map_source_range_to_destination(&temp))
            .flat_map(|hum| farm_data.humidity_to_location.map_source_range_to_destination(&hum))
            .map(|r| r.start)
            .min()
            .unwrap();
    }

    #[test]
    fn sample_p1() {
        let data = SAMPLE;
        let farm_data = parse_farm_data(data);

        let aaa: u64 = min_location(&farm_data);

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn sample_p2() {
        let data = SAMPLE;
        let farm_data = parse_farm_data(data);

        let aaa: u64 = min_location_by_ranges(&farm_data);

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(5, "input.txt");
        let farm_data = parse_farm_data(&data);

        let aaa: u64 = min_location(&farm_data);

        println!("Answer: {aaa:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(5, "input.txt");
        let farm_data = parse_farm_data(&data);

        let aaa: u64 = min_location_by_ranges(&farm_data);

        println!("Answer: {aaa:?}");
    }
}