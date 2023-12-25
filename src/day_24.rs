#[cfg(test)]
mod day24 {
    use std::fmt::{Display, Formatter};
    use itertools::Itertools;
    use num::abs;
    use z3::ast::Ast;
    use crate::read_data_file;

    static SAMPLE_1: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;

    struct Point2D {
        x: f64,
        y: f64,
    }

    struct Point3D {
        x: f64,
        y: f64,
        z: f64
    }

    struct Area2D {
        min: Point2D,
        max: Point2D,
    }

    impl Area2D {
        fn is_in_area(&self, p: &Point2D) -> bool {
            if p.x < self.min.x {
                return false;
            }

            if p.x > self.max.x {
                return false;
            }

            if p.y < self.min.y {
                return false;
            }

            if p.y > self.max.y {
                return false;
            }

            return true;
        }
    }

    struct Line2D {
        p1: Point2D,
        p2: Point2D,
    }

    struct Line3D {
        p1: Point3D,
        p2: Point3D,
    }

    impl Line2D {
        fn intersects(&self, other: &Line2D) -> Option<Point2D> {
            let x1 = self.p1.x;
            let x2 = self.p2.x;
            let x3 = other.p1.x;
            let x4 = other.p2.x;

            let y1 = self.p1.y;
            let y2 = self.p2.y;
            let y3 = other.p1.y;
            let y4 = other.p2.y;

            let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

            if close_enough(denom, 0f64) {
                return None;
            }

            let px = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denom;
            let py = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denom;

            return Some(Point2D { x: px, y: py });
        }
    }

    #[derive(Debug, Clone)]
    struct Hailstone {
        init_x: f64,
        init_y: f64,
        init_z: f64,

        vx: i64,
        vy: i64,
        vz: i64,
    }

    impl Display for Hailstone {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{ init_x: {}, init_y: {}, init_z: {}, vx: {}, vy: {}, vz: {} }}", self.init_x, self.init_y, self.init_z, self.vx, self.vy, self.vz)
        }
    }

    impl Hailstone {
        fn to_xy_line(&self) -> Line2D {
            return Line2D {
                p1: Point2D {
                    x: self.init_x,
                    y: self.init_y,
                },
                p2: Point2D {
                    x: self.init_x + self.vx as f64 * 100000000000f64,
                    y: self.init_y + self.vy as f64 * 100000000000f64,
                },
            };
        }

        fn to_xyz_line(&self) -> Line3D {
            return Line3D {
                p1: Point3D {
                    x: self.init_x,
                    y: self.init_y,
                    z: self.init_z
                },
                p2: Point3D {
                    x: self.init_x + self.vx as f64 * 100000000000f64,
                    y: self.init_y + self.vy as f64 * 100000000000f64,
                    z: self.init_z + self.vz as f64 * 100000000000f64,
                },
            };
        }

        fn is_xy_point_in_past(&self, p: &Point2D) -> bool {
            let xn = (p.x - self.init_x) / self.vx as f64;
            // let yn = (p.y - self.init_y) / self.vy as f64;

            // if !close_enough(xn, yn) {
            //     panic!();
            // }

            return if xn <= 0f64 {
                true
            } else {
                false
            };
        }

        fn intersects_xy_in_future(&self, other: &Hailstone) -> Option<Point2D> {
            let a = self.to_xy_line();
            let b = other.to_xy_line();

            if let Some(i) = a.intersects(&b) {
                let a_future = !self.is_xy_point_in_past(&i);
                let b_future = !other.is_xy_point_in_past(&i);

                if a_future && b_future {
                    return Some(i);
                }
            }

            return None;
        }
    }

    fn close_enough(a: f64, b: f64) -> bool {
        return abs(a - b) < 0.000000001;
    }

    fn parse_hailstone(input: &str) -> Hailstone {
        let (init_str, v_str) = input.split("@").collect_tuple().unwrap();

        let (init_x, init_y, init_z) = init_str.split(",")
            .map(|s| s.trim().parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();

        let (vx, vy, vz) = v_str.split(",")
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        return Hailstone {
            init_x,
            init_y,
            init_z,
            vx,
            vy,
            vz,
        };
    }

    fn parse_hailstones(input: &str) -> Vec<Hailstone> {
        input
            .lines()
            .map(|l| parse_hailstone(l))
            .collect_vec()
    }

    fn apply_p1(input: &str, area: &Area2D) -> usize {
        let stones = parse_hailstones(input);

        stones
            .into_iter()
            .combinations(2)
            .map(|stns| stns.into_iter().collect_tuple().unwrap())
            .map(|(a, b)| a.intersects_xy_in_future(&b))
            .flatten()
            .filter(|i| area.is_in_area(i))
            .count()
    }

    fn apply_p2(input: &str) -> i64 {
        let stones = parse_hailstones(input);

        let cfg = z3::Config::new();
        let context = z3::Context::new(&cfg);
        let solver = z3::Solver::new(&context);

        let r_x = z3::ast::Int::new_const(&context, "r_x");
        let r_y = z3::ast::Int::new_const(&context, "r_y");
        let r_z = z3::ast::Int::new_const(&context, "r_z");

        let r_vx = z3::ast::Int::new_const(&context, "r_vx");
        let r_vy = z3::ast::Int::new_const(&context, "r_vy");
        let r_vz = z3::ast::Int::new_const(&context, "r_vz");

        for (i,s) in stones.iter().enumerate() {
            let a = z3::ast::Int::from_i64(&context, s.init_x as i64);
            let b = z3::ast::Int::from_i64(&context, s.init_y as i64);
            let c = z3::ast::Int::from_i64(&context, s.init_z as i64);

            let va = z3::ast::Int::from_i64(&context, s.vx);
            let vb = z3::ast::Int::from_i64(&context, s.vy);
            let vc = z3::ast::Int::from_i64(&context, s.vz);

            let t= z3::ast::Int::new_const(&context, format!("t{i}"));

            solver.assert(&t.gt(&z3::ast::Int::from_i64(&context, 0)));
            solver.assert(&(r_x.clone() + r_vx.clone() * t.clone())._eq(&(a + va * t.clone())));
            solver.assert(&(r_y.clone() + r_vy.clone() * t.clone())._eq(&(b + vb * t.clone())));
            solver.assert(&(r_z.clone() + r_vz.clone() * t.clone())._eq(&(c + vc * t.clone())));
        }

        if solver.check() == z3::SatResult::Sat {
            let Some(m) = solver.get_model() else {
                panic!("Failed to solve!");
            };

            let result = m.eval(&(r_x + r_y +r_z), true).unwrap();

            return result.as_i64().unwrap();
        }

        panic!();
    }

    #[test]
    fn sample_1_p1() {
        let data = SAMPLE_1;

        let area = Area2D {
            min: Point2D { x: 7f64, y: 7f64 },
            max: Point2D { x: 27f64, y: 27f64 },
        };

        let results = apply_p1(data, &area);
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
        let data = read_data_file(24, "input.txt");

        let area = Area2D {
            min: Point2D { x: 200000000000000f64, y: 200000000000000f64 },
            max: Point2D { x: 400000000000000f64, y: 400000000000000f64 },
        };

        let results = apply_p1(&data, &area);
        println!("Answer: {results:?}");
    }

    #[test]
    fn part_2() {
        let data = read_data_file(24, "input.txt");

        let results = apply_p2(&data);
        println!("Answer: {results:?}");
    }
}