# Drift

Scalable terminal-first PostgreSQL schema diff engine built in Rust.

Drift is an interactive database schema visualization and diff engine designed for large-scale PostgreSQL environments.

It provides:

* side-by-side schema comparison
* recursive tree visualization
* fuzzy navigation
* interactive viewport rendering
* scalable schema introspection
* terminal-first developer workflows

Drift explores how large database schemas can be modeled, diffed, and rendered efficiently inside terminal interfaces.

---

# Features

* Interactive TUI
* Side-by-side schema comparison
* Expand/collapse schema trees
* Fuzzy search navigation
* PostgreSQL schema introspection
* Index detection
* Metadata summaries
* Scroll viewport rendering
* Large-scale schema stress testing
* Designed for enterprise-scale schemas

---

# Screenshots

## Schema Diff

```txt
SOURCE                                TARGET

users                                 users
├── columns                           ├── columns
│   ├── id                            │   ├── id
│   ├── username                      │   ├── username
│   └── email                         │   ├── email
                                      │   └── phone
```

## Fuzzy Navigation

```txt
/ table_200
```

---

# Why Drift Exists

Most schema tooling focuses primarily on migration generation.

Drift focuses on:

* schema visibility
* interactive exploration
* scalable rendering
* large-scale navigation
* terminal-native workflows

The project intentionally models database schemas as recursive graph structures rather than flat migration artifacts.

This allows:

* hierarchical visualization
* recursive diff traversal
* subtree collapsing
* scalable viewport rendering
* fuzzy graph navigation

---

# Architecture

```txt
PostgreSQL
    ↓
Schema Introspection
    ↓
Tree Graph
    ↓
Diff Engine
    ↓
Flattened Viewport
    ↓
Interactive TUI
```

See:

* ARCHITECTURE.md

---

# Tech Stack

* Rust
* Tokio
* SQLx
* Ratatui
* Crossterm
* PostgreSQL

---

# Quick Start

## Clone

```bash
git clone https://github.com/YOUR_USERNAME/drift.git

cd drift
```

## Run PostgreSQL Test Databases

```bash
docker compose up -d
```

## Start Drift

```bash
cargo run -- \
  --source postgres://postgres:password@localhost:5433/db1 \
  --target postgres://postgres:password@localhost:5434/db2
```

---

# Keyboard Shortcuts

| Key   | Action            |
| ----- | ----------------- |
| ↑ / ↓ | Navigate          |
| Enter | Expand / Collapse |
| /     | Fuzzy Search      |
| Esc   | Exit Search       |
| q     | Quit              |

---

# Large Schema Testing

Drift includes synthetic enterprise-scale schema generation for stress testing.

Current stress testing includes:

* 200+ tables
* 10k+ columns
* chained foreign keys
* indexes
* schema drift simulation

This is used to evaluate:

* recursive graph traversal
* flattening cost
* viewport synchronization
* interactive filtering
* rendering scalability

---

# Current Capabilities

* Columns
* Indexes
* Recursive diffing
* Interactive viewport
* Search navigation

---

# Roadmap

## Schema Engine

* [ ] Foreign key introspection
* [ ] Primary key introspection
* [ ] Constraints
* [ ] Trigger introspection
* [ ] Materialized views

## Rendering Engine

* [ ] Incremental flattening
* [ ] Virtualized rendering
* [ ] Sticky headers
* [ ] Async rendering
* [ ] Lazy subtree expansion

## UX

* [ ] Search highlighting
* [ ] Auto-expand search paths
* [ ] Vim keybindings
* [ ] Mouse support
* [ ] Minimap

## Databases

* [ ] MySQL support
* [ ] SQLite support
* [ ] CockroachDB support

---

# Performance Goals

Drift is intentionally designed around:

* large schema traversal
* scalable rendering
* low-memory viewport projection
* terminal-first infrastructure workflows

The project explores systems-level UI challenges such as:

* recursive graph traversal
* viewport synchronization
* flattening strategies
* incremental rendering
* large graph navigation

---

# Contributing

PRs are welcome.

Areas especially interesting:

* rendering performance
* graph algorithms
* PostgreSQL introspection
* virtualization
* terminal UX
* async architecture

---

# License

MIT
  Copyright 2026 dheerajthota0531

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

     http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
