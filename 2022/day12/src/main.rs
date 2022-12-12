use std::vec;

use common::*;
use rayon::prelude::*;

fn main() {
    let input = read_input_file_for_project_as_string!();
    {
        let _timer = Timer::new("Part 1 from_start");
        println!("Part1: {}", part1(&input, false).unwrap());
    }
    {
        let _timer = Timer::new("Part 1 from_end");
        println!("Part1: {}", part1(&input, true).unwrap());
    }
    {
        let _timer = Timer::new("Part 2 from_start");
        println!("Part2: {}", part2(&input, false).unwrap());
    }
    {
        let _timer = Timer::new("Part 2 from_end");
        println!("Part2: {}", part2(&input, true).unwrap());
    }
    {
        let _timer = Timer::new("Part 2 calc all and cache");
        println!("Part2: {}", part2_calc_all_and_cache(&input).unwrap());
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

    fn calc_distance_from_start(&self, start: usize, end: usize) -> R<usize> {
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
                for position in [up, down, left, right] {
                    if let Some(new_tile) =
                        self.is_tile_new_and_navigable(position, &all_tiles, &new_tiles, tile, false)
                    {
                        new_tiles.push(new_tile)
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

    fn calc_distance_from_end(&self, start: usize, end: usize) -> R<usize> {
        let start_tile = Tile {
            position: self.index_to_x_y(end),
            height: b'z',
            distance: 0,
        };
        let end = self.index_to_x_y(start);
        let mut all_tiles = vec![start_tile];
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
                for position in [up, down, left, right] {
                    if let Some(new_tile) = self.is_tile_new_and_navigable(position, &all_tiles, &new_tiles, tile, true)
                    {
                        new_tiles.push(new_tile)
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

    fn calc_all_distances(&mut self, end: usize) -> R<Vec<Tile>> {
        let start_tile = Tile {
            position: self.index_to_x_y(end),
            height: b'z',
            distance: 0,
        };
        let mut all_tiles = vec![start_tile];
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
                for position in [up, down, left, right] {
                    if let Some(new_tile) = self.is_tile_new_and_navigable(position, &all_tiles, &new_tiles, tile, true)
                    {
                        new_tiles.push(new_tile);
                    }
                }
            }
            num_to_skip = new_tiles.len();
            all_tiles.append(&mut new_tiles);
            if num_to_skip == 0 {
                // This point cannot reach the end
                #[cfg(debug)]
                println!("stuck! {:?}", all_tiles.first().unwrap().position);
                return Ok(all_tiles);
            }
        }
    }
    /// Determines if the new tile
    fn is_tile_new_and_navigable(
        &self,
        position: (isize, isize),
        all_tiles: &[Tile],
        new_tiles: &[Tile],
        tile: &Tile,
        from_end: bool,
    ) -> Option<Tile> {
        match self.get(position) {
            Some(u) => {
                let navigable = if from_end {
                    tile.height <= u + 1
                } else {
                    u <= tile.height + 1
                };
                if navigable
                    && !all_tiles.iter().any(|f| f.position == position) // Check for new
                    && !new_tiles.iter().any(|f| f.position == position)
                {
                    Some(Tile {
                        position,
                        height: u,
                        distance: tile.distance + 1,
                    })
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

#[derive(Clone, Copy)]
struct Tile {
    position: (isize, isize),
    height: u8,
    distance: usize,
}

fn part1(input: &str, from_end: bool) -> R<usize> {
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
    if from_end {
        map.calc_distance_from_end(start, end)
    } else {
        map.calc_distance_from_start(start, end)
    }
}

fn part2(input: &str, from_end: bool) -> R<usize> {
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
    if from_end {
        Ok(starts
            .into_par_iter()
            .map(|start| {
                let map = Map {
                    tiles: tiles.clone(),
                    width: width.unwrap(),
                    height,
                };
                let dist = map.calc_distance_from_end(start, end).unwrap();
                dist
            })
            .min()
            .unwrap())
    } else {
        Ok(starts
            .into_par_iter()
            .map(|start| {
                let map = Map {
                    tiles: tiles.clone(),
                    width: width.unwrap(),
                    height,
                };
                let dist = map.calc_distance_from_start(start, end).unwrap();
                dist
            })
            .min()
            .unwrap())
    }
}

fn part2_calc_all_and_cache(input: &str) -> R<usize> {
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
    let mut min = usize::MAX;

    let mut map = Map {
        tiles,
        width: width.unwrap(),
        height,
    };
    let tiles = map.calc_all_distances(end).unwrap();
    for start in starts {
        match tiles
            .iter()
            .find(|x| map.index_to_x_y(start) == x.position)
            .map(|x| x.distance)
        {
            Some(distance) => {
                if distance < min {
                    min = distance;
                }
            }
            None => continue,
        }
    }

    Ok(min)
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
        assert_eq!(part1(SAMPLE1, true).unwrap(), 31);
    }
    #[test]
    fn p2_test() {
        assert_eq!(part2(SAMPLE1, true).unwrap(), 29);
    }
}
