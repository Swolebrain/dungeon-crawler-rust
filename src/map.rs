use crate::camera::Camera;
use crate::globalimports::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, PartialEq, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    return ((y * SCREEN_WIDTH) + x) as usize;
}

pub fn is_in_bounds(point: Point) -> bool {
    point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn map_idx(x: i32, y: i32) -> usize {
        return ((y * SCREEN_WIDTH) + x) as usize;
    }

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..camera.bottom_y {
            for x in camera.left_x..camera.right_x {
                if !self.in_bounds(Point::new(x, y)) {
                    continue;
                }
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('.'),
                        );
                    }
                    TileType::Wall => {
                        ctx.set(
                            x - camera.left_x,
                            y - camera.top_y,
                            WHITE,
                            BLACK,
                            to_cp437('#'),
                        );
                    }
                }
            }
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        is_in_bounds(point)
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}
