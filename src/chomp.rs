
#[derive(PartialEq, Copy, Clone, Hash)]
pub struct Chomp<const WIDTH: usize, const HEIGHT: usize> {
    a: [[bool; WIDTH]; HEIGHT],
}

impl<const WIDTH: usize, const HEIGHT: usize> Chomp<WIDTH, HEIGHT> {
    pub fn new() -> Chomp<WIDTH, HEIGHT> {
        return Chomp{
            a: [[false; WIDTH]; HEIGHT],
        };
    }
    pub fn print(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.a[y][x] {
                    print!("#");
                }
                else {
                    print!("O");
                }
            }
            println!("");
        }
    }
    pub fn click(&mut self, x: usize, y: usize) -> bool {
        if x >= WIDTH || y >= HEIGHT {
            return false;
        }
        if Chomp::<WIDTH, HEIGHT>::is_final_block(x, y) {
            // the final block can not be chosen as the click target
            return false;
        }
        if self.a[y][x] {
            return false;
        }
        for _y in 0..=y {
            for _x in x..WIDTH {
                self.a[_y][_x] = true;
            }
        }
        return true;
    }
    pub fn reset(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.a[y][x] = false;
            }
        }
    }
    pub fn is_finished(&mut self) -> bool {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if Chomp::<WIDTH, HEIGHT>::is_final_block(x, y) {
                    continue;
                }
                if !self.a[y][x] {
                    return false;
                }
            }
        }
        return true;
    }

    fn is_final_block(x: usize, y: usize) -> bool {
        if x == 0 && y == (HEIGHT - 1) {
            return true;
        }
        return false;
    }

    fn simulate_turn(mut self, x: usize, y: usize) -> bool {
        self.click(x, y);

        let best_move = self.get_best_move();
        if best_move.0 == std::usize::MAX {
            return true;
        }
        return false;
    }

    pub fn get_best_move(&mut self) -> (usize, usize) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if Chomp::<WIDTH, HEIGHT>::is_final_block(x, y) {
                    continue;
                }
                if !self.a[y][x] {
                    if self.clone().simulate_turn(x, y) {
                        return (x, y);
                    }
                }
            }
        }
        return (std::usize::MAX, std::usize::MAX);
    }

}


