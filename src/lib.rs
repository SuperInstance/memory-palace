//! # memory-palace — Method of Loci Memory Indexing
//!
//! In the ancient method of loci, you place memories in spatial locations within
//! an imagined building. This crate implements that metaphor as a graph-based
//! memory index with rooms, corridors, and spatial recall.

use std::collections::{HashMap, HashSet, VecDeque};

// ─── Coordinates ─────────────────────────────────────────────────────────────

/// 3D coordinates within a room.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coord {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn distance_to(&self, other: &Coord) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).sqrt()
    }
}

// ─── Locus (Memory Item) ─────────────────────────────────────────────────────

/// An item placed in a specific location within a room.
#[derive(Debug, Clone)]
pub struct Locus {
    pub id: usize,
    pub name: String,
    pub coord: Coord,
    pub tags: HashSet<String>,
    pub data: Option<String>,
}

impl Locus {
    pub fn new(id: usize, name: &str, coord: Coord) -> Self {
        Self {
            id,
            name: name.to_string(),
            coord,
            tags: HashSet::new(),
            data: None,
        }
    }

    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        for tag in tags {
            self.tags.insert(tag.to_string());
        }
        self
    }

    pub fn with_data(mut self, data: &str) -> Self {
        self.data = Some(data.to_string());
        self
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    pub fn distance_to(&self, other: &Locus) -> f64 {
        self.coord.distance_to(&other.coord)
    }
}

// ─── Room ────────────────────────────────────────────────────────────────────

/// A named location within the palace.
#[derive(Debug, Clone)]
pub struct Room {
    pub id: usize,
    pub name: String,
    pub description: String,
    loci: HashMap<usize, Locus>,
    next_locus_id: usize,
}

impl Room {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            description: String::new(),
            loci: HashMap::new(),
            next_locus_id: 0,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    /// Place a locus in this room.
    pub fn place(&mut self, name: &str, coord: Coord) -> usize {
        let id = self.next_locus_id;
        self.next_locus_id += 1;
        let locus = Locus::new(id, name, coord);
        self.loci.insert(id, locus);
        id
    }

    /// Place a locus with tags.
    pub fn place_with_tags(&mut self, name: &str, coord: Coord, tags: &[&str]) -> usize {
        let id = self.next_locus_id;
        self.next_locus_id += 1;
        let locus = Locus::new(id, name, coord).with_tags(tags);
        self.loci.insert(id, locus);
        id
    }

    /// Get a locus by id.
    pub fn get_locus(&self, id: usize) -> Option<&Locus> {
        self.loci.get(&id)
    }

    /// Get all loci.
    pub fn loci(&self) -> &HashMap<usize, Locus> {
        &self.loci
    }

    /// Number of loci in this room.
    pub fn locus_count(&self) -> usize {
        self.loci.len()
    }

    /// Find loci near a coordinate within radius.
    pub fn find_near(&self, coord: Coord, radius: f64) -> Vec<&Locus> {
        self.loci.values()
            .filter(|l| l.coord.distance_to(&coord) <= radius)
            .collect()
    }

    /// Find loci by tag.
    pub fn find_by_tag(&self, tag: &str) -> Vec<&Locus> {
        self.loci.values().filter(|l| l.has_tag(tag)).collect()
    }
}

// ─── Corridor ────────────────────────────────────────────────────────────────

/// A weighted connection between rooms.
#[derive(Debug, Clone)]
pub struct Corridor {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
    pub name: String,
}

impl Corridor {
    pub fn new(from: usize, to: usize, weight: f64) -> Self {
        Self { from, to, weight, name: String::new() }
    }

    pub fn named(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

// ─── Palace ──────────────────────────────────────────────────────────────────

/// The Memory Palace — a graph of rooms connected by corridors.
#[derive(Debug, Clone)]
pub struct Palace {
    rooms: HashMap<usize, Room>,
    corridors: Vec<Corridor>,
    next_room_id: usize,
}

impl Palace {
    pub fn new() -> Self {
        Self { rooms: HashMap::new(), corridors: Vec::new(), next_room_id: 0 }
    }

    /// Add a room to the palace.
    pub fn add_room(&mut self, name: &str) -> usize {
        let id = self.next_room_id;
        self.next_room_id += 1;
        self.rooms.insert(id, Room::new(id, name));
        id
    }

    /// Get a room by id.
    pub fn get_room(&self, id: usize) -> Option<&Room> {
        self.rooms.get(&id)
    }

    /// Get a mutable room by id.
    pub fn get_room_mut(&mut self, id: usize) -> Option<&mut Room> {
        self.rooms.get_mut(&id)
    }

    /// Connect two rooms with a corridor.
    pub fn connect(&mut self, from: usize, to: usize, weight: f64) {
        self.corridors.push(Corridor::new(from, to, weight));
    }

    /// Number of rooms.
    pub fn room_count(&self) -> usize {
        self.rooms.len()
    }

    /// Number of corridors.
    pub fn corridor_count(&self) -> usize {
        self.corridors.len()
    }

    /// Total loci across all rooms.
    pub fn total_loci(&self) -> usize {
        self.rooms.values().map(|r| r.locus_count()).sum()
    }

    /// Shortest path between two rooms (BFS, unweighted).
    pub fn shortest_path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        if from == to {
            return Some(vec![from]);
        }

        let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
        for c in &self.corridors {
            adj.entry(c.from).or_default().push(c.to);
            adj.entry(c.to).or_default().push(c.from);
        }

        let mut visited = HashSet::new();
        let mut parent: HashMap<usize, usize> = HashMap::new();
        let mut queue = VecDeque::new();

        visited.insert(from);
        queue.push_back(from);

        while let Some(current) = queue.pop_front() {
            if current == to {
                let mut path = vec![to];
                let mut node = to;
                while let Some(&p) = parent.get(&node) {
                    path.push(p);
                    node = p;
                }
                path.reverse();
                return Some(path);
            }

            for &neighbor in adj.get(&current).unwrap_or(&Vec::new()) {
                if visited.insert(neighbor) {
                    parent.insert(neighbor, current);
                    queue.push_back(neighbor);
                }
            }
        }

        None
    }

    /// Find all loci matching a tag across all rooms.
    pub fn find_by_tag(&self, tag: &str) -> Vec<(&Room, &Locus)> {
        let mut results = Vec::new();
        for room in self.rooms.values() {
            for locus in room.find_by_tag(tag) {
                results.push((room, locus));
            }
        }
        results
    }

    /// Navigate to a locus by name (linear search).
    pub fn recall(&self, name: &str) -> Vec<(&Room, &Locus)> {
        let mut results = Vec::new();
        for room in self.rooms.values() {
            for locus in room.loci.values() {
                if locus.name.contains(name) {
                    results.push((room, locus));
                }
            }
        }
        results
    }
}

impl Default for Palace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_distance() {
        let a = Coord::new(0.0, 0.0, 0.0);
        let b = Coord::new(3.0, 4.0, 0.0);
        assert!((a.distance_to(&b) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_locus_with_tags() {
        let l = Locus::new(0, "test", Coord::origin()).with_tags(&["alpha", "beta"]);
        assert!(l.has_tag("alpha"));
        assert!(!l.has_tag("gamma"));
    }

    #[test]
    fn test_room_place() {
        let mut room = Room::new(0, "test_room");
        let id = room.place("item1", Coord::new(1.0, 2.0, 3.0));
        assert_eq!(room.locus_count(), 1);
        assert_eq!(room.get_locus(id).unwrap().name, "item1");
    }

    #[test]
    fn test_room_find_near() {
        let mut room = Room::new(0, "test");
        room.place("near", Coord::new(1.0, 0.0, 0.0));
        room.place("far", Coord::new(10.0, 0.0, 0.0));
        let found = room.find_near(Coord::origin(), 2.0);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name, "near");
    }

    #[test]
    fn test_room_find_by_tag() {
        let mut room = Room::new(0, "test");
        room.place_with_tags("tagged", Coord::origin(), &["important"]);
        room.place("untagged", Coord::origin());
        let found = room.find_by_tag("important");
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn test_palace_add_rooms() {
        let mut p = Palace::new();
        let r0 = p.add_room("entrance");
        let r1 = p.add_room("library");
        assert_eq!(p.room_count(), 2);
        assert_ne!(r0, r1);
    }

    #[test]
    fn test_palace_connect() {
        let mut p = Palace::new();
        let r0 = p.add_room("a");
        let r1 = p.add_room("b");
        p.connect(r0, r1, 1.0);
        assert_eq!(p.corridor_count(), 1);
    }

    #[test]
    fn test_shortest_path_direct() {
        let mut p = Palace::new();
        let r0 = p.add_room("a");
        let r1 = p.add_room("b");
        p.connect(r0, r1, 1.0);
        let path = p.shortest_path(r0, r1).unwrap();
        assert_eq!(path, vec![r0, r1]);
    }

    #[test]
    fn test_shortest_path_multi_hop() {
        let mut p = Palace::new();
        let r0 = p.add_room("a");
        let r1 = p.add_room("b");
        let r2 = p.add_room("c");
        p.connect(r0, r1, 1.0);
        p.connect(r1, r2, 1.0);
        let path = p.shortest_path(r0, r2).unwrap();
        assert_eq!(path.len(), 3);
    }

    #[test]
    fn test_shortest_path_disconnected() {
        let mut p = Palace::new();
        let r0 = p.add_room("a");
        let r1 = p.add_room("b");
        assert!(p.shortest_path(r0, r1).is_none());
    }

    #[test]
    fn test_palace_find_by_tag() {
        let mut p = Palace::new();
        let r0 = p.add_room("room0");
        p.get_room_mut(r0).unwrap().place_with_tags("item", Coord::origin(), &["secret"]);
        let results = p.find_by_tag("secret");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_palace_recall() {
        let mut p = Palace::new();
        let r0 = p.add_room("room0");
        let r1 = p.add_room("room1");
        p.get_room_mut(r0).unwrap().place("golden key", Coord::origin());
        p.get_room_mut(r1).unwrap().place("silver key", Coord::origin());
        let results = p.recall("key");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_palace_total_loci() {
        let mut p = Palace::new();
        let r0 = p.add_room("r0");
        let r1 = p.add_room("r1");
        p.get_room_mut(r0).unwrap().place("a", Coord::origin());
        p.get_room_mut(r0).unwrap().place("b", Coord::origin());
        p.get_room_mut(r1).unwrap().place("c", Coord::origin());
        assert_eq!(p.total_loci(), 3);
    }

    #[test]
    fn test_palace_shortest_path_self() {
        let mut p = Palace::new();
        let r0 = p.add_room("a");
        let path = p.shortest_path(r0, r0).unwrap();
        assert_eq!(path, vec![r0]);
    }
}
