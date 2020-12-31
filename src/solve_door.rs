use itertools::Itertools;

pub fn solve_door() {
    let values = vec![2i32, 3, 5, 7, 9];

    println!("matching: {:?}", values);
    println!("to equation '_ + _ * _^2 + _^3 - _ = 399'");

    let results: Vec<_> = values
        .iter()
        .copied()
        .permutations(values.len())
        .map(|nums| {
            (
                nums.clone(),
                nums[0] + nums[1] * nums[2].pow(2) + nums[3].pow(3) - nums[4],
            )
        })
        .filter(|(_, b)| *b == 399)
        .next()
        .unwrap()
        .0;

    println!("{:?}", results);
}

// [9, 2, 5, 7, 3]
// blue, red, shiny, concave, corroded
