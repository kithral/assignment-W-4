# Project Vision: Programmable World Engine

## Overview

Assignment W-4 is evolving into a **programmable multiplayer world engine** that combines:

- The **programmability** of TinyMUSH/MOO servers
- The **visual richness** of modern game engines
- The **performance** of Rust and Bevy ECS
- The **flexibility** of user-defined content

## Inspiration Sources

### From TinyMUSH/MOO

- **In-world programming** - Players/admins code features without recompiling
- **Soft-code flexibility** - Game logic exists in the database, not the binary
- **Permission system** - Fine-grained control over what users can do
- **Persistent objects** - Everything is an object with attributes
- **Text-based protocol** - Simple, accessible interface

### From EVE Online

- **Server authority** - Server is the source of truth
- **Client prediction** - Responsive gameplay despite latency
- **Scalable architecture** - Handle many concurrent players
- **Complex systems** - Emergent gameplay from simple rules

### From Modern Game Engines

- **ECS architecture** - Data-oriented design for performance
- **Visual rendering** - 3D graphics for immersion
- **Real-time updates** - Smooth, responsive gameplay
- **Asset pipeline** - Support for custom art and models

## Core Principles

### 1. Content Agnosticism

The engine should not care what kind of world it's running:

- **Not** a "space game" engine
- **Not** a "fantasy RPG" engine
- **Is** a programmable platform for any genre

**Examples:**

- A medieval fantasy kingdom
- A cyberpunk city
- A space station
- A modern-day social hub
- A puzzle game world
- An educational simulation

### 2. Dual-Interface Design

**Graphical Client:**

```
Player → Graphical Client (Bevy) → UDP → Server
         ├─ 3D rendering
         ├─ Client-side prediction
         ├─ Visual feedback
         └─ Mouse/keyboard input
```

**Text Client:**

```
Player → Telnet/SSH Client → TCP → Server
         ├─ Text descriptions
         ├─ Command-line interface
         ├─ Classic MUD/MUSH feel
         └─ Accessible anywhere
```

Both interfaces interact with the **same world state**.

### 3. Programmable Everything

**In-World Scripting Language:**

```rust
// Example: Admin creates a custom door object
@create Magic Door
@set Magic Door/script = {
    on_examine(player) {
        if player.has_key("ancient_key") {
            emit("The door glows with recognition.");
        } else {
            emit("Ancient runes warn: 'Only the worthy may pass.'");
        }
    }

    on_use(player) {
        if player.has_key("ancient_key") {
            teleport(player, "secret_chamber");
            emit_room("The door swings open with a creak.");
        } else {
            damage(player, 10);
            emit("The door shocks you for " + 10 + " damage!");
        }
    }
}
```

**No Recompilation Needed:**

- Admins write code in-game
- Changes take effect immediately
- Safe sandboxing prevents server crashes
- Version control for scripts in database

### 4. Standardized Language, Custom Worlds

**What's Standardized:**

- Scripting language syntax
- Core object model (everything is an entity)
- Permission system
- Network protocol
- Client-server API

**What's Customizable:**

- All game rules
- All art assets
- All commands and verbs
- World layout and structure
- Economy and progression
- Combat or non-combat systems

## Technical Architecture

### Server Components

```
┌─────────────────────────────────────────────┐
│              Server Core                     │
├─────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │   ECS    │  │ Scripting│  │  Network │  │
│  │  Engine  │  │   VM     │  │  Layer   │  │
│  └──────────┘  └──────────┘  └──────────┘  │
├─────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  │
│  │   Auth   │  │ Persistence│ │  Events  │  │
│  │  System  │  │  (DB)     │  │  System  │  │
│  └──────────┘  └──────────┘  └──────────┘  │
└─────────────────────────────────────────────┘
         ↓              ↓              ↓
    ┌────────┐    ┌────────┐    ┌────────┐
    │Graphical│    │  Text  │    │  HTTP  │
    │ Client │    │ Client │    │  API   │
    └────────┘    └────────┘    └────────┘
```

### Entity Component System

Everything is an entity with components:

```rust
// A player character
Entity {
    components: [
        Position(x: 10.0, y: 5.0, z: 0.0),
        Renderable(model: "humanoid_base"),
        Scriptable(scripts: ["on_login", "on_chat"]),
        Inventory(items: [sword_entity, potion_entity]),
        Permissions(level: PLAYER),
        NetworkId(connection: tcp_conn_123),
    ]
}

// A custom door (created by admin script)
Entity {
    components: [
        Position(x: 15.0, y: 10.0, z: 0.0),
        Renderable(model: "custom_door"),
        Scriptable(scripts: ["on_examine", "on_use"]),
        Attributes(locked: true, key_id: "ancient_key"),
        Permissions(level: PUBLIC),
    ]
}
```

### Scripting Language Design

**Goals:**

- **Safe:** Can't crash the server
- **Fast:** Compiled or JIT'd, not interpreted
- **Familiar:** Rust-like or Lua-like syntax
- **Sandboxed:** Limited API access
- **Interruptible:** Can timeout long-running scripts

**Candidate Languages:**

1. **Rhai** (Rust scripting language)
   - Pros: Rust-like syntax, native integration, sandboxed
   - Cons: Less mature ecosystem

2. **Lua** (via mlua)
   - Pros: Mature, fast (LuaJIT), familiar to many
   - Cons: Different syntax from Rust

3. **Custom DSL**
   - Pros: Tailored exactly to our needs
   - Cons: More implementation work, learning curve

**Recommendation:** Start with **Rhai** for Rust familiarity, evaluate others later.

## User Workflows

### World Builder Workflow

```bash
# 1. Connect to server
telnet game.example.com 4201

# 2. Login as admin
login wizard secretpassword

# 3. Create a new area
@dig "Enchanted Forest"
@teleport "Enchanted Forest"

# 4. Create interactive objects
@create "Ancient Tree"
@set Ancient Tree/description = "A massive oak with glowing runes."
@set Ancient Tree/script = {
    on_examine(player) {
        if random() > 0.5 {
            emit("A fairy appears briefly!");
            spawn_entity("fairy", nearby(player));
        }
    }
}

# 5. Set permissions
@chmod Ancient Tree = PUBLIC

# 6. Test in graphical client
# (Switch to graphical client, see the tree rendered in 3D)
```

### Player Workflow

**Text Client:**

```
> look
Enchanted Forest
A mystical grove bathed in ethereal light. Ancient trees tower above.
Exits: north, south, east
Objects: Ancient Tree, Fairy Circle

> examine tree
A massive oak with glowing runes.
[A fairy appears briefly!]

> inventory
You are carrying: Rusty Sword, Health Potion, 50 gold
```

**Graphical Client:**

- See 3D rendered forest
- Click on Ancient Tree to examine
- See fairy spawn animation
- Move with WASD keys
- Same underlying actions as text client

## Development Phases

### Phase 1: Foundation (Current)

- ✅ Rust workspace structure
- ✅ Bevy ECS integration
- ✅ Lightyear networking basics
- ✅ Development tooling
- 🔲 Basic entity/component system

### Phase 2: Dual Protocol (Near-term)

- 🔲 Text-based protocol (Telnet)
- 🔲 Command parser
- 🔲 Room/object system
- 🔲 Basic persistence (SQLite)

### Phase 3: Scripting (Mid-term)

- 🔲 Embedded scripting language (Rhai)
- 🔲 Sandboxing and safety
- 🔲 Script API design
- 🔲 In-world code editor

### Phase 4: Graphics (Mid-term)

- 🔲 3D rendering in Bevy client
- 🔲 Asset loading system
- 🔲 Custom model support
- 🔲 Sync text and graphical state

### Phase 5: Polish (Long-term)

- 🔲 Permission system
- 🔲 Database optimization
- 🔲 Web-based admin panel
- 🔲 Documentation generator
- 🔲 Example worlds

## Learning Opportunities

This expanded scope offers even more learning:

### New Rust Concepts

- **Language implementation:** Parser, AST, interpreter/compiler
- **Sandboxing:** Safe execution of untrusted code
- **Database interaction:** SQLx, Diesel, or SeaORM
- **Async I/O:** Tokio for Telnet server
- **FFI:** Potential Lua integration

### New System Design

- **Multi-protocol architecture:** TCP + UDP on same server
- **Scripting language design:** Syntax, semantics, runtime
- **Content pipeline:** Hot-reload, versioning, distribution
- **Permission systems:** Fine-grained access control

### New DevOps

- **Database migrations:** Schema evolution
- **Backup strategies:** World state persistence
- **Monitoring:** Script execution metrics
- **Scaling:** Horizontal world sharding

## Comparison to Existing Projects

### vs. TinyMUSH/MOO

**What we add:**

- Modern graphics
- ECS performance
- Type-safe scripting
- Bevy ecosystem

**What we keep:**

- In-world programming
- Text protocol
- Soft-coded content

### vs. Roblox/Core/Dreams

**What we add:**

- Text interface option
- Open-source server
- Self-hosting capability
- Rust performance

**What we keep:**

- User-created content
- Scripting language
- Content agnostic

### vs. Veloren/Vintage Story

**What we add:**

- In-world programming
- Content agnosticism
- Dual interfaces

**What we keep:**

- Open-source
- Voxel/3D rendering
- Community-driven

## Success Criteria

The engine is successful when:

1. **A non-programmer** can create a simple interactive object via text commands
2. **A programmer** can write complex game logic without touching Rust
3. **A player** can seamlessly switch between text and graphical clients
4. **A world** can run independently with custom rules and art
5. **The server** handles 100+ concurrent players without lag

## Next Steps

1. **Update documentation** to reflect new vision
2. **Research scripting languages** (Rhai vs Lua vs custom)
3. **Design entity/component schema** for soft-coded objects
4. **Prototype text protocol** alongside UDP
5. **Plan database schema** for persistent worlds

This is an incredibly ambitious and educational project that will teach you not just Rust, but language design, network protocols, database systems, and game architecture! 🦀

---

**Key Insight:** You're not building a game—you're building a **platform for building games**.
