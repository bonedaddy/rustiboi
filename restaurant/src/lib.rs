#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // this takes us to the crate root
        // to access the serve_order call
        super::serve_order();
        // alternatively we can do
        crate::serve_order();
    }
    fn cook_order() {}
}


fn serve_order() {}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // relative
    front_of_house::hosting::add_to_waitlist();
}

