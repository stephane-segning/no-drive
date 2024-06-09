pub use anyhow;
pub use shaku;
pub use thiserror;

pub mod common;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
