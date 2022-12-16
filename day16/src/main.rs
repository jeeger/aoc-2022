extern crate nom;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, u32},
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use std::{
    cmp::max,
    collections::{BTreeSet, HashMap, HashSet},
    fs::read_to_string,
};

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct Edge {
    from: String,
    rate: u32,
    to: String,
}

impl Edge {
    fn new(from: &str, rate: u32, to: &str) -> Self {
        Self {
            from: from.to_owned(),
            rate,
            to: to.to_owned(),
        }
    }
}

fn parse_edge(input: &str) -> IResult<&str, HashSet<Edge>> {
    let tunnel_str = tuple((
        tag("; tunnel"),
        opt(tag("s")),
        tag(" lead"),
        opt(tag("s")),
        tag(" to valve"),
        opt(tag("s")),
        tag(" "),
    ));
    map(
        tuple((
            preceded(tag("Valve "), alpha1),
            preceded(tag(" has flow rate="), u32),
            preceded(tunnel_str, separated_list1(tag(", "), alpha1)),
        )),
        |(from, rate, to)| {
            to.iter()
                .map(|to| Edge::new(from, rate, to))
                .collect::<HashSet<Edge>>()
        },
    )(input)
}

type PressureMap<'a> = HashMap<&'a str, u32>;
type DistanceMap<'a> = HashMap<(&'a str, &'a str), u32>;

fn parse_input(input: &str) -> IResult<&str, HashSet<Edge>> {
    let parsed = separated_list1(newline, parse_edge)(input);
    parsed.map(|(rest, result)| {
        (
            rest,
            result.into_iter().flatten().collect::<HashSet<Edge>>(),
        )
    })
}

fn calculate_distances<'a>(es: &'a HashSet<Edge>) -> HashMap<(&'a str, &'a str), u32> {
    let mut result: HashMap<(&str, &str), u32> = HashMap::new();
    let v: HashSet<&str> = es
        .iter()
        .flat_map(|e| [e.from.as_str(), e.to.as_str()])
        .collect();
    for e in es {
        result.insert((&e.from, &e.to), 1);
    }
    for e in es {
        result.insert((&e.from, &e.from), 0);
        result.insert((&e.to, &e.to), 0);
    }
    for k in v.iter() {
        for i in v.iter() {
            if i == k {
                continue;
            }
            for j in v.iter() {
                if i == j {
                    continue;
                }
                let dist_i_j = *result.get(&(i, j)).unwrap_or(&u32::MAX);
                let dist_i_k = *result.get(&(i, k)).unwrap_or(&u32::MAX);
                let dist_k_j = *result.get(&(k, j)).unwrap_or(&u32::MAX);
                if dist_i_k.saturating_add(dist_k_j) < dist_i_j {
                    result.insert((i, j), dist_i_k.saturating_add(dist_k_j));
                }
            }
        }
    }
    result
}

fn distances_and_pressures(e: &HashSet<Edge>) -> (HashMap<(&str, &str), u32>, HashMap<&str, u32>) {
    (
        calculate_distances(e),
        e.iter()
            .map(|e| (e.from.as_str(), e.rate))
            .filter(|(_, rate)| rate > &0)
            .collect(),
    )
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct CacheEntry<'a> {
    pos: String,
    remaining_time: u32,
    opened: BTreeSet<&'a str>,
}

struct MaxPressureCalculator<'a> {
    nodes: HashSet<String>,
    pressures: PressureMap<'a>,
    distances: DistanceMap<'a>,
}

impl<'a> MaxPressureCalculator<'a> {
    fn new(pressures: PressureMap<'a>, distances: DistanceMap<'a>) -> Self {
        Self {
            nodes: pressures.keys().map(|k| k.to_string()).collect(),
            pressures,
            distances,
        }
    }

    fn get_maximum_released_pressure_from(
        &'a self,
        pos: &'a str,
        remaining_time: u32,
        opened: &BTreeSet<&'a str>,
        cache: &mut HashMap<CacheEntry<'a>, u32>,
    ) -> u32 {
        let entry = CacheEntry {
            pos: pos.to_string(),
            remaining_time,
            opened: opened.clone(),
        };
        if cache.contains_key(&entry) {
            return cache[&entry];
        }
        let mut highest_pressure = 0;
        if remaining_time > 1 {
            let mut next_opened = opened.clone();
            next_opened.insert(pos);
            for next in &self.nodes {
                if next == pos {
                    continue;
                }
                let distance_to_next = self.distances[&(pos, next.as_str())];
                // Recursive calculation without opening the current valve
                if remaining_time > distance_to_next {
                    highest_pressure = max(
                        highest_pressure,
                        self.get_maximum_released_pressure_from(
                            next,
                            remaining_time - distance_to_next,
                            &opened,
                            cache,
                        ),
                    );
                } 
                if !opened.contains(pos) && remaining_time > distance_to_next && self.pressures.get(pos).unwrap_or(&0) > &0 {
                    let pressure_with_opened_valve = self.get_maximum_released_pressure_from(
                        next,
                        remaining_time - distance_to_next - 1,
                        &next_opened,
                        cache,
                    );
                    highest_pressure = max(
                        highest_pressure,
                        pressure_with_opened_valve + (remaining_time - 1) * self.pressures[pos],
                    );
                }
                if !opened.contains(pos) {
                highest_pressure = max(
                    highest_pressure,
                    self.pressures.get(pos).unwrap_or(&0) * (remaining_time - 1));
                }
            }
        }
        cache.insert(entry, highest_pressure);
        return highest_pressure;
    }

    fn get_maximum_released_pressure(&'a mut self, start: &'a str, time: u32) -> u32 {
        let mut cache = HashMap::new();
        self.get_maximum_released_pressure_from(start, time, &BTreeSet::new(), &mut cache)
    }
}

fn solution1(input: &str) -> u32 {
    let parsed = parse_input(input).unwrap().1;
    let (distances, pressures) = distances_and_pressures(&parsed);
    let mut calculator = MaxPressureCalculator::new(pressures, distances);
    calculator.get_maximum_released_pressure("AA", 30)
}

fn solution2(_input: &str) -> isize {
    0
}

#[cfg(test)]
mod test {
    use crate::{
        distances_and_pressures, parse_edge, parse_input, solution1, solution2, Edge, MaxPressureCalculator,
    };
    use std::collections::HashSet;

    const TEST_STRING: &str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_parse_edge() {
        assert_eq!(
            parse_edge("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB")
                .unwrap()
                .1,
            HashSet::from([
                Edge::new("AA", 0, "DD"),
                Edge::new("AA", 0, "II"),
                Edge::new("AA", 0, "BB")
            ])
        );
        assert_eq!(
            parse_edge("Valve HH has flow rate=0; tunnel leads to valves GG")
                .unwrap()
                .1,
            HashSet::from([Edge::new("HH", 0, "GG")])
        );
        assert_eq!(
            parse_edge("Valve GG has flow rate=0; tunnels lead to valves FF, HH")
                .unwrap()
                .1,
            HashSet::from([Edge::new("GG", 0, "FF"), Edge::new("GG", 0, "HH")])
        );
    }

    #[test]
    fn test_parse_input() {
        let (rest, result) = parse_input(TEST_STRING).unwrap();
        assert_eq!(rest, "");
        assert_eq!(result.len(), 20);
        assert_eq!(
            result,
            HashSet::from([
                Edge::new("AA", 0, "DD"),
                Edge::new("AA", 0, "II"),
                Edge::new("AA", 0, "BB"),
                Edge::new("BB", 13, "CC"),
                Edge::new("BB", 13, "AA"),
                Edge::new("CC", 2, "DD"),
                Edge::new("CC", 2, "BB"),
                Edge::new("DD", 20, "CC"),
                Edge::new("DD", 20, "AA"),
                Edge::new("DD", 20, "EE"),
                Edge::new("EE", 3, "FF"),
                Edge::new("EE", 3, "DD"),
                Edge::new("FF", 0, "EE"),
                Edge::new("FF", 0, "GG"),
                Edge::new("GG", 0, "FF"),
                Edge::new("GG", 0, "HH"),
                Edge::new("HH", 22, "GG"),
                Edge::new("II", 0, "AA"),
                Edge::new("II", 0, "JJ"),
                Edge::new("JJ", 21, "II")
            ])
        );
    }

    #[test]
    fn test_dynamic_programming() {
        let parsed = parse_input(TEST_STRING).unwrap().1;
        let (distances, pressures) = distances_and_pressures(&parsed);
        let mut calculator = MaxPressureCalculator::new(pressures, distances);
        assert_eq!(calculator.get_maximum_released_pressure("AA", 30),
                   1651);
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(TEST_STRING), 1651);
    }

    #[ignore]
    #[test]
    fn test_solution2() {
        assert_eq!(solution2(TEST_STRING), 56000011);
    }
}

fn main() {
    println!(
        "Solution 1: {}",
        solution1(&read_to_string("input.txt").unwrap())
    );
    // println!(
    //     "Solution 2: {}",
    //     solution2(&read_to_string("input.txt").unwrap())
    // );
}
