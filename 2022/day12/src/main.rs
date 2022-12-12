use std::vec;

use common::*;
use rayon::prelude::*;

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

struct Map {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn index_to_x_y(&self, index: usize) -> (isize, isize) {
        let y = index / self.width;
        let x = index % self.width;
        (x as isize, y as isize)
    }

    fn get(&self, (x, y): (isize, isize)) -> Option<u8> {
        if x >= self.width as isize || y >= self.height as isize || x.is_negative() || y.is_negative() {
            None
        } else {
            let y: usize = y as usize;
            let x: usize = x as usize;
            Some(self.tiles[y * self.width + x])
        }
    }

    fn calc_distance(&self, start: usize, end: usize) -> R<usize> {
        let end = self.index_to_x_y(end);
        let start = Tile {
            position: self.index_to_x_y(start),
            height: b'a',
            distance: 0,
        };
        let mut all_tiles = vec![start];
        let mut num_to_skip = 0;
        loop {
            let mut new_tiles: Vec<Tile> = vec![];
            // only process the new tiles addec
            for tile in &all_tiles[num_to_skip..all_tiles.len()] {
                // Check the up down left right for each new tile
                let up = (tile.position.0, tile.position.1 - 1);
                let down = (tile.position.0, tile.position.1 + 1);
                let left = (tile.position.0 - 1, tile.position.1);
                let right = (tile.position.0 + 1, tile.position.1);
                if let Some(u) = self.get(up) {
                    if u <= tile.height + 1
                        && !all_tiles.iter().any(|f| f.position == up)
                        && !new_tiles.iter().any(|f| f.position == up)
                    {
                        new_tiles.push(Tile {
                            position: up,
                            height: u,
                            distance: tile.distance + 1,
                        });
                    }
                }
                if let Some(d) = self.get(down) {
                    if d <= tile.height + 1
                        && !all_tiles.iter().any(|f| f.position == down)
                        && !new_tiles.iter().any(|f| f.position == down)
                    {
                        new_tiles.push(Tile {
                            position: down,
                            height: d,
                            distance: tile.distance + 1,
                        });
                    }
                }
                if let Some(l) = self.get(left) {
                    if l <= tile.height + 1
                        && !all_tiles.iter().any(|f| f.position == left)
                        && !new_tiles.iter().any(|f| f.position == left)
                    {
                        new_tiles.push(Tile {
                            position: left,
                            height: l,
                            distance: tile.distance + 1,
                        });
                    }
                }
                if let Some(r) = self.get(right) {
                    if r <= tile.height + 1
                        && !all_tiles.iter().any(|f| f.position == right)
                        && !new_tiles.iter().any(|f| f.position == right)
                    {
                        new_tiles.push(Tile {
                            position: right,
                            height: r,
                            distance: tile.distance + 1,
                        });
                    }
                }
            }
            num_to_skip = new_tiles.len();
            all_tiles.append(&mut new_tiles);
            if num_to_skip == 0 {
                // This point cannot reach the end
                #[cfg(debug)]
                println!("stuck! {:?}", all_tiles.first().unwrap().position);
                return Ok(usize::MAX);
            }
            if let Some(tile) = all_tiles.iter().find(|x| x.position == end) {
                #[cfg(debug)]
                println!("Found  {:?} = {}", all_tiles.first().unwrap().position, tile.distance);
                return Ok(tile.distance);
            }
        }
    }
}

struct Tile {
    position: (isize, isize),
    height: u8,
    distance: usize,
}

fn part1(input: &str) -> R<usize> {
    let mut tiles: Vec<char> = vec![];
    let mut width = None;
    let height = input.lines().count();
    for line in input.lines() {
        if width.is_none() {
            width = Some(line.len());
        }
        for c in line.chars() {
            tiles.push(c);
        }
    }
    let start = tiles.iter().position(|x| *x == 'S').unwrap();
    let end = tiles.iter().position(|x| *x == 'E').unwrap();
    let tiles: Vec<u8> = tiles
        .into_iter()
        .map(|x| match x {
            'S' => b'a',
            'E' => b'z',
            _ => x as u8,
        })
        .collect();
    let map = Map {
        tiles,
        width: width.unwrap(),
        height,
    };
    map.calc_distance(start, end)
}

fn part2(input: &str) -> R<usize> {
    let mut tiles: Vec<char> = vec![];
    let mut width = None;
    let height = input.lines().count();
    for line in input.lines() {
        if width.is_none() {
            width = Some(line.len());
        }
        for c in line.chars() {
            tiles.push(c);
        }
    }
    let end = tiles.iter().position(|x| *x == 'E').unwrap();
    let tiles: Vec<u8> = tiles
        .into_iter()
        .map(|x| match x {
            'S' => b'a',
            'E' => b'z',
            _ => x as u8,
        })
        .collect();
    // Get all the start indexes
    let starts = tiles
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == b'a')
        .map(|(i, _)| i.to_owned())
        .collect::<Vec<_>>();
    // Run all calculations in parallel and return the minimum
    Ok(starts
        .into_par_iter()
        .map(|start| {
            let map = Map {
                tiles: tiles.clone(),
                width: width.unwrap(),
                height,
            };
            let dist = map.calc_distance(start, end).unwrap();
            dist
        })
        .min()
        .unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE1: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
    #[test]
    fn p1_test() {
        assert_eq!(part1(SAMPLE1).unwrap(), 31);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1).unwrap(), 29);
    }
}
