#[cfg(test)]
mod day25 {
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use fdg_img::Settings;
    use fdg_img::style::{BLACK, Color, IntoFont, TextStyle};
    use fdg_img::style::text_anchor::{HPos, Pos, VPos};
    use fdg_sim::{ForceGraph, ForceGraphHelper, Simulation, SimulationParameters};

    use itertools::{assert_equal, Itertools};

    use crate::read_data_file;

    static SAMPLE_1: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;

    fn parse_lines(input: &str) -> (HashSet<String>, HashSet<(String, String)>) {
        let mut all_components = HashSet::new();
        let mut all_wires = HashSet::new();

        for l in input.lines() {
            let (head, tail_str) = l.split(":").collect_tuple().unwrap();

            all_components.insert(head.to_owned());

            for c in tail_str.split(" ") {
                let comp = c.trim();
                if !comp.is_empty() {
                    all_components.insert(comp.to_owned());
                    all_wires.insert((head.to_owned(), comp.to_owned()));
                }
            }
        }


        return (all_components, all_wires);
    }

    fn dfs(current: String, wires: &HashSet<(String, String)>, seen: &mut HashSet<String>) -> usize {
        let mut to_visit = HashSet::new();
        for (a,b) in wires {
            if *a == current {
                to_visit.insert(b);
            }

            if *b == current {
                to_visit.insert(a);
            }
        }

        let mut cnt = 0;
        for v in to_visit {
            if seen.contains(v) {
                continue;
            }

            seen.insert(v.to_owned());
            cnt += 1;
            cnt += dfs(v.to_owned(), wires, seen);
        }

        return cnt;
    }

    fn apply_p1(input: &str) -> usize {
        let (all_components, all_wires) = parse_lines(input);

        let mut wires = all_wires.clone();

        wires.remove(&("hvm".to_owned(),"grd".to_owned()));
        wires.remove(&("grd".to_owned(),"hvm".to_owned()));

        wires.remove(&("zfk".to_owned(),"jmn".to_owned()));
        wires.remove(&("jmn".to_owned(),"zfk".to_owned()));

        wires.remove(&("pmn".to_owned(),"kdc".to_owned()));
        wires.remove(&("kdc".to_owned(),"pmn".to_owned()));

        assert_eq!(all_wires.len()-3, wires.len());

        let mut s1 = HashSet::new();
        let hvm_cnt = dfs("hvm".to_owned(), &wires, &mut s1);

        assert_eq!(hvm_cnt, s1.len());

        let mut s2 = HashSet::new();
        let grd_cnt = dfs("grd".to_owned(), &wires, &mut s2);

        assert_eq!(grd_cnt, s2.len());

        assert_eq!(all_components.len(), hvm_cnt + grd_cnt);

        println!("hvm: {hvm_cnt}, grd: {grd_cnt}");
        return hvm_cnt * grd_cnt;

        // let mut graph: ForceGraph<String, ()> = ForceGraph::default();
        //
        // let mut nodes = HashMap::new();
        //
        // for c in all_components {
        //     let n = graph.add_force_node(c.to_owned(), c.to_owned());
        //     nodes.insert(c, n);
        // }
        //
        // for (a,b) in wires {
        //     let a_node = nodes.get(&a).unwrap();
        //     let b_node = nodes.get(&b).unwrap();
        //     graph.add_edge(*a_node,*b_node, ());
        // }

        // let text_style = Some(TextStyle {
        //     font: ("sans-serif", 20).into_font(),
        //     color: BLACK.to_backend_color(),
        //     pos: Pos {
        //         h_pos: HPos::Left,
        //         v_pos: VPos::Center,
        //     },
        // });
        //
        // let mut settings = Settings::default();
        // settings.print_progress = true;
        // settings.text_style = text_style;
        // let svg = fdg_img::gen_image(graph, Some(settings)).unwrap();
        //
        // fs::write("ring.svg", svg.as_bytes()).unwrap();



        return 0;
    }

    #[test]
    fn sample_1_p1() {
        let data = SAMPLE_1;

        let results = apply_p1(data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn sample_1_p2() {
        let data = SAMPLE_1;


        let results = 0;
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(25, "input.txt");

        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(25, "input.txt");

        let results = 0;
        println!("Answer: {results:?}");
    }
}