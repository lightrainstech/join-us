use join_us::{digits, from_pig_latin, rotate, to_pig_latin, unique};

fn main() {
    println!("digits(12345)      -> {:?}", digits(12345));
    println!("unique([1,2,2,3,1])-> {:?}", unique(&[1, 2, 2, 3, 1]));
    println!(
        "to_pig_latin(...)  -> {:?}",
        to_pig_latin("The quick brown fox")
    );
    println!(
        "from_pig_latin(...) -> {:?}",
        from_pig_latin("Hetay uickqay rownbay oxfay")
    );
    let mut list = vec![1, 2, 3, 4, 5, 6];
    rotate(&mut list, 2);
    println!("rotate([1..6], 2)  -> {:?}", list);
}
