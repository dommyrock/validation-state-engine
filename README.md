# Rule Validation engine / Simulation engine

Re-implementation of 'BusinessRules' validation engine (i previously architected and coded in C#) but this time with Rust with </br>/ tokio async runtime / parallel tokio worker threads / [channels](https://docs.rs/tokio/latest/tokio/sync/watch/fn.channel.html) / file watching / 0-24h simulation and error aggregations.

## File Watching

I have decided to use tokio::sync::watch here to watch for the file changes. </br>

Other options were :

Notify-rs

Couple of examples

- [Async monitor](https://github.com/notify-rs/notify/blob/main/examples/async_monitor.rs)
- [Debouncer](https://github.com/notify-rs/notify/blob/main/examples/debouncer_full.rs)

### Other considerations

- [RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html)

> One CONFIG Writer (task) and multiple Reader (tasks).
