use std::{
  cmp::Reverse,
  collections::{HashMap, HashSet},
};

use aoc::prelude::*;

solution! {
  year: 2025,
  day: 8,
  parse,
  part_1: |input| part_1(input, 1000),
  part_2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
  x: u32,
  y: u32,
  z: u32,
}

fn sorted_edges(points: &[Point]) -> Vec<(Point, Point)> {
  let mut edges = Vec::with_capacity((points.len() * (points.len() + 1)) / 2);
  for (i, &p1) in points.iter().enumerate() {
    for &p2 in points.iter().skip(i + 1) {
      edges.push((p1, p2));
    }
  }

  edges.sort_by_cached_key(|(p1, p2)| p1.distance_squared(p2));
  edges
}

struct JunctionGraph {
  networks: Vec<HashSet<Point>>,
  nulled_networks: usize,
  memberships: HashMap<Point, usize>,
}

impl JunctionGraph {
  fn new() -> Self {
    Self {
      networks: vec![],
      nulled_networks: 0,
      memberships: HashMap::new(),
    }
  }

  fn total_points(&self) -> usize {
    self.memberships.len()
  }

  fn total_networks(&self) -> usize {
    self.networks.len() - self.nulled_networks
  }

  fn add_connection(&mut self, p1: Point, p2: Point) {
    match (self.memberships.get(&p1), self.memberships.get(&p2)) {
      (Some(&net1), Some(&net2)) => {
        if net1 != net2 {
          let net2_members = std::mem::take(&mut self.networks[net2]);
          for net2_member in &net2_members {
            self.memberships.insert(*net2_member, net1);
          }
          self.networks[net1].extend(net2_members);
          self.nulled_networks += 1;
        }
      }
      (Some(&net), None) => {
        self.memberships.insert(p2, net);
        self.networks[net].insert(p2);
      }
      (None, Some(&net)) => {
        self.memberships.insert(p1, net);
        self.networks[net].insert(p1);
      }
      (None, None) => {
        self.memberships.insert(p1, self.networks.len());
        self.memberships.insert(p2, self.networks.len());
        self.networks.push(HashSet::from([p1, p2]));
      }
    }
  }
}

impl Point {
  fn new(x: u32, y: u32, z: u32) -> Self {
    Self { x, y, z }
  }

  fn distance_squared(&self, other: &Point) -> u64 {
    (self.x.abs_diff(other.x) as u64).pow(2)
      + (self.y.abs_diff(other.y) as u64).pow(2)
      + (self.z.abs_diff(other.z) as u64).pow(2)
  }
}

fn parse(input: &str) -> Vec<Point> {
  use parse::*;

  input.parse_lines(
    separated_list1(tag(","), u32).map(|ns| match ns.as_slice() {
      [x, y, z] => Point::new(*x, *y, *z),
      _ => panic!("should be three values per row"),
    }),
  )
}

fn part_1(points: Vec<Point>, connections: usize) -> usize {
  let mut graph = JunctionGraph::new();
  for &(p1, p2) in sorted_edges(&points).iter().take(connections) {
    graph.add_connection(p1, p2);
  }

  let mut networks = graph.networks;
  networks.sort_by_key(|net| Reverse(net.len()));
  networks.into_iter().take(3).map(|net| net.len()).product()
}

fn part_2(points: Vec<Point>) -> u32 {
  let mut graph = JunctionGraph::new();
  for (p1, p2) in sorted_edges(&points) {
    graph.add_connection(p1, p2);
    if graph.total_points() == points.len() && graph.total_networks() == 1 {
      return p1.x * p2.x;
    }
  }

  unreachable!("should have found full network before exhausting edges")
}

#[cfg(test)]
mod tests {
  use super::*;
  use indoc::indoc;

  const SAMPLE_INPUT: &str = indoc! {"
    162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689
  "};

  #[test]
  fn test_part1() {
    assert_eq!(part_1(parse(SAMPLE_INPUT), 10), 40);
  }

  #[test]
  fn test_part2() {
    assert_eq!(part_2(parse(SAMPLE_INPUT)), 25272);
  }
}
