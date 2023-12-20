use crate::util::AdventHelper;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::Debug;

use crate::day20::Module::{Broadcaster, Conjunction, FlipFlop};
use crate::day20::Signal::{High, Low};
use rustc_hash::FxHashMap;

pub fn main() {
    let advent = AdventHelper::from_file_name(file!());
    let mut system: MachineSystem = parse_system(&advent.parse_from_strings());

    advent.part1("Pulse product: {}", system.push_button(1000, false));
    advent.part2("Pulse product: {}", system.push_button(10000000000, true))
}

fn parse_system(input: &Vec<String>) -> MachineSystem {
    let mut modules = FxHashMap::default();
    let mut destinations = FxHashMap::default();
    for line in input {
        let (raw_name, raw_destinations) = line.split(" -> ").collect_tuple().unwrap();
        let indicator = raw_name.chars().next().unwrap();
        let name = raw_name.replace(['%', '&'], "");
        let ds = raw_destinations
            .split(", ")
            .map(|d| d.to_string())
            .collect_vec();
        let module = match indicator {
            '%' => FlipFlop { is_on: false },
            '&' => Conjunction {
                latest_signals: FxHashMap::default(),
            },
            'b' => Broadcaster,
            _ => unreachable!(),
        };
        modules.insert(name.clone(), module);
        destinations.insert(name, ds);
    }
    for (name, module) in modules.iter_mut() {
        if let Conjunction {
            latest_signals: signals,
        } = module
        {
            for (input, ds) in &destinations {
                if ds.contains(name) {
                    signals.insert(input.clone(), Low);
                }
            }
        }
    }
    MachineSystem {
        modules,
        destinations,
    }
}

struct MachineSystem {
    modules: FxHashMap<String, Module>,
    destinations: FxHashMap<String, Vec<String>>,
}

impl MachineSystem {
    pub(crate) fn push_button(&mut self, times: usize, exit_on_rx: bool) -> usize {
        let mut low_count = 0;
        let mut high_count = 0;

        for button_presses in 1..=times {
            if button_presses % 1000000 == 0 {
                println!("{button_presses}");
            }
            let mut pulses =
                VecDeque::from([("button".to_string(), Low, "broadcaster".to_string())]);
            while let Some((src, signal, cur)) = pulses.pop_front() {
                if signal == Low && exit_on_rx && cur == "rx" {
                    return button_presses;
                }

                if signal == Low {
                    low_count += 1
                } else {
                    high_count += 1
                }

                let Some(current) = self.modules.get_mut(&cur) else {
                    continue;
                };
                // println!("{src} -{:?}-> {cur}", signal);
                // println!("{current:?}");

                let new_signal = match current {
                    Conjunction {
                        ref mut latest_signals,
                    } => {
                        latest_signals.insert(src, signal);
                        if latest_signals.values().all(|x| *x == High) {
                            Some(Low)
                        } else {
                            Some(High)
                        }
                    }
                    FlipFlop { is_on } => match signal {
                        High => None,
                        Low => {
                            *is_on = !*is_on;
                            match is_on {
                                true => Some(High),
                                false => Some(Low),
                            }
                        }
                    },
                    Broadcaster => Some(signal),
                };
                if let Some(new_signal) = new_signal {
                    for dst in &self.destinations[&cur] {
                        pulses.push_back((cur.clone(), new_signal, dst.to_string()))
                    }
                };
            }
        }
        println!("{low_count} {high_count}");
        low_count * high_count
    }
}

#[derive(Debug)]
enum Module {
    Conjunction {
        latest_signals: FxHashMap<String, Signal>,
    },
    FlipFlop {
        is_on: bool,
    },
    Broadcaster,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Signal {
    High,
    Low,
}
