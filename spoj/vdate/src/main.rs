use std::io;

fn vdate_solution(x: u32, y: u32) -> i32 {
    // If I have the same leftover spaces as known spaces that means I decided to give
    // 0 locations per girl, which would imply I could have been dating any number of
    // girls.
    if y == x {
        return -1;
    }

    // If cannot have more leftover spaces than known spaces.
    if y > x {
        return 0;
    }

    let mut possible_solutions = 0;
    let assigned_locations = x - y;
    for i in 1..assigned_locations {
        // We only check until the square root of the assigned locations as we will
        // check both cases using only one factor.
        if i * i > assigned_locations {
            break;
        }

        // Check if i is a number that is a contender for being the number of possible
        // girls Verma is dating or the dating locations he distributed to each girl
        if assigned_locations % i != 0 {
            continue;
        }

        // Let's check if the possible number of girls he is dating could have
        // obtained an additional location. If they could have, it is impossible
        // that he is dating the specified number of girls as he is maximizing
        // the number of locations per girl.

        // Case 1:
        // Number of locations distributed to each girl: assigned_locations / i
        // Number of girls he is dating: i
        if i > y {
            possible_solutions += 1;
        }

        // Do not count twice the same number
        if i * i == assigned_locations {
            continue;
        }

        // Case 2:
        // Number of locations distributed to each girl: i
        // Number of girls he is dating: assigned_locations / i
        // That is check assigned_locations / i > y
        if assigned_locations > y * i {
            possible_solutions += 1;
        }
    }
    possible_solutions
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let number_of_cases: u8 = line.trim().parse().unwrap();
    for _ in 0..number_of_cases {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut numbers_array = line.trim().split(' ');
        let x: u32 = numbers_array.next().unwrap().parse().unwrap();
        let y: u32 = numbers_array.next().unwrap().parse().unwrap();
        println!("{}", vdate_solution(x, y));
    }
}
