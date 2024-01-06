use std::{
    collections::{HashMap, VecDeque},
    io::{stdin, Lines, StdinLock},
    ops::{Add, AddAssign},
    println,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pulse {
    LOW,
    HIGH,
}

fn flip(pulse: Pulse) -> Pulse {
    match pulse {
        Pulse::LOW => Pulse::HIGH,
        Pulse::HIGH => Pulse::LOW,
    }
}

type GateId = String;

#[derive(Debug)]
struct Signal {
    sender: GateId,
    receiver: GateId,
    pulse: Pulse,
}

#[derive(Debug, Clone)]
enum GateState {
    Broadcast,
    FlipFlop {
        state: Pulse, // LOW == off, HIGH == on
    },
    Conjunction {
        src_states: HashMap<GateId, Pulse>,
    },
}

#[derive(Debug, Clone)]
struct Gate {
    state: GateState,
    outs: Vec<GateId>,
}

impl Gate {
    fn receive(&mut self, signal: Signal) -> Vec<Signal> {
        let out_pulse = match &self.state {
            GateState::Broadcast => {
                assert!(signal.pulse == Pulse::LOW);
                Some(Pulse::LOW)
            }
            GateState::FlipFlop { state } => {
                if signal.pulse == Pulse::LOW {
                    let new_state = flip(*state);
                    self.state = GateState::FlipFlop { state: new_state };
                    Some(new_state)
                } else {
                    None
                }
            }
            GateState::Conjunction { src_states } => {
                let mut new_src_states = src_states.clone();
                *new_src_states.get_mut(&signal.sender).unwrap() = signal.pulse;
                let out_pulse = if new_src_states.values().all(|&p| p == Pulse::HIGH) {
                    Pulse::LOW
                } else {
                    Pulse::HIGH
                };
                self.state = GateState::Conjunction {
                    src_states: new_src_states,
                };
                Some(out_pulse)
            }
        };
        match out_pulse {
            Some(out_pulse) => self
                .outs
                .iter()
                .map(|out| Signal {
                    sender: signal.receiver.clone(),
                    receiver: out.clone(),
                    pulse: out_pulse,
                })
                .collect(),
            None => vec![],
        }
    }
}

#[derive(Debug)]
struct Input {
    gates: HashMap<GateId, Gate>,
}

impl From<Lines<StdinLock<'_>>> for Input {
    fn from(lines: Lines<StdinLock<'_>>) -> Self {
        let mut gates = lines
            .map(|line| {
                let line = line.unwrap();
                let (gate_id, outs) = line.split_once(" -> ").unwrap();
                let outs = outs.split(", ").map(String::from).collect();
                if let Some(gate_id) = gate_id.strip_prefix("%") {
                    (
                        gate_id.into(),
                        Gate {
                            state: GateState::FlipFlop { state: Pulse::LOW },
                            outs,
                        },
                    )
                } else if let Some(gate_id) = gate_id.strip_prefix("&") {
                    (
                        gate_id.into(),
                        Gate {
                            state: GateState::Conjunction {
                                src_states: HashMap::new(),
                            },
                            outs,
                        },
                    )
                } else {
                    assert!(gate_id == "broadcaster");
                    (
                        gate_id.into(),
                        Gate {
                            state: GateState::Broadcast,
                            outs,
                        },
                    )
                }
            })
            .fold(HashMap::<GateId, Gate>::new(), |mut gates, (id, gate)| {
                gates.insert(id, gate);
                gates
            });
        let mut sources = HashMap::new();
        for (src, gate) in gates.iter() {
            for out in gate.outs.iter() {
                let gate = gates.get(out);
                if gate.is_none() {
                    continue;
                }
                if let GateState::Conjunction { .. } = gate.unwrap().state {
                    sources
                        .entry(out.clone())
                        .and_modify(|srcs: &mut Vec<GateId>| srcs.push(src.clone()))
                        .or_insert(vec![src.clone()]);
                }
            }
        }
        for (conj, srcs) in sources.into_iter() {
            gates.entry(conj).and_modify(|gate: &mut Gate| {
                gate.state = GateState::Conjunction {
                    src_states: srcs.into_iter().map(|src| (src, Pulse::LOW)).collect(),
                }
            });
        }
        Input { gates }
    }
}

fn broadcast_low<T>(
    mut gates: HashMap<GateId, Gate>,
    signal_reducer: impl Fn(&Signal) -> T,
) -> (HashMap<GateId, Gate>, T)
where
    T: Default + AddAssign,
{
    let mut signals = VecDeque::new();
    signals.push_back(Signal {
        sender: GateId::from("button"),
        receiver: GateId::from("broadcaster"),
        pulse: Pulse::LOW,
    });
    let mut acc = T::default();
    while let Some(signal) = signals.pop_front() {
        acc += signal_reducer(&signal);
        if let Some(gate) = gates.get_mut(&signal.receiver) {
            gate.receive(signal)
                .into_iter()
                .for_each(|signal| signals.push_back(signal));
        }
    }
    (gates, acc)
}

#[derive(Default)]
struct LowHighPulseCount {
    lows: i64,
    highs: i64,
}

impl Add for LowHighPulseCount {
    type Output = LowHighPulseCount;
    fn add(self, rhs: Self) -> Self::Output {
        LowHighPulseCount {
            lows: self.lows + rhs.lows,
            highs: self.highs + rhs.highs,
        }
    }
}

impl AddAssign for LowHighPulseCount {
    fn add_assign(&mut self, rhs: Self) {
        self.lows += rhs.lows;
        self.highs += rhs.highs;
    }
}

fn count_low_high_pulses(signal: &Signal) -> LowHighPulseCount {
    match signal.pulse {
        Pulse::LOW => LowHighPulseCount { lows: 1, highs: 0 },
        Pulse::HIGH => LowHighPulseCount { lows: 0, highs: 1 },
    }
}

fn part1(input: Input) -> i64 {
    let (_, LowHighPulseCount { lows, highs }) = (0..1000).fold(
        (input.gates, LowHighPulseCount::default()),
        |(gates, lows_highs), _| {
            let (new_gates, new_lows_highs) = broadcast_low(gates, count_low_high_pulses);
            (new_gates, lows_highs + new_lows_highs)
        },
    );
    lows * highs
}

fn count_lows_to(dest: &str, signal: &Signal) -> i64 {
    if signal.receiver == dest && signal.pulse == Pulse::LOW {
        1
    } else {
        0
    }
}

fn part2(input: Input) -> i64 {
    // Observing the input it's visible that "rx" is a conj with 4 inputs, each of which is a conj
    // with only 1 input. Finding the cycles yielding 1 to those will allow us to find the cycle
    // yielding 1 to "rx" (the LCM of four).
    vec!["jg", "kv", "mr", "rz"]
        .into_iter()
        .map(|dest| {
            let mut gates = input.gates.clone();
            let mut num_press = 0;
            loop {
                num_press += 1;
                let (new_gates, num_lows_to_rx) =
                    broadcast_low(gates, |signal| count_lows_to(dest, signal));
                if num_lows_to_rx == 1 {
                    return num_press;
                }
                gates = new_gates;
            }
        })
        .reduce(|acc, n| num::integer::lcm(acc, n))
        .unwrap()
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part2(input));
}
