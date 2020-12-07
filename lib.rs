#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use ink_lang as ink;

#[ink::contract]
mod mytest {
    use alloc::string::String;

    #[ink(storage)]
    pub struct Mytest {
        value: String,
    }

    impl Mytest {
        #[ink(constructor)]
        pub fn new(init_value: String) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(String::from("init!"))
        }

        #[ink(message)]
        pub fn flip(&mut self, new_value: String) {
            self.value = new_value;
        }

        #[ink(message)]
        pub fn get(&self) -> String {
            self.value.clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn default_works() {
            let mytest = Mytest::default();
            assert_eq!(mytest.get(), "init!");
        }

        #[test]
        fn it_works() {
            let mut mytest = Mytest::new("A new message".to_string());
            assert_eq!(mytest.get(), "A new message");
            mytest.flip("Another message".to_string());
            assert_eq!(mytest.get(), "Another message");
        }
    }
}
