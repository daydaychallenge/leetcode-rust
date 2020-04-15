pub mod dd_challenge;

fn main() {
    println!("Hello, world!");
    dd_challenge::check_if_word_is_valid_after_substitution_01003::do_sth();

    /*let to_valid = "abcabc";
    let mut sp= to_valid.find("a");

    match sp {
        Some(s) => {
            let s1 = "abcabc".replacen("ab", "", 1);
            println!("{:?}", s1);
            println!("{:?} Find str: {:?} idx is {:?}", to_valid, "a", s);
        },

        None =>
            println!("Not found!")
    }*/
}
