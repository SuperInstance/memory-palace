//! Method of loci for agent memory organization with spatial recall.
//!
//! Implements a memory palace system where memories are stored in spatial rooms
//! connected by corridors, with anchors for memorable location bindings and
//! path traversal for sequential recall.

use std::collections::{HashMap, HashSet};

// ── Module: room ─────────────────────────────────────────────────────────

/// A spatial memory container holding associated items.
#[derive(Debug, Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub items: HashMap<String, String>, // item_id -> description
    pub atmosphere: String, // sensory cue for recall
}

impl Room {
    /// Create a new room.
    pub fn new(id: &str, name: &str, atmosphere: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            items: HashMap::new(),
            atmosphere: atmosphere.to_string(),
        }
    }

    /// Place an item in this room.
    pub fn place(&mut self, item_id: &str, description: &str) {
        self.items.insert(item_id.to_string(), description.to_string());
    }

    /// Retrieve an item by id.
    pub fn retrieve(&self, item_id: &str) -> Option<&str> {
        self.items.get(item_id).map(|s| s.as_str())
    }

    /// Remove an item from the room.
    pub fn remove_item(&mut self, item_id: &str) -> Option<String> {
        self.items.remove(item_id)
    }

    /// Number of items in the room.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Is the room empty?
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// List all item ids.
    pub fn item_ids(&self) -> Vec<&str> {
        self.items.keys().map(|s| s.as_str()).collect()
    }

    /// Search items by description substring.
    pub fn search(&self, query: &str) -> Vec<(&str, &str)> {
        self.items
            .iter()
            .filter(|(_, desc)| desc.contains(query))
            .map(|(id, desc)| (id.as_str(), desc.as_str()))
            .collect()
    }
}

// ── Module: corridor ─────────────────────────────────────────────────────

/// A connection between two rooms with a direction label.
#[derive(Debug, Clone)]
pub struct Corridor {
    pub from: String,
    pub to: String,
    pub direction: String,
    pub cue: String, // sensory cue for this transition
}

impl Corridor {
    /// Create a new corridor.
    pub fn new(from: &str, to: &str, direction: &str, cue: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            direction: direction.to_string(),
            cue: cue.to_string(),
        }
    }

    /// Does this corridor go from `room`?
    pub fn goes_from(&self, room: &str) -> bool {
        self.from == room
    }

    /// Does this corridor go to `room`?
    pub fn goes_to(&self, room: &str) -> bool {
        self.to == room
    }
}

/// A network of corridors connecting rooms.
#[derive(Debug, Clone)]
pub struct CorridorMap {
    corridors: Vec<Corridor>,
}

impl CorridorMap {
    /// Create an empty corridor map.
    pub fn new() -> Self {
        Self { corridors: Vec::new() }
    }

    /// Add a corridor.
    pub fn add(&mut self, corridor: Corridor) {
        self.corridors.push(corridor);
    }

    /// Get corridors from a room.
    pub fn from_room(&self, room: &str) -> Vec<&Corridor> {
        self.corridors.iter().filter(|c| c.from == room).collect()
    }

    /// Get corridors to a room.
    pub fn to_room(&self, room: &str) -> Vec<&Corridor> {
        self.corridors.iter().filter(|c| c.to == room).collect()
    }

    /// Number of corridors.
    pub fn len(&self) -> usize {
        self.corridors.len()
    }

    /// Is the map empty?
    pub fn is_empty(&self) -> bool {
        self.corridors.is_empty()
    }

    /// Find a path from start to goal using BFS.
    pub fn find_path(&self, start: &str, goal: &str) -> Vec<String> {
        if start == goal {
            return vec![start.to_string()];
        }
        let mut visited = HashSet::new();
        let mut queue = vec![(start.to_string(), vec![start.to_string()])];
        visited.insert(start.to_string());

        while let Some((current, path)) = queue.pop() {
            for corridor in self.from_room(&current) {
                if corridor.to == goal {
                    let mut result = path.clone();
                    result.push(goal.to_string());
                    return result;
                }
                if !visited.contains(&corridor.to) {
                    visited.insert(corridor.to.clone());
                    let mut new_path = path.clone();
                    new_path.push(corridor.to.clone());
                    queue.push((corridor.to.clone(), new_path));
                }
            }
        }
        vec![] // no path found
    }
}

impl Default for CorridorMap {
    fn default() -> Self {
        Self::new()
    }
}

// ── Module: anchor ───────────────────────────────────────────────────────

/// A memorable location binding that links a concept to a specific room/position.
#[derive(Debug, Clone)]
pub struct Anchor {
    pub concept: String,
    pub room_id: String,
    pub position: (f64, f64), // x, y coordinates in room
    pub strength: f64,        // memory strength (0.0-1.0)
    pub sensory_tags: Vec<String>,
}

impl Anchor {
    /// Create a new anchor.
    pub fn new(concept: &str, room_id: &str, x: f64, y: f64) -> Self {
        Self {
            concept: concept.to_string(),
            room_id: room_id.to_string(),
            position: (x, y),
            strength: 1.0,
            sensory_tags: Vec::new(),
        }
    }

    /// Add a sensory tag.
    pub fn add_tag(&mut self, tag: &str) {
        if !self.sensory_tags.contains(&tag.to_string()) {
            self.sensory_tags.push(tag.to_string());
        }
    }

    /// Decay the anchor strength.
    pub fn decay(&mut self, factor: f64) {
        self.strength *= factor;
        if self.strength < 0.01 {
            self.strength = 0.0;
        }
    }

    /// Strengthen the anchor (e.g., on recall).
    pub fn strengthen(&mut self, amount: f64) {
        self.strength = (self.strength + amount).min(1.0);
    }

    /// Distance to another anchor.
    pub fn distance_to(&self, other: &Anchor) -> f64 {
        let dx = self.position.0 - other.position.0;
        let dy = self.position.1 - other.position.1;
        (dx * dx + dy * dy).sqrt()
    }

    /// Is the anchor still memorable?
    pub fn is_memorable(&self) -> bool {
        self.strength >= 0.1
    }
}

/// Collection of anchors.
#[derive(Debug, Clone)]
pub struct AnchorMap {
    anchors: HashMap<String, Anchor>, // concept -> anchor
}

impl AnchorMap {
    /// Create an empty anchor map.
    pub fn new() -> Self {
        Self { anchors: HashMap::new() }
    }

    /// Place an anchor.
    pub fn place(&mut self, anchor: Anchor) {
        self.anchors.insert(anchor.concept.clone(), anchor);
    }

    /// Recall an anchor by concept.
    pub fn recall(&mut self, concept: &str) -> Option<&Anchor> {
        if let Some(anchor) = self.anchors.get_mut(concept) {
            anchor.strengthen(0.1);
        }
        self.anchors.get(concept)
    }

    /// Get anchors in a specific room.
    pub fn in_room(&self, room_id: &str) -> Vec<&Anchor> {
        self.anchors.values().filter(|a| a.room_id == room_id).collect()
    }

    /// Decay all anchors.
    pub fn decay_all(&mut self, factor: f64) {
        for anchor in self.anchors.values_mut() {
            anchor.decay(factor);
        }
    }

    /// Remove forgotten anchors (strength < 0.1).
    pub fn prune_forgotten(&mut self) -> usize {
        let before = self.anchors.len();
        self.anchors.retain(|_, a| a.is_memorable());
        before - self.anchors.len()
    }

    /// Number of anchors.
    pub fn len(&self) -> usize {
        self.anchors.len()
    }

    /// Is the map empty?
    pub fn is_empty(&self) -> bool {
        self.anchors.is_empty()
    }

    /// Find nearest anchor to a position in a room.
    pub fn nearest(&self, room_id: &str, pos: (f64, f64)) -> Option<&Anchor> {
        self.anchors
            .values()
            .filter(|a| a.room_id == room_id)
            .min_by(|a, b| {
                let da = (a.position.0 - pos.0).powi(2) + (a.position.1 - pos.1).powi(2);
                let db = (b.position.0 - pos.0).powi(2) + (b.position.1 - pos.1).powi(2);
                da.partial_cmp(&db).unwrap()
            })
    }
}

impl Default for AnchorMap {
    fn default() -> Self {
        Self::new()
    }
}

// ── Module: walk ─────────────────────────────────────────────────────────

/// A path through the memory palace for sequential recall.
#[derive(Debug, Clone)]
pub struct Walk {
    pub steps: Vec<WalkStep>,
    pub current_index: usize,
}

/// A single step in a walk.
#[derive(Debug, Clone)]
pub struct WalkStep {
    pub room_id: String,
    pub action: WalkAction,
}

/// Action at a walk step.
#[derive(Debug, Clone)]
pub enum WalkAction {
    Enter(String),        // enter room with direction
    Observe(String),      // observe an item
    MoveTo((f64, f64)),   // move to position
    Recall(String),       // recall a concept
}

impl Walk {
    /// Create a new walk from steps.
    pub fn new(steps: Vec<WalkStep>) -> Self {
        Self { steps, current_index: 0 }
    }

    /// Current step.
    pub fn current(&self) -> Option<&WalkStep> {
        self.steps.get(self.current_index)
    }

    /// Advance to the next step.
    pub fn advance(&mut self) -> Option<&WalkStep> {
        if self.current_index < self.steps.len() - 1 {
            self.current_index += 1;
            Some(&self.steps[self.current_index])
        } else {
            None
        }
    }

    /// Go back one step.
    pub fn go_back(&mut self) -> Option<&WalkStep> {
        if self.current_index > 0 {
            self.current_index -= 1;
            Some(&self.steps[self.current_index])
        } else {
            None
        }
    }

    /// Total steps in the walk.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Is the walk empty?
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Is the walk complete?
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.steps.len() - 1
    }

    /// Reset to the beginning.
    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    /// Extract all recalled concepts from the walk.
    pub fn recalled_concepts(&self) -> Vec<&str> {
        self.steps
            .iter()
            .filter_map(|s| match &s.action {
                WalkAction::Recall(c) => Some(c.as_str()),
                _ => None,
            })
            .collect()
    }

    /// Rooms visited in order.
    pub fn rooms_visited(&self) -> Vec<&str> {
        self.steps.iter().map(|s| s.room_id.as_str()).collect()
    }
}

/// Build a walk from a path of room IDs using a corridor map.
pub fn build_walk(path: &[String], corridors: &CorridorMap) -> Walk {
    let mut steps = Vec::new();
    for (i, room_id) in path.iter().enumerate() {
        if i == 0 {
            steps.push(WalkStep {
                room_id: room_id.clone(),
                action: WalkAction::Enter("start".into()),
            });
        } else {
            let direction = corridors
                .from_room(&path[i - 1])
                .into_iter()
                .find(|c| c.to == *room_id)
                .map(|c| c.direction.clone())
                .unwrap_or_else(|| "forward".into());
            steps.push(WalkStep {
                room_id: room_id.clone(),
                action: WalkAction::Enter(direction),
            });
        }
    }
    Walk::new(steps)
}

// ── Module: construction ─────────────────────────────────────────────────

/// Builder for constructing memory palaces from data.
#[derive(Debug, Clone)]
pub struct PalaceBuilder {
    rooms: HashMap<String, Room>,
    corridors: CorridorMap,
    anchors: AnchorMap,
}

impl PalaceBuilder {
    /// Create a new palace builder.
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            corridors: CorridorMap::new(),
            anchors: AnchorMap::new(),
        }
    }

    /// Add a room.
    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.id.clone(), room);
    }

    /// Connect two rooms.
    pub fn connect(&mut self, from: &str, to: &str, direction: &str, cue: &str) {
        self.corridors.add(Corridor::new(from, to, direction, cue));
    }

    /// Anchor a concept to a room.
    pub fn anchor(&mut self, concept: &str, room_id: &str, x: f64, y: f64) {
        self.anchors.place(Anchor::new(concept, room_id, x, y));
    }

    /// Build the completed palace.
    pub fn build(self) -> Palace {
        Palace {
            rooms: self.rooms,
            corridors: self.corridors,
            anchors: self.anchors,
        }
    }

    /// Number of rooms planned.
    pub fn num_rooms(&self) -> usize {
        self.rooms.len()
    }

    /// Number of corridors planned.
    pub fn num_corridors(&self) -> usize {
        self.corridors.len()
    }
}

impl Default for PalaceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// A completed memory palace.
#[derive(Debug, Clone)]
pub struct Palace {
    pub rooms: HashMap<String, Room>,
    pub corridors: CorridorMap,
    pub anchors: AnchorMap,
}

impl Palace {
    /// Get a room by id.
    pub fn get_room(&self, id: &str) -> Option<&Room> {
        self.rooms.get(id)
    }

    /// Get a mutable room by id.
    pub fn get_room_mut(&mut self, id: &str) -> Option<&mut Room> {
        self.rooms.get_mut(id)
    }

    /// Number of rooms.
    pub fn num_rooms(&self) -> usize {
        self.rooms.len()
    }

    /// Find a path between two rooms.
    pub fn find_path(&self, from: &str, to: &str) -> Vec<String> {
        self.corridors.find_path(from, to)
    }

    /// Recall a concept, returning the room and anchor info.
    pub fn recall(&mut self, concept: &str) -> Option<RecallResult> {
        let anchor = self.anchors.recall(concept)?;
        let room = self.rooms.get(&anchor.room_id)?;
        Some(RecallResult {
            concept: concept.to_string(),
            room_name: room.name.clone(),
            room_atmosphere: room.atmosphere.clone(),
            position: anchor.position,
            strength: anchor.strength,
        })
    }
}

/// Result of recalling a concept.
#[derive(Debug, Clone)]
pub struct RecallResult {
    pub concept: String,
    pub room_name: String,
    pub room_atmosphere: String,
    pub position: (f64, f64),
    pub strength: f64,
}

/// Build a palace from pairs of (concept, description).
pub fn build_from_pairs(pairs: &[(String, String)], room_prefix: &str) -> Palace {
    let mut builder = PalaceBuilder::new();
    let room_capacity = 5;
    let num_rooms = pairs.len().div_ceil(room_capacity);

    for i in 0..num_rooms {
        let room_id = format!("{}_{}", room_prefix, i);
        let room = Room::new(&room_id, &format!("Room {}", i), "neutral");
        builder.add_room(room);
        if i > 0 {
            let prev_id = format!("{}_{}", room_prefix, i - 1);
            builder.connect(&prev_id, &room_id, "forward", "doorway");
        }
    }

    for (idx, (concept, desc)) in pairs.iter().enumerate() {
        let room_idx = idx / room_capacity;
        let pos_idx = idx % room_capacity;
        let room_id = format!("{}_{}", room_prefix, room_idx);
        if let Some(room) = builder.rooms.get_mut(&room_id) {
            room.place(concept, desc);
        }
        let x = (pos_idx as f64) * 2.0;
        let y = 0.0;
        builder.anchor(concept, &room_id, x, y);
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Room tests ──

    #[test]
    fn test_room_new() {
        let r = Room::new("r1", "Kitchen", "warm and bright");
        assert_eq!(r.id, "r1");
        assert_eq!(r.name, "Kitchen");
        assert!(r.is_empty());
    }

    #[test]
    fn test_room_place_retrieve() {
        let mut r = Room::new("r1", "Kitchen", "warm");
        r.place("key", "golden key on the counter");
        assert_eq!(r.retrieve("key"), Some("golden key on the counter"));
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn test_room_remove_item() {
        let mut r = Room::new("r1", "Kitchen", "warm");
        r.place("key", "golden key");
        let removed = r.remove_item("key");
        assert_eq!(removed, Some("golden key".into()));
        assert!(r.is_empty());
    }

    #[test]
    fn test_room_search() {
        let mut r = Room::new("r1", "Library", "quiet");
        r.place("b1", "red book on shelf");
        r.place("b2", "blue book on table");
        r.place("b3", "red cup on desk");
        let results = r.search("red");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_room_item_ids() {
        let mut r = Room::new("r1", "Room", "neutral");
        r.place("a", "item a");
        r.place("b", "item b");
        assert_eq!(r.item_ids().len(), 2);
    }

    // ── Corridor tests ──

    #[test]
    fn test_corridor_new() {
        let c = Corridor::new("r1", "r2", "north", "dark doorway");
        assert!(c.goes_from("r1"));
        assert!(c.goes_to("r2"));
        assert!(!c.goes_from("r2"));
    }

    #[test]
    fn test_corridor_map_add() {
        let mut cm = CorridorMap::new();
        cm.add(Corridor::new("r1", "r2", "north", "door"));
        assert_eq!(cm.len(), 1);
    }

    #[test]
    fn test_corridor_map_from_room() {
        let mut cm = CorridorMap::new();
        cm.add(Corridor::new("r1", "r2", "north", "door"));
        cm.add(Corridor::new("r1", "r3", "east", "arch"));
        assert_eq!(cm.from_room("r1").len(), 2);
        assert_eq!(cm.from_room("r2").len(), 0);
    }

    #[test]
    fn test_corridor_map_find_path_direct() {
        let mut cm = CorridorMap::new();
        cm.add(Corridor::new("r1", "r2", "north", "door"));
        let path = cm.find_path("r1", "r2");
        assert_eq!(path, vec!["r1", "r2"]);
    }

    #[test]
    fn test_corridor_map_find_path_multi() {
        let mut cm = CorridorMap::new();
        cm.add(Corridor::new("r1", "r2", "n", "d"));
        cm.add(Corridor::new("r2", "r3", "e", "a"));
        let path = cm.find_path("r1", "r3");
        assert_eq!(path, vec!["r1", "r2", "r3"]);
    }

    #[test]
    fn test_corridor_map_no_path() {
        let mut cm = CorridorMap::new();
        cm.add(Corridor::new("r1", "r2", "n", "d"));
        let path = cm.find_path("r1", "r4");
        assert!(path.is_empty());
    }

    #[test]
    fn test_corridor_map_same_room() {
        let cm = CorridorMap::new();
        let path = cm.find_path("r1", "r1");
        assert_eq!(path, vec!["r1"]);
    }

    #[test]
    fn test_corridor_map_default() {
        let cm = CorridorMap::default();
        assert!(cm.is_empty());
    }

    // ── Anchor tests ──

    #[test]
    fn test_anchor_new() {
        let a = Anchor::new("gravity", "physics_room", 1.0, 2.0);
        assert_eq!(a.concept, "gravity");
        assert_eq!(a.room_id, "physics_room");
        assert!((a.strength - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_anchor_tags() {
        let mut a = Anchor::new("fire", "r1", 0.0, 0.0);
        a.add_tag("warm");
        a.add_tag("red");
        a.add_tag("warm"); // duplicate
        assert_eq!(a.sensory_tags.len(), 2);
    }

    #[test]
    fn test_anchor_decay() {
        let mut a = Anchor::new("x", "r1", 0.0, 0.0);
        a.decay(0.5);
        assert!((a.strength - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_anchor_strengthen() {
        let mut a = Anchor::new("x", "r1", 0.0, 0.0);
        a.strength = 0.5;
        a.strengthen(0.3);
        assert!((a.strength - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_anchor_strengthen_cap() {
        let mut a = Anchor::new("x", "r1", 0.0, 0.0);
        a.strengthen(5.0);
        assert!((a.strength - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_anchor_distance() {
        let a1 = Anchor::new("a", "r1", 0.0, 0.0);
        let a2 = Anchor::new("b", "r1", 3.0, 4.0);
        assert!((a1.distance_to(&a2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_anchor_memorable() {
        let mut a = Anchor::new("x", "r1", 0.0, 0.0);
        assert!(a.is_memorable());
        a.strength = 0.05;
        assert!(!a.is_memorable());
    }

    #[test]
    fn test_anchor_map_place_recall() {
        let mut am = AnchorMap::new();
        am.place(Anchor::new("gravity", "physics", 1.0, 1.0));
        let a = am.recall("gravity").unwrap();
        assert_eq!(a.room_id, "physics");
    }

    #[test]
    fn test_anchor_map_decay_all() {
        let mut am = AnchorMap::new();
        am.place(Anchor::new("a", "r1", 0.0, 0.0));
        am.place(Anchor::new("b", "r1", 0.0, 0.0));
        am.decay_all(0.5);
        assert_eq!(am.len(), 2);
    }

    #[test]
    fn test_anchor_map_prune() {
        let mut am = AnchorMap::new();
        let mut a1 = Anchor::new("a", "r1", 0.0, 0.0);
        a1.strength = 0.5;
        let mut a2 = Anchor::new("b", "r1", 0.0, 0.0);
        a2.strength = 0.01;
        am.place(a1);
        am.place(a2);
        let pruned = am.prune_forgotten();
        assert_eq!(pruned, 1);
        assert_eq!(am.len(), 1);
    }

    #[test]
    fn test_anchor_map_nearest() {
        let mut am = AnchorMap::new();
        am.place(Anchor::new("a", "r1", 1.0, 1.0));
        am.place(Anchor::new("b", "r1", 10.0, 10.0));
        let nearest = am.nearest("r1", (2.0, 2.0)).unwrap();
        assert_eq!(nearest.concept, "a");
    }

    #[test]
    fn test_anchor_map_in_room() {
        let mut am = AnchorMap::new();
        am.place(Anchor::new("a", "r1", 0.0, 0.0));
        am.place(Anchor::new("b", "r2", 0.0, 0.0));
        assert_eq!(am.in_room("r1").len(), 1);
    }

    #[test]
    fn test_anchor_map_default() {
        let am = AnchorMap::default();
        assert!(am.is_empty());
    }

    // ── Walk tests ──

    #[test]
    fn test_walk_new() {
        let w = Walk::new(vec![]);
        assert!(w.is_empty());
    }

    #[test]
    fn test_walk_advance() {
        let w = Walk::new(vec![
            WalkStep { room_id: "r1".into(), action: WalkAction::Enter("start".into()) },
            WalkStep { room_id: "r2".into(), action: WalkAction::Enter("north".into()) },
        ]);
        let mut w = w;
        assert_eq!(w.current().unwrap().room_id, "r1");
        w.advance();
        assert_eq!(w.current().unwrap().room_id, "r2");
    }

    #[test]
    fn test_walk_go_back() {
        let mut w = Walk::new(vec![
            WalkStep { room_id: "r1".into(), action: WalkAction::Enter("start".into()) },
            WalkStep { room_id: "r2".into(), action: WalkAction::Enter("north".into()) },
        ]);
        w.advance();
        w.go_back();
        assert_eq!(w.current().unwrap().room_id, "r1");
    }

    #[test]
    fn test_walk_complete() {
        let mut w = Walk::new(vec![
            WalkStep { room_id: "r1".into(), action: WalkAction::Enter("start".into()) },
        ]);
        assert!(w.is_complete());
    }

    #[test]
    fn test_walk_reset() {
        let mut w = Walk::new(vec![
            WalkStep { room_id: "r1".into(), action: WalkAction::Enter("start".into()) },
            WalkStep { room_id: "r2".into(), action: WalkAction::Enter("n".into()) },
        ]);
        w.advance();
        w.reset();
        assert_eq!(w.current().unwrap().room_id, "r1");
    }

    #[test]
    fn test_walk_recalled_concepts() {
        let w = Walk::new(vec![
            WalkStep { room_id: "r1".into(), action: WalkAction::Enter("start".into()) },
            WalkStep { room_id: "r1".into(), action: WalkAction::Recall("gravity".into()) },
            WalkStep { room_id: "r2".into(), action: WalkAction::Recall("light".into()) },
        ]);
        assert_eq!(w.recalled_concepts(), vec!["gravity", "light"]);
    }

    #[test]
    fn test_walk_rooms_visited() {
        let w = Walk::new(vec![
            WalkStep { room_id: "r1".into(), action: WalkAction::Enter("start".into()) },
            WalkStep { room_id: "r2".into(), action: WalkAction::Enter("n".into()) },
        ]);
        assert_eq!(w.rooms_visited(), vec!["r1", "r2"]);
    }

    #[test]
    fn test_build_walk() {
        let mut cm = CorridorMap::new();
        cm.add(Corridor::new("r1", "r2", "north", "door"));
        cm.add(Corridor::new("r2", "r3", "east", "arch"));
        let walk = build_walk(&["r1".into(), "r2".into(), "r3".into()], &cm);
        assert_eq!(walk.len(), 3);
    }

    // ── Construction tests ──

    #[test]
    fn test_builder_new() {
        let b = PalaceBuilder::new();
        assert_eq!(b.num_rooms(), 0);
    }

    #[test]
    fn test_builder_add_room() {
        let mut b = PalaceBuilder::new();
        b.add_room(Room::new("r1", "Kitchen", "warm"));
        assert_eq!(b.num_rooms(), 1);
    }

    #[test]
    fn test_builder_connect() {
        let mut b = PalaceBuilder::new();
        b.add_room(Room::new("r1", "A", "x"));
        b.add_room(Room::new("r2", "B", "y"));
        b.connect("r1", "r2", "north", "door");
        assert_eq!(b.num_corridors(), 1);
    }

    #[test]
    fn test_builder_build() {
        let mut b = PalaceBuilder::new();
        b.add_room(Room::new("r1", "Kitchen", "warm"));
        let palace = b.build();
        assert_eq!(palace.num_rooms(), 1);
    }

    #[test]
    fn test_palace_get_room() {
        let mut b = PalaceBuilder::new();
        b.add_room(Room::new("r1", "Kitchen", "warm"));
        let palace = b.build();
        assert!(palace.get_room("r1").is_some());
        assert!(palace.get_room("r99").is_none());
    }

    #[test]
    fn test_palace_recall() {
        let mut b = PalaceBuilder::new();
        b.add_room(Room::new("r1", "Physics", "bright"));
        b.anchor("gravity", "r1", 1.0, 1.0);
        let mut palace = b.build();
        let result = palace.recall("gravity").unwrap();
        assert_eq!(result.room_name, "Physics");
    }

    #[test]
    fn test_build_from_pairs() {
        let pairs = vec![
            ("a".into(), "alpha".into()),
            ("b".into(), "beta".into()),
            ("c".into(), "gamma".into()),
            ("d".into(), "delta".into()),
            ("e".into(), "epsilon".into()),
            ("f".into(), "zeta".into()),
        ];
        let palace = build_from_pairs(&pairs, "test");
        assert_eq!(palace.num_rooms(), 2); // 6 items, 5 per room = 2 rooms
        assert_eq!(palace.find_path("test_0", "test_1").len(), 2);
    }

    #[test]
    fn test_build_from_pairs_empty() {
        let palace = build_from_pairs(&[], "empty");
        assert_eq!(palace.num_rooms(), 0);
    }

    #[test]
    fn test_palace_get_room_mut() {
        let mut b = PalaceBuilder::new();
        b.add_room(Room::new("r1", "Kitchen", "warm"));
        let mut palace = b.build();
        let room = palace.get_room_mut("r1").unwrap();
        room.place("key", "golden key");
        assert_eq!(palace.get_room("r1").unwrap().len(), 1);
    }

    #[test]
    fn test_palace_find_path_no_connection() {
        let mut b = PalaceBuilder::new();
        b.add_room(Room::new("r1", "A", "x"));
        b.add_room(Room::new("r2", "B", "y"));
        let palace = b.build();
        assert!(palace.find_path("r1", "r2").is_empty());
    }

    #[test]
    fn test_palace_recall_nonexistent() {
        let mut b = PalaceBuilder::new();
        b.add_room(Room::new("r1", "Room", "x"));
        let mut palace = b.build();
        assert!(palace.recall("nonexistent").is_none());
    }

    #[test]
    fn test_corridor_to_room() {
        let mut cm = CorridorMap::new();
        cm.add(Corridor::new("r1", "r2", "n", "d"));
        assert_eq!(cm.to_room("r2").len(), 1);
        assert_eq!(cm.to_room("r1").len(), 0);
    }
}
