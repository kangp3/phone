- Fix To tag handling, it can be empty twice on responses
- Fix Double From header in ACK
- Migrate error handling to `anyhow`
- Create HTTP server for calling
- Consolidate internet connection logic
- Improve error handling in loops (phone state machine, spawned threads)

cargo run --bin tlser
