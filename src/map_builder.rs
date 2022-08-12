use crate::globalimports::*;
use std::cmp::{max, min};

const NUM_ROOMS: usize = 25;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        mb.player_start = mb.rooms[0].center();
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter_mut() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                self.rooms.push(room);
                room.for_each(|p| {
                    if is_in_bounds(p) {
                        self.map.tiles[map_idx(p.x, p.y)] = TileType::Floor;
                    }
                });
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev_room_center = rooms[i - 1].center();
            let curr_room_center = room.center();

            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(
                    prev_room_center.x,
                    curr_room_center.x,
                    prev_room_center.y,
                );
                self.apply_vertical_tunnel(
                    prev_room_center.y,
                    curr_room_center.y,
                    curr_room_center.x,
                );
            } else {
                self.apply_vertical_tunnel(
                    prev_room_center.y,
                    curr_room_center.y,
                    prev_room_center.x,
                );
                self.apply_horizontal_tunnel(
                    prev_room_center.x,
                    curr_room_center.x,
                    curr_room_center.y,
                );
            }
        }
    }
}
