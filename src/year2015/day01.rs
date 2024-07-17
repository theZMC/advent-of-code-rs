pub fn solve(challenge: &str) -> (i32, i32) {
    let mut floor: i32 = 0;
    let mut instruction_count = 1;
    let mut negative_floor_instruction = 0;
    challenge.chars().for_each(|c| {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 && negative_floor_instruction == 0 {
            negative_floor_instruction = instruction_count;
        }

        instruction_count += 1;
    });

    (floor, negative_floor_instruction)
}
