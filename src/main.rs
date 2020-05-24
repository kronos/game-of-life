use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    input::Input,
    run, Result, Settings, Window,
};

use rand::Rng;
use stopwatch::Stopwatch;

pub struct Universe {
    data: [Vec<bool>; 2],
    width: usize,
    height: usize,
    n: usize,
}

impl Universe {
    fn new(width: usize, height: usize) -> Universe {
        return Universe {
            data: [vec![false; width * height], vec![false; width * height]],
            width,
            height,
            n: 0
        };
    }

    fn alive(&self, row: usize, col: usize) -> bool {
        self.data[self.n][row * self.width + col]
    }

    fn refresh(&mut self) {
        let mut idx = 0;
        let w = self.width as i32;
        let h = self.height as i32;
        for i in 0..h {
            for j in 0..w {
                let mut cnt =  if self.data[self.n][idx] { -1 } else { 0 };
                for p in -1..=1 {
                    let row = ((i + p + h) % h) as usize;
                    for q in -1..=1 {
                        let col = ((j + q + w) % w) as usize;
                        if self.alive(row, col) {
                            cnt += 1;
                        }
                    }
                }
                self.data[1 - self.n][idx] = (self.data[self.n][idx] && cnt == 2) || cnt == 3;
                idx += 1
            }
        }
        self.n = 1 - self.n;
    }

    fn init(&mut self, mut f: impl FnMut(usize, usize) -> bool) {
        let mut idx = 0;
        for i in 0..self.height {
            for j in 0..self.width {
                self.data[self.n][idx] = f(i, j);
                idx += 1
            }
        }
    }
}

fn main() {
    run(
        Settings {
            title: "Game of life",
            ..Settings::default()
        },
        app,
    );
}

fn draw(universe: &Universe, gfx: &mut Graphics) {
    let offset = 2;
    let side = 5;
    let space = 2;
    let mut y = offset;
    gfx.clear(Color::BLACK);
    for i in 0..universe.height {
        y += side + space;
        for j in 0..universe.width {
            if universe.alive(i, j) {
                let rect = Rectangle::new(
                    Vector::new(offset + (side + space) * j as i32, y),
                    Vector::new(side, side),
                    
                );
                gfx.fill_rect(&rect, Color::GREEN);
            }
        }
    }
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let sw = Stopwatch::start_new();
    let mut universe = Universe::new(145, 108);
    let mut rng = rand::thread_rng();

    universe.init( move |i, j| -> bool {
        rng.gen_range(0, i + j + 1) < (i + j)/2
    });

    draw(&universe, &mut gfx);
    gfx.present(&window)?;
    let mut cnt = 1;

    loop {
        if sw.elapsed_ms() > 50*cnt {
            cnt += 1;
            universe.refresh();
            draw(&universe, &mut gfx);
            gfx.present(&window)?;
        }

        while let Some(_) = input.next_event().await {
        }
    }
}
