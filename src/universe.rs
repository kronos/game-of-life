use crate::storage::Storage;

pub struct Universe<T: Storage> {
    storage: T,
    width: usize,
    height: usize,
}

impl<T: Storage> Universe<T> {
    pub fn new(width: usize, height: usize) -> Universe<T> {
        let size = width * height;
        return Universe::<T> {
            storage: T::new(size),
            width,
            height,
        };
    }

    pub fn alive(&self, row: usize, col: usize) -> bool {
        self.storage.get(row * self.width + col)
    }

    pub fn refresh(&mut self) {
        let mut idx = 0;
        let w = self.width as i32;
        let h = self.height as i32;
        for i in 0..h {
            for j in 0..w {
                let current_alive = self.storage.get(idx);
                let mut alive_around = if current_alive { -1 } else { 0 };
                for p in -1..=1 {
                    let row = ((i + p + h) % h) as usize;
                    for q in -1..=1 {
                        let col = ((j + q + w) % w) as usize;
                        if self.alive(row, col) {
                            alive_around += 1;
                        }
                    }
                }

                self.storage.set(
                    idx,
                    (current_alive && alive_around == 2) || alive_around == 3,
                );
                idx += 1
            }
        }
        self.storage.switch();
    }

    pub fn init(&mut self, mut f: impl FnMut(usize, usize) -> bool) {
        let mut idx = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                if f(i, j) {
                    self.storage.set(idx, true);
                }
                idx += 1
            }
        }
        self.storage.switch();
    }

    pub fn width(&self) -> usize {
        return self.width;
    }

    pub fn height(&self) -> usize {
        return self.height;
    }
}
