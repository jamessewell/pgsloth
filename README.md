# pgsloth
The Postgres extension that believes in taking its sweet, sweet time. 🦥

Tired of fast queries? Let’s slow things down. 🤙

No hustle. No pressure. Just relaxed, unbothered, barely-scalable performance.

Key Features:
💤 Query Nap Mode - Takes a break mid-query. It deserves it ✅
🏝️ Relaxed Replication - Data gets there…eventually. (to be implemented)
🌀 Index-Free Indexing - Because the journey is the destination. (to be implemented)

Your query might run in 3 seconds… or 3 hours. Honestly? We’re not checking.

## Compilation

Install Rust and PGRX:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install pgrx-cli
cargo pgrx init
```

Add `pgsloth` to `shared_preload_libraries` in `~./pgrx/data-17/postgresql.conf`

Compile and install pgsloth in a local instance:

```
cargo pgrx run pg17

```

## Usage

```
pgsloth=# select 1;
NOTICE:  Time for a nap! pgsloth is sleeping for 5179 ms
 ?column? 
----------
        1
(1 row)

pgsloth=# 
pgsloth=# select 1;
NOTICE:  Time for a nap! pgsloth is sleeping for 7168 ms
 ?column? 
----------
        1
(1 row)
```
