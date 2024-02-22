mod chomp;

fn main() {
    let mut chomp: chomp::Chomp<11, 8> = chomp::Chomp::new();
    let best_move = chomp.get_best_move();
    println!("{} {}", best_move.0, best_move.1);
    chomp.click(best_move.0, best_move.1);
    chomp.print();
    chomp.reset();
}

