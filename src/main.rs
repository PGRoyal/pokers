use cards::Card;

mod cards;
mod combos;

fn main() {
    println!("{}", Card::new_card_vec([(14, 2), (12, 3)]));
    println!("{}", Card::new(14, 2));
    println!("{}", Card::new(8, 3));
    println!("{}", Card::new(1, 2));
    println!("{}", Card::new(14, 4));
    let one = Card::new(3, 2);
    let two = Card::new(8, 3);
    let twotwo = Card::new(8, 2);
    let three = Card::new(14, 1);
    let threethree = Card::new(14, 2);
    println!("{}", one > two);
    println!("{}", two > one);
    println!("{}", one.get_number());
    println!("{}", one.get_suit());
    let mut card_vec = vec![two, one, three, twotwo];
    println!("{}", card_vec);
    card_vec.sort();
    println!("{}", card_vec);
    println!("{}", combos::has_pair(&card_vec));
    println!("{}", combos::has_two_pair(&card_vec));
    card_vec = vec![one, two, three];
    println!("{}", combos::has_pair(&card_vec));
    card_vec = vec![one, twotwo, two, three, threethree];
    println!("{}", combos::has_pair(&card_vec));
    println!("{}", combos::has_two_pair(&card_vec));

}
