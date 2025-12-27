# Project Scope Change Summary

## What Changed

Your project has evolved from a **space simulation game** to a **programmable world engine platform**.

### Before (Space Simulation)

- Specific genre: Space/EVE Online inspired
- Hardcoded game mechanics
- Single graphical interface
- Fixed world theme

### After (Programmable Engine)

- **Content agnostic:** Any genre (fantasy, sci-fi, modern, etc.)
- **User-programmable:** In-world scripting language
- **Dual interface:** Both graphical (3D) and text-based (MUD/MUSH)
- **Flexible worlds:** Users define their own rules, art, and systems

## Why This Is Better for Learning

The new scope offers **significantly more learning opportunities**:

### Original Scope Learning

- ✅ Rust basics
- ✅ ECS architecture
- ✅ Networking
- ✅ DevOps/CI/CD

### Expanded Scope Learning (Adds)

- ⭐ **Language implementation** - Embed Rhai or build custom DSL
- ⭐ **Sandboxing** - Safe execution of untrusted code
- ⭐ **Multi-protocol servers** - TCP + UDP simultaneously
- ⭐ **Database integration** - Persistent user content (SQLite/PostgreSQL)
- ⭐ **Hot-reloading** - Dynamic code updates
- ⭐ **Permission systems** - Fine-grained access control
- ⭐ **Content pipelines** - Asset management and distribution

## What You're Building Now

### The Elevator Pitch
>
> "A programmable multiplayer platform where users can create their own worlds with custom rules and art, accessible through both graphical 3D clients and classic text-based interfaces—all without forking the codebase."

### Think of it as

- **Roblox/Core** - but with text interface option and self-hosted
- **TinyMUSH/MOO** - but with modern graphics and Rust performance
- **Minecraft modding** - but standardized and content-agnostic

### Real Use Cases

**Example 1: Fantasy RPG**

```
Admin creates via text client:
@create "Magic Sword"
@set Magic Sword/script = {
    on_equip(player) {
        player.strength += 10;
        emit("You feel power surge through you!");
    }
}

Players see in graphical client:
[3D model of glowing sword]
[Character stats update visually]
```

**Example 2: Social Hub**

```
Admin creates via scripting:
@create "Dance Floor"
@set Dance Floor/script = {
    on_enter(player) {
        play_music("party_mix");
        player.add_animation("dance");
    }
}

Players experience:
[Music starts playing]
[Avatar begins dancing animation]
```

**Example 3: Puzzle Game**

```
Admin scripts logic:
@create "Pressure Plate"
@set Pressure Plate/script = {
    on_step(player) {
        if count_players(this.room) >= 4 {
            open_door("secret_passage");
        }
    }
}

Works in both:
- Text: "As you step on the plate, you hear grinding stone..."
- Graphics: [Door animation slides open]
```

## Technical Architecture Changes

### New Components Needed

```
assignment-W-4/
├── shared/
│   ├── components.rs    # ECS components (enhanced)
│   ├── protocol.rs      # Multi-protocol support
│   ├── scripting.rs     # NEW: Script interface
│   └── database.rs      # NEW: Persistence layer
├── server/
│   ├── main.rs
│   ├── script_vm.rs     # NEW: Rhai execution
│   ├── tcp_server.rs    # NEW: Text protocol
│   ├── udp_server.rs    # Existing: Graphics protocol
│   └── permissions.rs   # NEW: Access control
├── client/              # Graphical client (existing)
└── telnet/              # NEW: Text client
```

### Technology Additions

| Component | Technology | Why |
|-----------|------------|-----|
| **Scripting** | Rhai | Rust-native, sandboxed by default |
| **Database** | SQLite/PostgreSQL | Persist user content |
| **Text Protocol** | Tokio TCP | Classic MUD/MUSH compatibility |
| **Permissions** | Custom system | Player/admin/builder roles |

## Development Phases

### Phase 1: Foundation (Current ✅)

- [x] Rust workspace setup
- [x] Bevy ECS basics
- [x] Lightyear networking
- [x] Development tooling
- [x] Documentation structure

### Phase 2: Core Engine (Next)

- [ ] Basic entity/component system
- [ ] Simple text protocol (Telnet)
- [ ] Command parser
- [ ] Room/object database
- [ ] SQLite persistence

### Phase 3: Scripting (Mid-term)

- [ ] Rhai integration
- [ ] Script API design
- [ ] Sandboxing and timeouts
- [ ] In-world code editor
- [ ] Script storage in database

### Phase 4: Graphics Integration

- [ ] 3D rendering in Bevy
- [ ] Asset loading system
- [ ] Sync text/graphics state
- [ ] Custom model support

### Phase 5: Advanced Features

- [ ] Permission system
- [ ] Hot-reloading
- [ ] Web admin panel
- [ ] Example worlds
- [ ] Documentation generator

## Recommended Next Steps

### 1. Research Phase (1-2 weeks)

- [ ] Read Rhai documentation: https://rhai.rs/book/
- [ ] Study TinyMUSH source code for inspiration
- [ ] Design basic object model
- [ ] Sketch database schema

### 2. Prototype Phase (2-4 weeks)

- [ ] Add Rhai to `Cargo.toml`
- [ ] Create simple script execution example
- [ ] Build basic Telnet server
- [ ] Implement simple command parser
- [ ] Test script sandboxing

### 3. Core Development (2-3 months)

- [ ] Build persistence layer
- [ ] Implement object system
- [ ] Create admin commands
- [ ] Add permission system
- [ ] Write example scripts

### 4. Integration (1-2 months)

- [ ] Connect graphical client
- [ ] Sync both protocols
- [ ] Add asset loading
- [ ] Polish user experience

## Learning Resources

### MUD/MUSH Development

- TinyMUSH Source: https://github.com/TinyMUSH/TinyMUSH
- MOO Programmer's Manual: http://www.hayseed.net/MOO/
- MUD Dev Archive: http://www.kanga.nu/lists/mud-dev/

### Embedded Scripting

- Rhai Book: https://rhai.rs/book/
- Crafting Interpreters: https://craftinginterpreters.com/
- mlua (if switching to Lua): https://github.com/mlua-rs/mlua

### Database in Rust

- SQLx: https://github.com/launchbadge/sqlx
- SeaORM: https://www.sea-ql.org/SeaORM/
- Diesel: https://diesel.rs/

### Multi-Protocol Servers

- Tokio Tutorial: https://tokio.rs/tokio/tutorial
- Network Programming Patterns: https://marabos.nl/atomics/

## Questions to Consider

As you develop, think about:

1. **Scripting Language**
   - Start with Rhai or evaluate Lua?
   - Custom DSL as Phase 2?

2. **Database Choice**
   - SQLite for simplicity or PostgreSQL for features?
   - Embedded or separate server?

3. **Protocol Design**
   - Text protocol: Telnet, SSH, or WebSocket?
   - How to sync state between protocols?

4. **Permission Model**
   - Player/Builder/Admin roles?
   - Fine-grained or coarse-grained?

5. **Content Distribution**
   - How do users share worlds?
   - Asset hosting strategy?

## Why This Is Exciting

This project now combines:

- **Nostalgia:** Classic MUD/MUSH gameplay
- **Modern Tech:** Rust, Bevy, real-time graphics
- **User Creativity:** Programmable by players
- **Learning Depth:** Language design, databases, networking, graphics

You're not just building a game—you're building a **platform for building games**! 🦀

---

**Files to Read:**

- [docs/vision.md](vision.md) - Complete vision document
- [docs/scripting-research.md](scripting-research.md) - Scripting language comparison
- [README.md](../README.md) - Updated project overview

**Next Action:** Decide whether to prototype Rhai integration or design the object model first.
