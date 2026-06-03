# Drift Architecture

This document explains the internal architecture of Drift.

The project is intentionally structured around scalable graph traversal and viewport-based rendering for large database schemas.

---

# High-Level Overview

```txt
PostgreSQL
    ↓
Schema Introspection
    ↓
Tree Graph
    ↓
Diff Engine
    ↓
Flattened Projection
    ↓
Viewport Filtering
    ↓
TUI Renderer
```

---

# Design Goals

Drift was designed around several core principles:

* scalable schema traversal
* terminal-first UX
* graph-oriented schema modeling
* interactive rendering
* large schema navigation
* incremental architecture evolution

The project intentionally treats database schemas as navigable graph structures rather than flat migration artifacts.

---

# Core Components

## 1. Schema Introspection Layer

Location:

```txt
src/db/
```

Responsible for:

* connecting to PostgreSQL
* loading schema metadata
* constructing schema trees

Current introspection:

* columns
* indexes

Planned:

* foreign keys
* constraints
* triggers
* materialized views

---

# 2. Tree Graph Layer

Location:

```txt
src/tree/
```

Schemas are represented as recursive graph structures.

Example:

```txt
database
└── users
    ├── columns
    │   ├── id
    │   └── email
    │
    ├── indexes
    │   └── idx_users_email
```

This abstraction enables:

* recursive traversal
* subtree collapsing
* graph diffing
* viewport projection

---

# 3. Diff Engine

Location:

```txt
src/tree/diff.rs
```

The diff engine recursively compares two schema trees.

Diff states:

* Added
* Removed
* Modified
* Unchanged

The resulting graph becomes the renderable application state.

---

# 4. Flattening Layer

Recursive trees are transformed into linear viewport rows.

Terminal rendering requires:

```txt
Vec<RenderableRows>
```

instead of recursive graph traversal per frame.

This layer handles:

* indentation
* tree connectors
* metadata formatting
* collapse state
* projection generation

---

# 5. Viewport System

The viewport system solves large-schema rendering problems.

Instead of rendering:

```txt
all rows
```

the renderer only displays:

```txt
visible rows
```

Architecture:

```txt
Flattened Rows
    ↓
Scroll Offset
    ↓
Viewport Slice
    ↓
Renderer
```

This is critical for scalability.

---

# 6. Search Pipeline

Search operates as a projection layer.

Architecture:

```txt
Flattened Rows
    ↓
Fuzzy Matching
    ↓
Filtered Indices
    ↓
Viewport
```

Current implementation:

* fuzzy filtering
* interactive navigation
* search jump navigation

Planned:

* search highlighting
* auto-expanded search paths
* indexed search caching

---

# 7. Rendering Engine

Location:

```txt
src/tui/
```

Rendering uses:

* Ratatui
* Crossterm

Current renderer:

* dual synchronized panes
* scroll viewport
* metadata summaries
* interactive tree expansion

---

# Scalability Considerations

The project intentionally explores:

* recursive graph traversal
* large schema rendering
* viewport synchronization
* flattening cost
* interactive filtering
* scalable TUI architecture

Large synthetic schemas are used to stress test:

* 10k+ columns
* hundreds of tables
* multiple indexes
* chained foreign keys

---

# Current Technical Limitations

Current architecture still rebuilds flattened rows on refresh:

```txt
full recursive flatten
```

Future versions will evolve toward:

* incremental flattening
* subtree-only updates
* virtualized rendering
* async background loading

---

# Future Architecture Evolution

## Incremental Flattening

Current:

```txt
full tree rebuild
```

Planned:

```txt
subtree-only projection updates
```

---

## Virtualized Rendering

Current:

```txt
visible viewport slice
```

Planned:

```txt
lazy materialized rows
```

---

## Async Introspection

Planned:

* parallel schema loading
* streaming introspection
* live refresh

---

## Graph-Based Relationships

Future foreign key support will evolve schemas from:

```txt
tree structures
```

toward:

```txt
hybrid graph/tree models
```

This will enable:

* relationship traversal
* dependency visualization
* graph-aware diffing

---

# Why This Architecture

The architecture intentionally separates:

* introspection
* graph modeling
* diff traversal
* viewport projection
* rendering

This separation allows independent evolution of:

* rendering strategies
* diff algorithms
* database backends
* navigation systems

without tightly coupling UI and schema logic.

---

# Long-Term Vision

Drift aims to evolve into a scalable database infrastructure visualization platform focused on:

* schema drift analysis
* large-scale schema navigation
* infrastructure observability
* migration verification
* graph-based database introspection

The long-term goal is to explore how large infrastructure systems can be represented efficiently inside terminal-native interfaces.