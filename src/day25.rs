use crate::util::AdventHelper;
use itertools::Itertools;
use pathfinding::prelude::{edmonds_karp_dense, strongly_connected_component};
use rustc_hash::{FxHashMap, FxHashSet};

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let adjacencies = parse(advent.parse_from_strings());

    advent.part1(
        "Product of cluster size: {}",
        product_of_components(&adjacencies),
    );
}

fn product_of_components(adjacencies: &FxHashMap<usize, Vec<usize>>) -> usize {
    let nodes = adjacencies.keys().cloned().collect_vec();
    let mut caps: Vec<((usize, usize), i32)> = vec![];
    for (a, bs) in adjacencies {
        for b in bs {
            caps.push(((*a, *b), 1));
        }
    }
    for (source, sink) in nodes.iter().tuple_combinations() {
        let (_, maximum_flow, minimum_cut) = edmonds_karp_dense(&nodes, source, sink, caps.clone());
        if maximum_flow != 3 {
            continue;
        }
        let cut: FxHashSet<(usize, usize)> =
            minimum_cut.iter().map(|((a, b), _)| (*a, *b)).collect();
        let source_component =
            strongly_connected_component(source, |n| succ_without_cut(n, adjacencies, &cut));
        let source_component_size = source_component.len();
        let sink_component_size = nodes.len() - source_component_size;

        return source_component_size * sink_component_size;
    }
    panic!()
}

fn succ_without_cut(
    n: &usize,
    adjacencies: &FxHashMap<usize, Vec<usize>>,
    cut: &FxHashSet<(usize, usize)>,
) -> Vec<usize> {
    adjacencies[n]
        .iter()
        .filter(|t| !cut.contains(&(*n, **t)))
        .cloned()
        .collect_vec()
}

fn parse(input: Vec<String>) -> FxHashMap<usize, Vec<usize>> {
    let mut succ = FxHashMap::default();
    for s in input {
        let (from, raw_to) = s.split(": ").collect_tuple().unwrap();
        for to in raw_to.split(' ') {
            for (a, b) in [(from, to), (to, from)] {
                succ.entry(a.to_string())
                    .and_modify(|v: &mut Vec<String>| v.push(b.to_string()))
                    .or_insert(vec![b.to_string()]);
            }
        }
    }
    let mut to_idx: FxHashMap<String, usize> = succ
        .keys()
        .enumerate()
        .map(|(a, b)| (b.clone(), a))
        .collect();
    succ.iter()
        .map(|(a, bs)| (to_idx[a], bs.iter().map(|b| to_idx[b]).collect_vec()))
        .collect()
}
