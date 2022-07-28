// in lib.rs

pub mod front_of_house {
 pub   mod hosting{
       pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }
    mod serving{
        fn take_order(){}
        fn server_order(){}
        fn take_payment(){}
        fn complain(){}
    }
}

pub fn eat_at_restaurant() {
    // call add_to_waitlist with **absolute path**:
    front_of_house::hosting::add_to_waitlist();

    // call with **relative path** 
    front_of_house::hosting::add_to_waitlist();
}