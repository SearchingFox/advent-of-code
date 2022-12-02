fn main() {
    let tmp1 = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    let input = tmp1.lines().map(|l| {
        let t = l.split_whitespace().collect::<Vec<_>>();
        let t1 = t[1].split(',').map(|x| {
            let t2 = x.split("..").collect::<Vec<_>>();
            (t2[0][2..].parse::<u32>().unwrap(), t2[1].parse::<u32>().unwrap())
        }).collect::<Vec<_>>();

        (t[0].clone(), t1[0].clone(), t1[1].clone(), t1[2].clone())
    }).collect::<Vec<_>>();

    println!("{:?}", input);
}
