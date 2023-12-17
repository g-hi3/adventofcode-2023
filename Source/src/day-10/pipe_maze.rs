#[derive(Debug, PartialEq, Clone)]
struct Coords {
    x: usize,
    y: usize
}

impl Coords {
    fn right(&self) -> Option<Self> {
        if self.x == usize::MAX {
            None
        } else {
            Some(Self {
                x: self.x + 1,
                y: self.y
            })
        }
    }
    fn left(&self) -> Option<Self> {
        if self.x == 0 {
            None
        }
        else {
            Some(Self {
                x: self.x - 1,
                y: self.y
            })
        }
    }
    fn up(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            Some(Self {
                x: self.x,
                y: self.y - 1
            })
        }
    }
    fn down(&self) -> Option<Self> {
        if self.y == usize::MAX {
            None
        } else {
            Some(Self {
                x: self.x,
                y: self.y + 1
            })
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Maze {
    tiles: Vec<Vec<Tile>>,
    starting_position: Option<Coords>
}

impl Maze {
    pub fn new(s: &str) -> Self {
        let mut grid = Vec::<Vec<Tile>>::new();
        let mut starting_position = Option::<Coords>::None;

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::<Tile>::new();

            for (x, c) in line.chars().enumerate() {
                if let Some(tile) = Tile::new(c) {
                    if let Tile::StartingPosition = &tile {
                        starting_position = Some(Coords { x, y });
                    }

                    row.push(tile);
                }
            }

            grid.push(row);
        }

        Self { tiles: grid, starting_position }
    }

    fn get_tile_at(&self, coords: &Coords) -> Option<&Tile> {
        self.tiles
            .get(coords.y)?
            .get(coords.x)
    }

    fn starting_position(&self) -> Option<&Coords> {
        match &self.starting_position {
            Some(starting_position) => Some(&starting_position),
            None => None
        }
    }

    pub fn loop_length(&self) -> Option<u32> {
        let starting_position = self.starting_position()?;

        if let Some(distance) = self.left_distance(&starting_position) {
            return Some(distance);
        }

        if let Some(distance) = self.right_distance(&starting_position) {
            return Some(distance);
        }

        if let Some(distance) = self.up_distance(&starting_position) {
            return Some(distance);
        }

        if let Some(distance) = self.down_distance(&starting_position) {
            return Some(distance);
        }

        None
    }

    fn right_distance(&self, from: &Coords) -> Option<u32> {
        match &from.right() {
            None => None,
            Some(right) => match self.get_tile_at(right) {
                Some(Tile::StartingPosition) => Some(0),
                Some(Tile::HorizontalPipe) => self.right_distance(right),
                Some(Tile::NorthWestBend) => self.up_distance(right),
                Some(Tile::SouthWestBend) => self.down_distance(right),
                _ => None
            }.map(|distance| distance + 1)
        }
    }

    fn up_distance(&self, from: &Coords) -> Option<u32> {
        match &from.up() {
            None => None,
            Some(up) => match self.get_tile_at(up) {
                Some(Tile::StartingPosition) => Some(0),
                Some(Tile::VerticalPipe) => self.up_distance(up),
                Some(Tile::SouthEastBend) => self.right_distance(up),
                Some(Tile::SouthWestBend) => self.left_distance(up),
                _ => None
            }.map(|distance| distance + 1)
        }
    }

    fn left_distance(&self, from: &Coords) -> Option<u32> {

        match &from.left() {
            None => None,
            Some(left) => match self.get_tile_at(left) {
                Some(Tile::StartingPosition) => Some(0),
                Some(Tile::HorizontalPipe) => self.left_distance(left),
                Some(Tile::NorthEastBend) => self.up_distance(left),
                Some(Tile::SouthEastBend) => self.down_distance(left),
                _ => None
            }.map(|distance| distance + 1)
        }
    }

    fn down_distance(&self, from: &Coords) -> Option<u32> {
        match &from.down() {
            None => None,
            Some(down) => match self.get_tile_at(down) {
                Some(Tile::StartingPosition) => Some(0),
                Some(Tile::VerticalPipe) => self.down_distance(down),
                Some(Tile::NorthWestBend) => self.left_distance(down),
                Some(Tile::NorthEastBend) => self.right_distance(down),
                _ => None
            }.map(|distance| distance + 1)
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    StartingPosition
}

impl Tile {
    fn new(c: char) -> Option<Tile> {
        match c {
            '|' => Some(Tile::VerticalPipe),
            '-' => Some(Tile::HorizontalPipe),
            'L' => Some(Tile::NorthEastBend),
            'J' => Some(Tile::NorthWestBend),
            '7' => Some(Tile::SouthWestBend),
            'F' => Some(Tile::SouthEastBend),
            '.' => Some(Tile::Ground),
            'S' => Some(Tile::StartingPosition),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_length() {
        let maze = Maze::new("-L|F7
7S-7|
L|7||
-L-J|
L|-JF");
        assert_eq!(maze.loop_length(), Some(8));

        let maze = Maze::new("7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ");
        assert_eq!(maze.loop_length(), Some(16));

    }
}