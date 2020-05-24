pub struct Universe {
    data: [Vec<bool>; 2],
    width: usize,
    height: usize,
    n: usize,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        return Universe {
            data: [vec![false; width * height], vec![false; width * height]],
            width,
            height,
            n: 0,
        };
    }

    pub fn alive(&self, row: usize, col: usize) -> bool {
        self.data[self.n][row * self.width + col]
    }

    pub fn refresh(&mut self) {
        let mut idx = 0;
        let w = self.width as i32;
        let h = self.height as i32;
        for i in 0..h {
            for j in 0..w {
                let mut alive_around = if self.data[self.n][idx] { -1 } else { 0 };
                for p in -1..=1 {
                    let row = ((i + p + h) % h) as usize;
                    for q in -1..=1 {
                        let col = ((j + q + w) % w) as usize;
                        if self.alive(row, col) {
                            alive_around += 1;
                        }
                    }
                }
                self.data[1 - self.n][idx] =
                    (self.data[self.n][idx] && alive_around == 2) || alive_around == 3;
                idx += 1
            }
        }
        self.n = 1 - self.n;
    }

    pub fn init(&mut self, mut f: impl FnMut(usize, usize) -> bool) {
        let mut idx = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                self.data[self.n][idx] = f(i, j);
                idx += 1
            }
        }
    }

    pub fn width(&self) -> usize {
        return self.width;
    }
    pub fn height(&self) -> usize {
        return self.height;
    }
}
