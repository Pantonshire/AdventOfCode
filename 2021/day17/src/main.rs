use std::collections::HashSet;

fn main() {
    let (tx1, tx2) = (137, 171);
    let (ty1, ty2) = (-98, -73);

    let mut vxs_eq = Vec::new();
    let mut vxs_ge = Vec::new();

    for vx in 1..=tx2 {
        let mut cvx = vx;
        let mut x = 0;
        let mut steps = 0;
        while x <= tx2 {
            if tx1 <= x && x <= tx2 {
                if cvx == 0 {
                    &mut vxs_ge
                } else {
                    &mut vxs_eq
                }.push((steps, vx));
            }
            x += cvx;
            if cvx == 0 {
                break;
            } else {
                cvx -= 1;
            }
            steps += 1;
        }
    }

    let mut max_valid_y = 0;
    let mut vs = HashSet::new();

    for vy in -100..1000 {
        let mut step = 0;
        let mut cvy = vy;
        let mut max_y = 0;
        let mut y = 0;
        while y >= ty1 {
            if y > max_y {
                max_y = y;
            }
            if ty1 <= y && y <= ty2 {
                let mut any = false;
                for (_, vx) in vxs_eq.iter().filter(|(s, _)| *s == step)
                    .chain(vxs_ge.iter().filter(|(s, _)| *s <= step))
                {
                    any = true;
                    vs.insert((*vx, vy));
                }
                if any && max_y > max_valid_y {
                    max_valid_y = max_y;
                }
            }
            step += 1;
            y += cvy;
            cvy -= 1;
        }
    }

    println!("Part 1: {}", max_valid_y);
    println!("Part 2: {}", vs.len());
}
