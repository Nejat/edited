#![doc(hidden)]

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 4 + 2;

        assert_ne!(result, 42);
    }
}
