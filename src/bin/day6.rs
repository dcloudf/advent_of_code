use std::fs::read_to_string;

fn main() {
    let file = read_to_string("./data/day6.txt").unwrap();
    println!("{}", parse_race_info(&file).iter().map(|info| info.number_ways_to_beat()).reduce(|x, y| x * y).unwrap());
    println!("{}", parse_race_info_with_bad_kerning(&file).number_ways_to_beat());
}

#[derive(Debug, PartialEq)]
struct RaceInfo {
    time: u64,
    distance: u64
}

impl RaceInfo {
    fn number_ways_to_beat(&self) -> u64 {
        (0..self.time + 1).filter(|delay| (self.time - delay) * (delay) > self.distance).count() as u64
    }
}

fn parse_race_info(lines: &str) -> Vec<RaceInfo> {
    let mut lines_iter = lines.lines();
    let line_time = lines_iter
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace();
    let line_distance = lines_iter.next().unwrap().split(':').nth(1).unwrap().split_whitespace();
    line_time.zip(line_distance).map(|(t, d)| RaceInfo{time: t.parse::<u64>().unwrap(), distance: d.parse::<u64>().unwrap()}).collect::<Vec<RaceInfo>>()
}

fn parse_race_info_with_bad_kerning(lines: &str) -> RaceInfo {
    let mut lines_iter = lines.lines();
    let time = lines_iter.next().unwrap().split(':').nth(1).unwrap().split_whitespace().map(|x| x.to_string()).reduce(|x, y| x + &y).unwrap().parse::<u64>().unwrap();
    let distance = lines_iter.next().unwrap().split(':').nth(1).unwrap().split_whitespace().map(|x| x.to_string()).reduce(|x, y| x + &y).unwrap().parse::<u64>().unwrap();
    RaceInfo{time, distance}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_race_info() {
        assert_eq!(parse_race_info("Time:      7  15   30\nDistance:  9  40  200"), vec![RaceInfo{time: 7, distance: 9}, RaceInfo{time: 15, distance: 40}, RaceInfo{time: 30, distance: 200}])
    }

    #[test]
    fn test_parse_race_info_with_bad_kerning() {
        assert_eq!(parse_race_info_with_bad_kerning("Time:      7  15   30\nDistance:  9  40  200"), RaceInfo{time: 71530, distance: 940200})
    }

    #[test]
    fn test_number_ways_to_beat() {
        assert_eq!(RaceInfo{time: 7, distance: 9}.number_ways_to_beat(), 4);
        assert_eq!(RaceInfo{time: 15, distance: 40}.number_ways_to_beat(), 8);
        assert_eq!(RaceInfo{time: 30, distance: 200}.number_ways_to_beat(), 9);
        assert_eq!(RaceInfo{time: 71530, distance: 940200}.number_ways_to_beat(), 71503);
    }
}