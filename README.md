# `err-handler` is a non-intrusive error handling marco that enhances your error handling
If you need to handle errors externally or consolidate them in a single location, err-handler is your logical choice
```toml
[dependencies]
err-handler = "0.1.0"
```
# Examples
```rust
use thiserror::Error;
use err_handler::err_handler;
#[derive(Debug, Error)]
enum Err {
    #[error("err1")]
    Err1,
    #[error("err2")]
    Err2,
}
// The ` err-handler` marco attribute can be any function name but must have a matching return type.
#[err_handler(task_handler)]
fn task(_v: i32) -> Result<i32, Err> {
    Err(Err::Err1)
}
fn task_handler(e: Err) -> Result<i32, Err> {
    match e {
        Err::Err1 => Ok(100),
        _ => Err(e)
    }
}
// If a target function is asynchronous, then its error handling function must also be asynchronous.
#[err_handler(crate::async_task_handler)]
async fn async_task(_v: i32) -> Result<i32, Err> {
    Err(Err::Err1)
}
async fn async_task_handler(e: Err) -> Result<i32, Err> {
    match e {
        Err::Err1 => Ok(100),
        _ => Err(e)
    }
}
#[tokio::main]
async fn main() -> Result<(), Err> {
    assert_eq!(task(0)?, 100);
    assert_eq!(async_task(0).await?, 100);
    Ok(())
}
```