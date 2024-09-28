use anyhow::{anyhow, Result};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Insufficient Bytes")]
    InsufficientBytes,
}

pub fn decode() -> Result<(), MyError> {
    return Err(MyError::InsufficientBytes);
}

pub fn arrange(a: i32) -> Result<(), anyhow::Error> {
    if a % 2 == 0 {
        decode()?;
        Ok(())
    } else {
        Err(anyhow!("Error"))
    }
}

fn peace(a: i32) {
    match arrange(a) {
        Ok(()) => {
            println!("OK")
        }
        Err(e) => {
            if let Some(myerror) = e.downcast_ref::<MyError>() {
                match myerror {
                    MyError::InsufficientBytes => println!("insufficient !!"),
                }
            } else {
                println!("not My Error");
            }
        }
    }
}

fn main() {
    peace(2);
    peace(3);
}
