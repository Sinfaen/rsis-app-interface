# RSIS App Interface
All RSIS apps/models should use this library as the means to integrate into the RSIS framework.

## Interface
This crate exposes a single trait meant for both the RSIS scheduler implementation, and as for what any RSIS app should expect to be able to interact with.

```rust
pub trait Framework : Send {
    fn get_simtick(&self) -> i64;
    fn get_simtime(&self) -> f64;
    fn request_rx(&mut self, id : i64) -> Option<Box<dyn ChannelRx>>;
    fn request_tx(&mut self, id : i64) -> Box<dyn ChannelTx>;
}
```

### Time
`get_simtick` will return a 0-based integer referring to the actual tick of time of the scheduler.

`get_simtime` will return a floating value representing the simulation time. This does not have to correspond to the value of `get_simtick` multiplied by the time delta.

### MSPC Integration
`request_rx` and `request_tx` are meant to be called in the initialization step for an app. They return structures that allow for apps to interact with the dynamic messaging system built on top of rust's `mspc` toolkit.
