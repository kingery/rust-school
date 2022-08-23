#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn deliver_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {

}

pub fn eat_at_restaurant() {
    // absolute call
    crate::front_of_house::hosting::add_to_waitlist();

    // relative call
    front_of_house::hosting::add_to_waitlist();
}
