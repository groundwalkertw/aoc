const N: usize = 5;

struct Checker {
    marked: bool,
    num: u16,
}
struct Board ([[Checker; N]; N]);

impl Board {
    fn mark(&mut self, num: u16) {
        for i in 0..N {
            for j in 0..N {
                if (self.0)[i][j].num == num {
                    (self.0)[i][j].marked = true;
                }
            }
        }
    }

    fn col_check(&self, i: usize) -> bool {
        for j in 0..N {
            if !self.0[j][i].marked {
                return false;
            }
        }
        true
    }
    fn row_check(&self, i: usize) -> bool {
        for j in 0..N {
            if !self.0[i][j].marked {
                return false;
            }
        }
        true
    }

    fn check(&self) -> bool {
        for i in 0..N {
            if self.col_check(i) || self.row_check(i) {
                return true;
            }
        }
        false
    }
    fn do_sum(&self) -> u16 {
        let iter = self.0.iter().flatten();
        let mut sum = 0;
        for checker in iter {
            if !checker.marked {
                sum += checker.num; 
            }
        }
        sum
    }
}
fn task1() {
    
}


