use crate::util::AdventHelper;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::str::FromStr;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let blocks = advent.parse_from_strings();

    advent.part1("Desintegratable blocks: {}", count_desintegratable(&blocks));
    advent.part2("Total falling blocks: {}", count_total_falling(&blocks));
}

type Point3 = (usize, usize, usize);
#[derive(Clone)]
struct Block {
    start: Point3,
    end: Point3,
}

impl Block {
    fn layer(&self, z: usize) -> FxHashSet<Point3> {
        let mut layer = FxHashSet::default();
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                layer.insert((x, y, z));
            }
        }
        layer
    }

    fn drop(&self) -> Option<Self> {
        if self.start.2 == 0 {
            return None;
        }
        Some(Block {
            start: (self.start.0, self.start.1, self.start.2 - 1),
            end: (self.end.0, self.end.1, self.end.2 - 1),
        })
    }
}

impl FromStr for Block {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .split('~')
            .map(|raw| {
                raw.split(',')
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        Ok(Block { start, end })
    }
}

fn count_desintegratable(blocks: &Vec<Block>) -> usize {
    let map = get_desintegratable(blocks);
    map.iter().filter(|(_k, v)| v.is_empty()).count()
}

fn count_total_falling(blocks: &Vec<Block>) -> usize {
    let n = blocks.len();
    let (_, _, blocks) = drop(blocks);

    blocks
        .into_iter()
        .combinations(n - 1)
        .map(|x| drop(&x.into_iter().collect_vec()).0)
        .sum()
}

fn get_desintegratable(blocks: &Vec<Block>) -> FxHashMap<usize, Vec<usize>> {
    let mut blocks = blocks.clone();
    let (_, rests_on, _) = drop(&mut blocks);

    let get_below: FxHashMap<usize, Vec<usize>> = rests_on
        .iter()
        .into_group_map_by(|(above, _below)| above)
        .iter()
        .map(|(k, v)| (**k, v.iter().map(|(_, below)| *below).collect_vec()))
        .collect();
    let get_above: FxHashMap<usize, Vec<usize>> = rests_on
        .iter()
        .into_group_map_by(|(_above, below)| below)
        .iter()
        .map(|(k, v)| (**k, v.iter().map(|(above, _)| *above).collect_vec()))
        .collect();
    let mut unstable_blocks = FxHashMap::default();
    for (i, _) in blocks.iter().enumerate() {
        let blocks_above = get_above.get(&i).unwrap_or(&vec![]).clone();
        let mut unstable_blocks_above = vec![];
        for j in blocks_above {
            if get_below[&j].iter().filter(|b| **b != i).count() == 0 {
                unstable_blocks_above.push(j);
            }
        }
        unstable_blocks.insert(i, unstable_blocks_above);
    }
    unstable_blocks
}

fn drop(blocks: &Vec<Block>) -> (usize, FxHashSet<(usize, usize)>, Vec<Block>) {
    let mut layer_above_map = FxHashMap::default();
    let mut rests_on = FxHashSet::default();
    let mut dropped = FxHashSet::default();
    let mut dropped_blocks = vec![];
    for (i, b) in blocks.iter().enumerate().sorted_by_key(|(_i, b)| b.start.2) {
        let mut b = b.clone();
        let mut has_dropped = false;
        loop {
            for p in b.layer(b.start.2) {
                if let Some(below_brick_idx) = layer_above_map.get(&p) {
                    rests_on.insert((i, *below_brick_idx));
                    has_dropped = true;
                }
            }
            if has_dropped {
                break;
            }
            if let Some(new_b) = b.drop() {
                b = new_b;
                dropped.insert(i);
            } else {
                break;
            }
        }
        let set: FxHashSet<Point3> = b.layer(b.end.2 + 1);
        for p in set {
            layer_above_map.insert(p, i);
        }
        dropped_blocks.push((i, b));
    }
    let dropped_blocks = dropped_blocks
        .into_iter()
        .sorted_by_key(|x| (*x).0)
        .map(|x| x.1)
        .collect_vec();
    (dropped.len(), rests_on, dropped_blocks)
}
