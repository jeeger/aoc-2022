use std::collections::{HashMap, HashSet, BinaryHeap};

use utils::{lines, Point, PointMappable};

struct Graph {
    nodes: HashSet<Point>,
    edges: HashMap<Point, HashSet<Point>>,
}

impl Graph {
    fn new() -> Self {
        Self { nodes: HashSet::new(), edges: HashMap::new() }
    }
    
    fn add_edge(&mut self, from: &Point, to: Point) {
        if !self.nodes.contains(from) {
            panic!("Trying to add edge to non-existing node");
        }
        self.edges
            .entry(*from)
            .and_modify(|s| {
                s.insert(to);
            });
    }

    fn add_node(&mut self, node: Point) {
        if !self.nodes.contains(&node) {
            self.nodes.insert(node);
            self.edges.insert(node, HashSet::new());
        }
    }

    fn adjacent(&self, p: &Point) -> &HashSet<Point> {
        &self.edges[p]
    }
}

fn height(c: char) -> u32 {
    match c {
        'S' => 0,
        'E' => 26,
        c if c.is_lowercase() => (c as u32) - 97,
        _ => panic!("Unknown height {}", c),
    }
}

fn parse_graph(lines: impl Iterator<Item = String>, is_adjacent: Box<dyn Fn(u32, u32) -> bool>) -> (Point, Point, Graph, impl PointMappable) {
    let mut heights: Vec<Vec<u32>> = Vec::new();
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    for (row, line) in lines.enumerate() {
        let mut cur_row = vec![];
        for (col, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some(Point::new(col.try_into().unwrap(), row.try_into().unwrap()));
            };
            if c == 'E' {
                end = Some(Point::new(col.try_into().unwrap(), row.try_into().unwrap()));
            };
            cur_row.push(height(c));
        };
        heights.push(cur_row);
    };
    let mut graph = Graph::new();
    for p in heights.all_points() {
        graph.add_node(p);
        for adjacent in heights.adjacent_points(&p) {
            graph.add_node(adjacent);
            let p_height = heights.at(&p);
            let adjacent_height = heights.at(&adjacent);
            if is_adjacent(p_height, adjacent_height) {
                graph.add_edge(&p, adjacent);
            };
        };
    };
    (start.unwrap(), end.unwrap(), graph, heights)
}

#[derive(PartialEq, Eq)]
struct PointDist {
    p: Point,
    dist: u32
}

impl PartialOrd for PointDist {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.dist.cmp(&self.dist))
    }
}

impl Ord for PointDist {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

fn shortest_path_length(g: &Graph, start: &Point, end: &HashSet<Point>) -> u32 {
        let mut dist: BinaryHeap<PointDist> = BinaryHeap::from([PointDist {p: *start, dist: 0}]);
    let mut seen = HashSet::new();
    while let Some(entry) = dist.pop() {
        if seen.contains(&entry.p) {
            continue;
        }
        let current_point = entry.p;
        if end.contains(&current_point) {
            return entry.dist;
        }
        for adjacent_point in g.adjacent(&current_point) {
            if !seen.contains(adjacent_point) {
                let adjacent_dist = entry.dist + 1;
                dist.push(PointDist { p: *adjacent_point, dist: adjacent_dist});
            }
            seen.insert(current_point);
        }
    };
    panic!("No path found.");
}

fn solution_1_adjacency(from_height: u32, to_height: u32) -> bool {
    to_height <= from_height + 1
}
   

fn solution1(lines: impl Iterator<Item = String>) -> u32 {
    let (start, end, graph, _heights) = parse_graph(lines, Box::new(solution_1_adjacency));
    let end_set = HashSet::from([end]);
    shortest_path_length(&graph, &start, &end_set)
}

fn solution_2_adjacency(from_height: u32, to_height: u32) -> bool {
    solution_1_adjacency(to_height, from_height)
}

fn solution2(lines: impl Iterator<Item = String>) -> u32 {
    let (_start, end, graph, heights) = parse_graph(lines, Box::new(solution_2_adjacency));
    let possible_end_points: HashSet<Point> = heights.all_points().filter(|p| heights.at(&p) == 0).collect();
    shortest_path_length(&graph, &end, &possible_end_points)
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{solution1, solution2, parse_graph, solution_1_adjacency};
    use utils::{string_lines, Point};

    fn test_iter() -> impl Iterator<Item = String> {
        string_lines(
            r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        )
    }

    #[test]
    fn test_parse_graph() {
        let (start, end, graph, _heights) = parse_graph(test_iter(), Box::new(solution_1_adjacency));
        assert_eq!(start, Point::new(0, 0));
        assert_eq!(end, Point::new(5, 2));
        assert_eq!(*graph.adjacent(&start), HashSet::from([
            Point::new(1, 0),
            Point::new(0, 1)
        ]));
        assert_eq!(*graph.adjacent(&end), HashSet::from([
            Point::new(5, 1),
            Point::new(4, 2),
            Point::new(6, 2),
            Point::new(5, 3)
        ]));
    }

    #[test]
    fn test_solution1() {
        assert_eq!(solution1(test_iter()), 31);
    }

    #[test]
    fn test_solution2() {
        assert_eq!(solution2(test_iter()), 29);
    }
}

fn main() {
    println!("Solution 1: {}", solution1(lines("input.txt")));
    println!("Solution 2: {}", solution2(lines("input.txt")));
}
