extern crate uia;
#[macro_use]
extern crate tokio_core;
#[macro_use]
extern crate log;

pub mod host;
pub mod stream;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
