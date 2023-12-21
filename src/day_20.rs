#[cfg(test)]
mod day20 {
    use std::any::Any;
    use std::collections::{HashMap, VecDeque};

    use itertools::Itertools;

    use crate::day_20::day20::PV::{HIGH, LOW};
    use crate::read_data_file;

    static SAMPLE_1: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    static SAMPLE_2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    enum PV {
        HIGH,
        LOW,
    }

    struct Pulse {
        from: String,
        to: String,
        value: PV,
    }

    trait Module {
        fn receive_pulse(&mut self, p: &Pulse) -> Option<PV>;
        fn outputs(&self) -> &Vec<String>;
        fn as_any(&mut self) -> &mut dyn Any;
    }

    struct FlipFlop {
        current_value: PV,
        output: Vec<String>,
    }

    impl Module for FlipFlop {
        fn receive_pulse(&mut self, p: &Pulse) -> Option<PV> {
            match p.value {
                HIGH => None,
                LOW => {
                    self.current_value = if self.current_value == HIGH { LOW } else { HIGH };
                    return Some(self.current_value);
                }
            }
        }

        fn outputs(&self) -> &Vec<String> {
            return &self.output;
        }

        fn as_any(&mut self) -> &mut dyn Any {
            self
        }
    }

    struct Conjunction {
        current_values: HashMap<String, PV>,
        output: Vec<String>,
    }

    impl Module for Conjunction {
        fn receive_pulse(&mut self, p: &Pulse) -> Option<PV> {
            self.current_values.insert(p.from.to_owned(), p.value);

            if self.current_values.values().into_iter().all(|v| *v == HIGH) {
                return Some(LOW);
            }

            return Some(HIGH);
        }

        fn outputs(&self) -> &Vec<String> {
            return &self.output;
        }

        fn as_any(&mut self) -> &mut dyn Any {
            self
        }
    }

    struct Broadcast {
        output: Vec<String>,
    }

    impl Module for Broadcast {
        fn receive_pulse(&mut self, p: &Pulse) -> Option<PV> {
            return Some(p.value);
        }

        fn outputs(&self) -> &Vec<String> {
            return &self.output;
        }

        fn as_any(&mut self) -> &mut dyn Any {
            self
        }
    }

    struct Machine {
        modules: HashMap<String, Box<dyn Module>>,
        counts: HashMap<PV, usize>,
    }

    impl Machine {
        fn push_button(&mut self) -> HashMap<String, usize> {
            let mut queue = VecDeque::new();

            let first_pulse = Pulse {
                from: "Button".to_owned(),
                to: "broadcaster".to_owned(),
                value: LOW,
            };

            self.counts.entry(first_pulse.value).and_modify(|c| *c += 1);
            queue.push_front(first_pulse);


            let mut tripped: HashMap<String, usize> = HashMap::from_iter([
                ("kd".to_owned(), 0usize),
                ("zf".to_owned(), 0usize),
                ("vg".to_owned(), 0usize),
                ("gs".to_owned(), 0usize),
                ("rx".to_owned(), 0usize)
            ]);

            while let Some(pulse) = queue.pop_back() {

                if pulse.value == LOW {
                    tripped.entry(pulse.to.to_owned()).and_modify(|v| *v += 1);
                }

                if let Some(m) = self.modules.get_mut(&pulse.to) {
                    let new_pulses = match m.receive_pulse(&pulse) {
                        Some(pv) => m.outputs().iter().map(|to| Pulse {
                            to: to.to_owned(),
                            from: pulse.to.to_owned(),
                            value: pv,
                        }).collect_vec(),
                        None => vec![]
                    };

                    new_pulses.into_iter().for_each(|p| {
                        self.counts.entry(p.value).and_modify(|c| *c += 1);
                        queue.push_front(p);
                    });
                }
            }

            return tripped;
        }
    }

    fn parse_machine(input: &str) -> Machine {
        let mut modules = input
            .lines()
            .map(|l| {
                let (prefix, suffix) = l.split(" -> ").collect_tuple().unwrap();

                let outputs = suffix.split(",").into_iter().map(|s| s.trim().to_owned()).collect_vec();

                let pair: (String, Box<dyn Module>) = match prefix.chars().next().unwrap() {
                    '%' => {
                        let name = prefix[1..].to_owned();
                        (
                            name,
                            Box::new(FlipFlop {
                                current_value: LOW,
                                output: outputs,
                            })
                        )
                    }
                    '&' => {
                        let name = prefix[1..].to_owned();
                        (
                            name,
                            Box::new(Conjunction {
                                current_values: HashMap::new(),
                                output: outputs,
                            })
                        )
                    }
                    'b' => {
                        let name = prefix.to_owned();
                        (
                            name,
                            Box::new(Broadcast {
                                output: outputs,
                            })
                        )
                    }
                    _ => panic!()
                };

                return pair;
            })
            .collect::<HashMap<String, Box<dyn Module>>>();

        let mut rev_idx: HashMap<String, Vec<String>> = HashMap::new();

        for (name, m) in &modules {
            for o in m.outputs() {
                rev_idx.entry(o.to_owned()).and_modify(|l| l.push(name.to_owned())).or_insert(vec![name.to_owned()]);
            }
        }

        for (name, m) in &mut modules {
            if let Some(conj) = m.as_any().downcast_mut::<Conjunction>() {
                let talks_to_me = rev_idx.get(name).unwrap();
                for from in talks_to_me {
                    conj.current_values.entry(from.to_owned()).or_insert(LOW);
                }
            }
        }

        return Machine {
            counts: HashMap::from_iter([(HIGH, 0), (LOW, 0)]),
            modules,
        };
    }

    fn apply_p1(input: &str) -> usize {
        let mut machine = parse_machine(input);

        for _ in 0..1000 {
            machine.push_button();
        }

        machine.counts.get(&HIGH).unwrap() * machine.counts.get(&LOW).unwrap()
    }

    fn apply_p2(input: &str) -> usize {
        let mut machine = parse_machine(input);

        let mut i = 0usize;

        loop {
            let tripped = machine.push_button();
            i = i + 1;

            for (port, cnt) in &tripped {
                if *cnt > 0 {
                    println!("Port: {port} fired: {cnt} times on push: {i}");
                }
            }


            if *tripped.get("rx").unwrap() > 0 {
                return i;
            }

            if i == 10000 {
                panic!("too long");
            }
        }
    }

    #[test]
    fn sample_1_p1() {
        let data = SAMPLE_1;


        let results = apply_p1(data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_2_p1() {
        let data = SAMPLE_2;


        let results = apply_p1(data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(20, "input.txt");


        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(20, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}