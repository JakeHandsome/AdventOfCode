use common::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1");
        println!("Part1: {}", part1(&input).unwrap());
    }
    {
        let _timer = Timer::new("Part 2");
        println!("Part2: {}", part2(&input).unwrap());
    }
}

fn part1(input: &str) -> R<usize> {
    let mut games = vec![];
    for line in input.lines() {
        let game = Game::new(line);
        if game.is_valid(MAX_ROUND_PT1) {
            games.push(game);
        }
    }
    Ok(games.into_iter().map(|x| x.id).sum())
}

fn part2(input: &str) -> R<usize> {
    let mut games = vec![];
    for line in input.lines() {
        games.push(Game::new(line));
    }
    Ok(games.into_iter().map(|x| x.power()).sum())
}

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Round {
    blue_count: usize,
    red_count: usize,
    green_count: usize,
}

const MAX_ROUND_PT1: Round = Round {
    blue_count: 14,
    green_count: 13,
    red_count: 12,
};

impl Game {
    pub fn new(line: &str) -> Self {
        let mut split1 = line.split(':');
        let game_and_id = split1.next().unwrap();
        let id: usize = game_and_id.split(' ').nth(1).unwrap().parse().unwrap();
        let mut game = Self { id, rounds: vec![] };
        let rounds = split1.next().unwrap().trim().split(';');
        for round_str in rounds {
            let mut round = Round::default();
            for (count, color) in round_str.split(',').map(|x| {
                let x = x.trim();
                let mut split = x.split(' ');
                let count: usize = split.next().unwrap().parse().unwrap();
                let color = split.next().unwrap();
                (count, color)
            }) {
                match color {
                    "blue" => round.blue_count = count,
                    "red" => round.red_count = count,
                    "green" => round.green_count = count,
                    _ => (),
                }
            }
            game.rounds.push(round);
        }
        game
    }
    // For part1 need to make sure game is valid based on max possible round
    fn is_valid(&self, max_round: Round) -> bool {
        self.rounds.iter().all(|round| {
            round.blue_count <= max_round.blue_count
                && round.red_count <= max_round.red_count
                && round.green_count <= max_round.green_count
        })
    }

    // Part2 each game needs a power calculated based on minimum number of colors in the bag
    fn power(&self) -> usize {
        let mut min_colors = Round::default();
        for round in &self.rounds {
            min_colors.blue_count = min_colors.blue_count.max(round.blue_count);
            min_colors.red_count = min_colors.red_count.max(round.red_count);
            min_colors.green_count = min_colors.green_count.max(round.green_count);
        }
        min_colors.blue_count * min_colors.red_count * min_colors.green_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 8);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 2286);
    }
}
