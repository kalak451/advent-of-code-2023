#[cfg(test)]
mod day22 {
    use std::cmp;
    use std::collections::HashSet;

    use itertools::Itertools;

    use crate::read_data_file;

    static SAMPLE_1: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;

    #[derive(Eq, PartialEq,Copy,Clone)]
    struct Piece {
        p1: (i64, i64, i64),
        p2: (i64, i64, i64),
        x_min: i64,
        y_min: i64,
        z_min: i64,
        x_max: i64,
        y_max: i64,
        z_max: i64,
    }

    impl Piece {
        fn new(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> Piece {
            return Piece {
                p1,
                p2,
                x_min: cmp::min(p1.0, p2.0),
                y_min: cmp::min(p1.1, p2.1),
                z_min: cmp::min(p1.2, p2.2),
                x_max: cmp::max(p1.0, p2.0),
                y_max: cmp::max(p1.1, p2.1),
                z_max: cmp::max(p1.2, p2.2)
            };
        }

        fn get_xyz_points(&self) -> HashSet<(i64,i64, i64)> {
            let mut results = HashSet::new();
            for x in self.x_min..=self.x_max {
                for y in self.y_min..=self.y_max {
                    for z in self.z_min..=self.z_max {
                        results.insert((x,y,z));
                    }

                }
            }
            return results;
        }
    }

    fn parse_pieces(input: &str) -> Vec<Piece> {
        input
            .lines()
            .enumerate()
            .map(|(_, l)| {
                let (p1_str, p2_str) = l.split("~").collect_tuple().unwrap();
                let p1: (i64, i64, i64) = p1_str.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap();
                let p2: (i64, i64, i64) = p2_str.split(",").map(|n| n.parse().unwrap()).collect_tuple().unwrap();
                Piece::new(p1, p2)
            })
            .collect_vec()
    }

    fn squash(pieces: &Vec<HashSet<(i64, i64, i64)>>) -> (Vec<HashSet<(i64, i64, i64)>>, usize) {
        let mut new = vec![];
        let mut fallen = HashSet::new();
        let mut a = 0usize;

        for b in pieces.iter().sorted_by(|a,b| {
            let am = a.iter().map(|aa| aa.2).min().unwrap();
            let bm = b.iter().map(|bb| bb.2).min().unwrap();
            Ord::cmp(&am, &bm)
        }) {
            let mut cb = b.clone();
            loop {
                let down = cb.iter().map( |(x,y,z)| (*x,*y,*z-1)).collect::<HashSet<_>>();
                if down.iter().any(|d| fallen.contains(d) || d.2 <=0) {
                    cb.iter().for_each(|p| {
                        fallen.insert(*p);
                    });
                    if cb != b.clone() {
                        a += 1
                    }
                    new.push(cb);
                    break;
                }

                cb = down
            }
        }

        return (new, a);
    }


    fn apply_p1(input: &str) -> usize {
        let pieces = parse_pieces(input).into_iter().map(|p| p.get_xyz_points()).collect_vec();
        let (squashed, _) = squash(&pieces);

        let xxx = squashed.iter().map(|p| {
            let s = squashed.iter().map(|pp| pp.to_owned()).filter(|pp| *pp != *p).collect_vec();
            let (_, a) = squash(&s);
            a
        }).collect_vec();

        return xxx.into_iter().filter(|i| *i == 0).count();
    }

    fn apply_p2(input: &str) -> usize {
        let pieces = parse_pieces(input).into_iter().map(|p| p.get_xyz_points()).collect_vec();
        let (squashed, _) = squash(&pieces);

        let xxx = squashed.iter().map(|p| {
            let s = squashed.iter().map(|pp| pp.to_owned()).filter(|pp| *pp != *p).collect_vec();
            let (_, a) = squash(&s);
            a
        }).collect_vec();

        return xxx.into_iter().sum();
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


        let results = apply_p2(data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_1() {
        let data = read_data_file(22, "input.txt");


        let results = apply_p1(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(22, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }

    #[test]
    fn hs_eq() {
        let mut a = HashSet::new();
        a.insert((3,2,1));
        a.insert((1,2,3));

        let mut n = HashSet::new();
        n.insert((1, 2, 3));
        n.insert((3,2,1));

        assert_eq!(a,n);
    }

}