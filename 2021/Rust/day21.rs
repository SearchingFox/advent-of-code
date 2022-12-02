fn main() {
    let mut p1 = 2;
    let mut p2 = 8;
    let mut pp1 = 0;
    let mut pp2 = 0;

    let mut step = 1;
    for k in (1..=100).cycle().step_by(6) {
        p1 = (p1 + 3*(k + 1)) % 10;
        p2 = (p2 + 3*k + 12) % 10;

        if p1 == 0 {
            pp1 += 10;
        } else {
            pp1 += p1;
        }
        if pp1 >= 1000 {
            println!("1 Won: {}", (step * 6 - 3) * pp2);
            break;
        }
        if p2 == 0 {
            p2 = 10;
        }
        pp2 += p2;
        if pp2 >= 1000 {
            println!("2 Won: {}", step * 6 * pp1);
            break;
        }
        step += 1;
    }
}
