use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

        for (bus, route) in routes.iter().enumerate() {
            for &stop in route {
                graph.entry(stop).or_default().push(bus as i32);
            }
        }

        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
        queue.push_back((source, 0));

        let mut visited_stops = HashSet::new();
        visited_stops.insert(source);

        let mut visited_buses = HashSet::new();

        while let Some((stop, moves)) = queue.pop_front() {
            if stop == target {
                return moves;
            }

            if let Some(buses) = graph.get(&stop) {
                for &bus in buses {
                    if visited_buses.contains(&bus) {
                        continue;
                    }

                    visited_buses.insert(bus);

                    for &next_stop in &routes[bus as usize] {
                        if visited_stops.contains(&next_stop) {
                            continue;
                        }
                        visited_stops.insert(next_stop);
                        queue.push_back((next_stop, moves + 1));
                    }
                }
            }
        }

        -1
    }
}

fn main() {
    println!("Hello, world!");
}
