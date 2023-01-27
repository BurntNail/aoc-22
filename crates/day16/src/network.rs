use std::collections::HashMap;

use crate::parse::{Valve, Int};

pub fn get_all_connections_from<'a> (start: &'a String, exclude: &[&String], map: impl Iterator<Item = (&'a String, &'a Valve)>) -> Vec<String> {
    map.filter_map(|(name, valve)| {
        if valve.contains(start) && !exclude.contains(&name) {
            Some(name.to_string())
        } else {
            None
        }
    }).collect()
}

pub fn all_combos (start: String, map: HashMap<String, Valve>, max_steps_left: Int) -> Vec<Vec<String>> {
    if max_steps_left <= 0 {
        return vec![];
    }

    let mut paths: Vec<Vec<String>> = vec![];

    for conn in get_all_connections_from(&start, &[&start], map.iter()) {
        println!("Checking from {conn}");
        let connections = all_combos(conn.clone(), map.clone(), max_steps_left - 1);
        for mut interior_path in connections {
            interior_path.insert(0, conn.clone());
            paths.push(interior_path);
        }
    }

    paths
}

pub fn get_best (paths: Vec<Vec<String>>, map: HashMap<String, Int>) -> Int {


    todo!()
}