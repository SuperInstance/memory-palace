# Memory Palace

[![crates.io](https://img.shields.io/crates/v/memory-palace.svg)](https://crates.io/crates/memory-palace)
[![docs.rs](https://docs.rs/memory-palace/badge.svg)](https://docs.rs/memory-palace)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Method of loci memory indexing for agents — rooms, corridors, anchors, and spatial recall.**

---

## The Problem

Agents need to remember things, but flat key-value stores don't match how memory actually works. Humans use spatial memory — the **method of loci** — to store and recall information by associating concepts with physical locations. This is one of the oldest and most powerful mnemonic techniques, dating back to ancient Greece.

## Why This Exists

Flat memory systems force linear scanning or hash lookups. The Memory Palace crate implements a **spatial memory architecture** where:
- Memories live in themed **rooms** with sensory atmospheres
- Rooms connect via **corridors** with directional cues
- Concepts are **anchored** to specific positions with strength decay
- **Walks** through the palace enable sequential recall
- BFS pathfinding connects distant memories

This creates a memory system where recall is *navigational* — you walk to where you stored something, and the spatial context helps you retrieve it.

## Architecture

```
     ┌──────────┐                    ┌──────────┐
     │ Kitchen  │──── doorway ───────│ Library  │
     │ (warm)   │                    │ (quiet)  │
     │          │                    │          │
     │ 🔑 key   │                    │ 📕 red   │
     │ 🪙 coin  │                    │ 📘 blue  │
     └──────────┘                    └──────────┘
          │                               │
        arch                            arch
          │                               │
     ┌──────────┐                    ┌──────────┐
     │  Garden  │──── gate ──────────│  Tower   │
     │ (bright) │                    │ (windy)  │
     │ 🌸 rose  │                    │ 🔭 star  │
     └──────────┘                    └──────────┘

  Anchors: concept → (room, x, y, strength, tags)
  Corridors: room ↔ room with direction + cue
  Walks: ordered sequence of rooms for recall
```

## Installation

```toml
[dependencies]
memory-palace = "0.1"
```

## API Reference

### `Room`

A spatial memory container with sensory atmosphere:

```rust
use memory_palace::Room;

let mut kitchen = Room::new("kitchen", "Kitchen", "warm and bright");
kitchen.place("key", "golden key on the counter");
kitchen.place("coin", "silver coin near the sink");

assert_eq!(kitchen.retrieve("key"), Some("golden key on the counter"));
assert_eq!(kitchen.len(), 2);

// Search by description
let results = kitchen.search("silver");
assert_eq!(results.len(), 1);
```

### `Corridor` & `CorridorMap`

Connections between rooms with BFS pathfinding:

```rust
use memory_palace::{Corridor, CorridorMap};

let mut map = CorridorMap::new();
map.add(Corridor::new("kitchen", "library", "north", "dark doorway"));
map.add(Corridor::new("library", "tower", "upstairs", "spiral stairs"));

// Find path from kitchen to tower
let path = map.find_path("kitchen", "tower");
assert_eq!(path, vec!["kitchen", "library", "tower"]);
```

### `Anchor` & `AnchorMap`

Memory anchors with strength decay and spatial proximity:

```rust
use memory_palace::{Anchor, AnchorMap};

let mut anchors = AnchorMap::new();
let mut gravity = Anchor::new("gravity", "physics-room", 1.0, 2.0);
gravity.add_tag("warm");
gravity.add_tag("heavy");

anchors.place(gravity);

// Recall strengthens the anchor
let a = anchors.recall("gravity").unwrap();
assert_eq!(a.room_id, "physics-room");

// Decay over time (forgetting)
anchors.decay_all(0.8); // 20% decay
anchors.prune_forgotten(); // remove < 0.1 strength
```

### `Walk`

Sequential path through the palace for ordered recall:

```rust
use memory_palace::{Walk, WalkStep, WalkAction};

let walk = Walk::new(vec![
    WalkStep { room_id: "kitchen".into(), action: WalkAction::Enter("start".into()) },
    WalkStep { room_id: "kitchen".into(), action: WalkAction::Observe("golden key".into()) },
    WalkStep { room_id: "library".into(), action: WalkAction::Enter("north".into()) },
    WalkStep { room_id: "library".into(), action: WalkAction::Recall("physics formula".into()) },
]);

let concepts = walk.recalled_concepts();
assert_eq!(concepts, vec!["physics formula"]);
```

### `PalaceBuilder` & `Palace`

Build complete memory palaces:

```rust
use memory_palace::*;

let mut builder = PalaceBuilder::new();
builder.add_room(Room::new("r1", "Entrance Hall", "marble and echoes"));
builder.add_room(Room::new("r2", "Study", "leather and ink"));
builder.connect("r1", "r2", "north", "heavy oak door");
builder.anchor("gravity", "r2", 3.0, 1.0);

let mut palace = builder.build();
let result = palace.recall("gravity").unwrap();
assert_eq!(result.room_name, "Study");
```

### `build_from_pairs`

Quick construction from concept-description pairs:

```rust
use memory_palace::build_from_pairs;

let pairs = vec![
    ("gravity".into(), "F = Gm₁m₂/r²".into()),
    ("light".into(), "E = mc²".into()),
    ("entropy".into(), "S = k·ln(W)".into()),
];
let palace = build_from_pairs(&pairs, "physics");
```

## Usage Examples

### Example 1: Building a Learning Palace

```rust
use memory_palace::*;

let mut builder = PalaceBuilder::new();

// Create themed rooms for different subjects
builder.add_room(Room::new("math", "Mathematics Room", "clean whiteboards"));
builder.add_room(Room::new("physics", "Physics Lab", "humming equipment"));
builder.add_room(Room::new("history", "History Archive", "dusty scrolls"));

// Connect rooms
builder.connect("math", "physics", "east", "shared equations on wall");
builder.connect("physics", "history", "south", "timeline corridor");

// Place knowledge anchors
builder.anchor("euler", "math", 1.0, 1.0);
builder.anchor("thermodynamics", "physics", 2.0, 3.0);
builder.anchor("renaissance", "history", 0.5, 2.0);

let palace = builder.build();
```

### Example 2: Memory Decay and Reinforcement

```rust
use memory_palace::*;

let mut anchors = AnchorMap::new();
anchors.place(Anchor::new("trivia", "lounge", 1.0, 1.0));

// Time passes — memory fades
anchors.decay_all(0.5);
anchors.decay_all(0.5);
// strength is now 0.25

// Recalling strengthens it back
let _ = anchors.recall("trivia");
// strength is now 0.35 (+0.1 from recall)
```

### Example 3: Path-Based Recall Walk

```rust
use memory_palace::*;

let mut cm = CorridorMap::new();
cm.add(Corridor::new("entrance", "hall", "forward", "grand door"));
cm.add(Corridor::new("hall", "study", "left", "narrow passage"));

let path = vec!["entrance".into(), "hall".into(), "study".into()];
let walk = build_walk(&path, &cm);

// Walk through the palace step by step
let mut walk = walk;
while let Some(step) = walk.advance() {
    println!("Entered: {} via {:?}", step.room_id, step.action);
}
```

## Mathematical Background

**Memory Strength Decay** follows exponential decay:

```
S(t) = S₀ × f^n
```

Where `f` is the decay factor (0.0-1.0) and `n` is the number of decay cycles.

**Euclidean Distance** between anchors:

```
d = √((x₁-x₂)² + (y₁-y₂)²)
```

**BFS Pathfinding** through corridors: O(V + E) where V = rooms, E = corridors.

The **method of loci** exploits the brain's spatial navigation system (hippocampal place cells) to create durable, retrieveable memory associations. This crate models that same principle for artificial agents.

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Room `place`/`retrieve` | O(1) | HashMap-backed |
| Room `search` | O(n) | Linear scan of items |
| Corridor BFS pathfinding | O(V + E) | Standard BFS |
| Anchor `recall` | O(1) | HashMap + strengthen |
| Anchor `decay_all` | O(n) | Single pass |
| Anchor `prune_forgotten` | O(n) | Retain filter |
| `build_from_pairs` | O(n) | Linear construction |

## Comparison with Alternatives

| Feature | memory-palace | HashMap | Redis |
|---------|-------------|---------|-------|
| Spatial organization | ✅ Rooms/positions | ❌ Flat | ❌ Flat |
| Navigational recall | ✅ Walks/paths | ❌ | ❌ |
| Memory decay | ✅ Strength-based | ❌ Manual TTL | ✅ TTL |
| Sensory cues | ✅ Atmospheres/tags | ❌ | ❌ |
| BFS pathfinding | ✅ Native | ❌ | ❌ |
| Zero dependencies | ✅ | ✅ | ❌ |

## License

Licensed under the [MIT License](LICENSE).

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests for your changes
4. Commit with conventional commits
5. Push and open a Pull Request
