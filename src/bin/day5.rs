use std::fs::read_to_string;

#[derive(Debug)]
struct DstSrcStep {
    destination: i64,
    source: i64,
    steps: i64,
}

impl DstSrcStep {
    fn new(dst: i64, src: i64, step: i64) -> Self {
        Self { destination: dst, source: src, steps: step}
    }

    fn is_in_range(&self, src: i64) -> Option<i64> {
        if (self.source..self.source+self.steps).contains(&src){
            let a = src - self.source;
            return Some(self.destination + a)
        }
        None
    }
}

#[derive(Default)]
#[derive(Debug)]
struct ConvertionMap {
    map: Vec<DstSrcStep>,
}

impl ConvertionMap {
    fn new(nums: Vec<i64>) -> Self {
        Self { map: vec![DstSrcStep::new(nums[0], nums[1], nums[2])] }
    }

    fn append_info(&mut self, nums: Vec<i64>) {
        self.map.push(DstSrcStep { destination: nums[0], source: nums[1], steps: nums[2] });
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn convert_location(&mut self, src: i64) -> i64 {
        let mut location = src;
        for range in &self.map {
            if let Some(dst) = range.is_in_range(src) {
                location = dst;
                break;
            }
        }
        location
    }
}

fn main() {
    // Read todays input
    let data = read_to_string("input/day5.txt").unwrap();
    let lines: Vec<String> = data.split('\n').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();

    // Part 1
    let mut seed_to_plant: Vec<i64> = vec![];  //First line
    let mut convertion_maps: Vec<ConvertionMap> = vec![];  // All maps

    // Get seeds_to_plant from first line 

    // Get all maps
    let mut current_map: Option<ConvertionMap> = None;
    for (line_no, line) in lines.iter().enumerate() {
        //println!("{}: {}", line_no, line);
        if line_no == 0 {
            let nums: Vec<i64> = line.split_whitespace().skip(1).filter_map(|s| s.parse().ok()).collect();
            seed_to_plant.extend(nums);
        } else if line.ends_with("map:") {
            // New map found. Save the previous one.
            if let Some(map) = current_map {
                convertion_maps.push(map);
                current_map = None;
            }
        } else {
            let nums: Vec<i64> = line.split_whitespace().filter_map(|s|s.parse().ok()).collect();
            if let Some(ref mut map) = current_map {
                map.append_info(nums);
            } else {
                current_map = Some(ConvertionMap::new(nums));
            }
        }
    }
    
    // Save last map
    if let Some(map) = current_map {
        convertion_maps.push(map);
    }

    let mut locations: Vec<i64> = vec![];
    for seed in seed_to_plant {
        let mut location = seed;
        for map in &mut convertion_maps {
            location = map.convert_location(location);
        }
        locations.push(location);
    }

    println!("Lowest location: {}", locations.iter().min().unwrap());

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_placeholder() {
    }
}
