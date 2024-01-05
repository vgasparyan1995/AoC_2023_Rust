use std::{
    collections::{HashMap, VecDeque},
    io::{stdin, Lines, StdinLock},
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

#[derive(Debug)]
enum GateState {
    Broadcast,
    FlipFlop {
        state: Pulse, // LOW == off, HIGH == on
    },
    Conjunction {
        src_states: HashMap<GateId, Pulse>,
    },
}

#[derive(Debug)]
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

fn broadcast_low(mut gates: HashMap<GateId, Gate>) -> (HashMap<GateId, Gate>, i64, i64) {
    let mut signals = VecDeque::new();
    signals.push_back(Signal {
        sender: GateId::from("button"),
        receiver: GateId::from("broadcaster"),
        pulse: Pulse::LOW,
    });
    let mut lows = 0;
    let mut highs = 0;
    while let Some(signal) = signals.pop_front() {
        match signal.pulse {
            Pulse::LOW => lows += 1,
            Pulse::HIGH => highs += 1,
        };
        if let Some(gate) = gates.get_mut(&signal.receiver) {
            gate.receive(signal)
                .into_iter()
                .for_each(|signal| signals.push_back(signal));
        }
    }
    (gates, lows, highs)
}

fn part1(input: Input) -> i64 {
    let (_, lows, highs) = (0..1000).fold((input.gates, 0, 0), |(gates, lows, highs), _| {
        let (new_gates, new_lows, new_highs) = broadcast_low(gates);
        (new_gates, lows + new_lows, highs + new_highs)
    });
    lows * highs
}

fn main() {
    let input = Input::from(stdin().lines());
    println!("{}", part1(input));
}
