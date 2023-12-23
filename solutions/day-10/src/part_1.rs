use super::parse_input;
pub fn solution(input: &str) -> i64 {
    let tiles = parse_input(input);

    // Find the S position in input
    let mut s_pos: (usize, usize) = (0, 0);
    for (i, row) in tiles.iter().enumerate() {
        if let Some((j, _)) = row.iter().enumerate().find(|(_, tile)| **tile == 'S') {
            s_pos = (i, j);
            break;
        };
    }

    // Navigate the pipes all the way back to S, counting how long the whole loop is
    let mut last: (usize, usize) = s_pos;
    let mut current: (usize, usize) = (s_pos.0 - 1, s_pos.1);
    let mut steps = 1;
    while current != s_pos {
        let next = match tiles[current.0][current.1] {
            '|' => {
                let possible_directions = [(current.0 - 1, current.1), (current.0 + 1, current.1)];
                possible_directions
                    .into_iter()
                    .find(|x| *x != last)
                    .unwrap()
            }
            '-' => {
                let possible_directions = [(current.0, current.1 + 1), (current.0, current.1 - 1)];
                possible_directions
                    .into_iter()
                    .find(|x| *x != last)
                    .unwrap()
            }

            'L' => {
                let possible_directions = [(current.0 - 1, current.1), (current.0, current.1 + 1)];
                possible_directions
                    .into_iter()
                    .find(|x| *x != last)
                    .unwrap()
            }

            'J' => {
                let possible_directions = [(current.0 - 1, current.1), (current.0, current.1 - 1)];
                possible_directions
                    .into_iter()
                    .find(|x| *x != last)
                    .unwrap()
            }
            '7' => {
                let possible_directions = [(current.0, current.1 - 1), (current.0 + 1, current.1)];
                possible_directions
                    .into_iter()
                    .find(|x| *x != last)
                    .unwrap()
            }

            'F' => {
                let possible_directions = [(current.0, current.1 + 1), (current.0 + 1, current.1)];
                possible_directions
                    .into_iter()
                    .find(|x| *x != last)
                    .unwrap()
            }
            _ => panic!("Unexpected pipe"),
        };

        last = current;
        current = next;
        steps += 1;
    }

    return steps / 2;
}
