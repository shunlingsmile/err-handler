use thiserror::Error;
use err_handler::err_handler;

#[derive(Debug, Error)]
enum Err {
    #[error("err1")]
    Err1,
    #[error("err2")]
    Err2,
}

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

fn main() -> Result<(), Err> {
    assert_eq!(task(0)?, 100);
    Ok(())
}