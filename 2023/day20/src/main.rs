use num::integer::lcm;
use std::collections::VecDeque;
use std::fs;

#[derive(Clone, Debug)]
struct FlipFlop {
    name: String,
    state: bool,
    outputs: Vec<String>,
}

#[derive(Clone, Debug)]
struct Conjuction {
    name: String,
    inputs: Vec<(String, bool)>,
    outputs: Vec<String>,
}

#[derive(Clone, Debug)]
struct Element {
    name: String,
    outputs: Vec<String>,
}

#[derive(Clone, Debug)]
enum CircuitElement {
    FlipFlop(FlipFlop),
    Conjuction(Conjuction),
    Element(Element),
}

fn parse_input(input: String) -> Vec<CircuitElement> {
    let mut result = input
        .lines()
        .map(|line| match &line[0..1] {
            "%" => {
                let (name, outputs) = parse_name_and_outputs(&line[1..]);
                CircuitElement::FlipFlop(FlipFlop {
                    name,
                    state: false,
                    outputs,
                })
            }
            "&" => {
                let (name, outputs) = parse_name_and_outputs(&line[1..]);
                CircuitElement::Conjuction(Conjuction {
                    name,
                    inputs: vec![],
                    outputs,
                })
            }
            _ => {
                let (name, outputs) = parse_name_and_outputs(&line);
                CircuitElement::Element(Element { name, outputs })
            }
        })
        .collect();

    populate_conjuction_inputs(&mut result);

    result
}

fn parse_name_and_outputs(line: &str) -> (String, Vec<String>) {
    let (input, outputs_raw) = line.split_once(" -> ").unwrap();
    let outputs = outputs_raw
        .split(", ")
        .map(|e| e.to_string())
        .collect::<Vec<String>>();
    (input.to_string(), outputs)
}

fn populate_conjuction_inputs(circuit: &mut Vec<CircuitElement>) {
    let circuit_clone = circuit.clone();
    let mut conjuctions = circuit
        .iter_mut()
        .filter(|e| match e {
            CircuitElement::Conjuction(_) => true,
            _ => false,
        })
        .collect::<Vec<&mut CircuitElement>>();

    while !conjuctions.is_empty() {
        let conjuction = conjuctions.pop().unwrap();
        let conjuction = match conjuction {
            CircuitElement::Conjuction(c) => c,
            _ => panic!("This should not happen"),
        };

        let inputs = circuit_clone
            .iter()
            .filter(|e| match e {
                CircuitElement::FlipFlop(e) => e.outputs.contains(&conjuction.name),
                CircuitElement::Conjuction(e) => e.outputs.contains(&conjuction.name),
                CircuitElement::Element(e) => e.outputs.contains(&conjuction.name),
            })
            .map(|e| match e {
                CircuitElement::FlipFlop(e) => (e.name.clone(), false),
                CircuitElement::Conjuction(e) => (e.name.clone(), false),
                CircuitElement::Element(e) => (e.name.clone(), false),
            })
            .collect::<Vec<(String, bool)>>();

        conjuction.inputs = inputs;
    }
}

fn send_signal(circuit: &mut Vec<CircuitElement>) -> (usize, usize) {
    let mut low_signals = 1;
    let mut high_signals = 0;

    let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();

    queue.push_back(("broadcaster".to_string(), "button".to_string(), false));

    while !queue.is_empty() {
        let (name, from, signal) = queue.pop_front().unwrap();

        if name == "output" {
            continue;
        }

        let element = find_elem(circuit, name.clone());

        match element {
            Some(e) => match e {
                CircuitElement::Element(e) => e.outputs.iter().for_each(|o| {
                    queue.push_back((o.clone(), e.name.clone(), signal));

                    if signal {
                        high_signals += 1;
                    } else {
                        low_signals += 1;
                    }
                }),
                CircuitElement::FlipFlop(e) => {
                    if !signal {
                        (*e).state = !e.state;
                        let new_signal = e.state;
                        e.outputs.iter().for_each(|o| {
                            queue.push_back((o.clone(), e.name.clone(), new_signal));

                            if new_signal {
                                high_signals += 1;
                            } else {
                                low_signals += 1;
                            }
                        });
                    }
                }
                CircuitElement::Conjuction(e) => {
                    (*e).inputs = e
                        .inputs
                        .iter()
                        .map(|(name, s)| {
                            if *name == from {
                                (name.clone(), signal)
                            } else {
                                (name.clone(), *s)
                            }
                        })
                        .collect();

                    let new_signal = !e.inputs.iter().all(|(_, s)| *s);
                    e.outputs.iter().for_each(|o| {
                        queue.push_back((o.clone(), e.name.clone(), new_signal));
                        if new_signal {
                            high_signals += 1;
                        } else {
                            low_signals += 1;
                        }
                    });
                }
            },
            None => {}
        };
    }

    (low_signals, high_signals)
}

fn send_signal_and_return_sent_pulse(
    circuit: &mut Vec<CircuitElement>,
    inspected_node: String,
) -> bool {
    let mut result = false;

    let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();

    queue.push_back(("broadcaster".to_string(), "button".to_string(), false));

    while !queue.is_empty() {
        let (name, from, signal) = queue.pop_front().unwrap();

        if name == "output" {
            continue;
        }

        let element = find_elem(circuit, name.clone());

        match element {
            Some(e) => match e {
                CircuitElement::Element(e) => e.outputs.iter().for_each(|o| {
                    queue.push_back((o.clone(), e.name.clone(), signal));
                }),
                CircuitElement::FlipFlop(e) => {
                    if !signal {
                        (*e).state = !e.state;
                        let new_signal = e.state;
                        e.outputs.iter().for_each(|o| {
                            queue.push_back((o.clone(), e.name.clone(), new_signal));
                        });
                    }
                }
                CircuitElement::Conjuction(e) => {
                    (*e).inputs = e
                        .inputs
                        .iter()
                        .map(|(name, s)| {
                            if *name == from {
                                (name.clone(), signal)
                            } else {
                                (name.clone(), *s)
                            }
                        })
                        .collect();

                    let new_signal = !e.inputs.iter().all(|(_, s)| *s);
                    e.outputs.iter().for_each(|o| {
                        if inspected_node == e.name.clone() && !result {
                            result = new_signal;
                        }

                        queue.push_back((o.clone(), e.name.clone(), new_signal));
                    });
                }
            },
            None => {}
        };
    }

    result
}

fn find_elem<'circuit>(
    circuit: &'circuit mut Vec<CircuitElement>,
    name: String,
) -> Option<&'circuit mut CircuitElement> {
    circuit.iter_mut().find(|e| match e {
        CircuitElement::FlipFlop(e) => e.name == name,
        CircuitElement::Conjuction(e) => e.name == name,
        CircuitElement::Element(e) => e.name == name,
    })
}

fn find_elem_with_child<'circuit>(
    circuit: &'circuit mut Vec<CircuitElement>,
    child_name: String,
) -> Option<&'circuit mut CircuitElement> {
    circuit.iter_mut().find(|e| match e {
        CircuitElement::FlipFlop(e) => e.outputs.contains(&child_name),
        CircuitElement::Conjuction(e) => e.outputs.contains(&child_name),
        CircuitElement::Element(e) => e.outputs.contains(&child_name),
    })
}

fn find_elems_with_child<'circuit>(
    circuit: &'circuit mut Vec<CircuitElement>,
    child_name: String,
) -> Vec<&'circuit mut CircuitElement> {
    circuit
        .iter_mut()
        .filter(|e| match e {
            CircuitElement::FlipFlop(e) => e.outputs.contains(&child_name),
            CircuitElement::Conjuction(e) => e.outputs.contains(&child_name),
            CircuitElement::Element(e) => e.outputs.contains(&child_name),
        })
        .collect::<_>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input");
    let mut circuit = parse_input(input);
    let circuit_base = circuit.clone();

    let val = (0..1000)
        .map(|_| send_signal(&mut circuit))
        .reduce(|(l1, h1), (l2, h2)| (l1 + l2, h1 + h2))
        .unwrap();

    println!("{:?}", val.0 * val.1);

    if let Some(_) = find_elem_with_child(&mut circuit, "rx".to_string()) {
        let last_conjunction = find_elem_with_child(&mut circuit, "rx".to_string()).unwrap();
        let important_outputs = match (*last_conjunction).clone() {
            CircuitElement::Element(e) => find_elems_with_child(&mut circuit, e.name),
            CircuitElement::FlipFlop(e) => find_elems_with_child(&mut circuit, e.name),
            CircuitElement::Conjuction(e) => find_elems_with_child(&mut circuit, e.name),
        };

        let output_names = important_outputs
            .iter()
            .map(|e| match e {
                CircuitElement::Element(e) => e.name.clone(),
                CircuitElement::FlipFlop(e) => e.name.clone(),
                CircuitElement::Conjuction(e) => e.name.clone(),
            })
            .collect::<Vec<String>>();

        let result = output_names
            .iter()
            .map(|name| {
                let mut c = circuit_base.clone();
                let mut result: usize = 0;

                loop {
                    result = result + 1;
                    if send_signal_and_return_sent_pulse(&mut c, name.clone()) {
                        break;
                    }
                }

                result
            })
            .reduce(|a, b| lcm(a, b))
            .unwrap();

        println!("{:?}", result);
    }
}
