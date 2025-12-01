fn main() {

    let input = include_str!("input.txt");
    let start = 50;
    
    let mut n_zeros = 0;
    let mut pos = start;

    for line in input.lines() {
        let dir = &line[0..1];
        let steps: i32 = line[1..].parse().unwrap();

        if dir == "L" {
            let zero_hits = if pos == 0 {
                steps.div_euclid(100)
            } else {
                (steps - pos).div_euclid(100) + 1
            };

            n_zeros += zero_hits;
            pos = (pos - steps).rem_euclid(100);
        }
        else if dir == "R" {

            let zero_hits = (pos + steps).div_euclid(100);

            n_zeros += zero_hits;
            pos = (pos + steps).rem_euclid(100);
        }

        println!("{} {} {}", dir, steps, pos);
    }

    println!("Number of times at position 0: {}", n_zeros);
}