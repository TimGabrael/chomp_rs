mod chomp;

fn main() {
    let mut chomp: chomp::Chomp<7, 4> = chomp::Chomp::new();
    let best_move = chomp.get_best_move();
    println!("{} {}", best_move.0, best_move.1);
    chomp.click(best_move.0, best_move.1);
    if chomp.is_finished() {
        println!("chomp game is in final game state");
    }
    chomp.print();
    chomp.reset();
}

