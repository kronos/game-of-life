use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    input::{Event::KeyboardInput, Input, Key},
    run, Result, Settings, Window,
};

use rand::Rng;
use stopwatch::Stopwatch;

static OFFSET: i32 = 2;
static SIDE: i32 = 5;
static SPACE: i32 = 2;
static WIDTH: i32 = 50;
static HEIGHT: i32 = 50;
static PAUSE_IN_MS: i64 = 50;

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
            n: 0,
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
                let mut cnt = if self.data[self.n][idx] { -1 } else { 0 };
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
            size: Vector::new(
                OFFSET * 2 + WIDTH * (SIDE + SPACE),
                OFFSET * 2 + HEIGHT * (SIDE + SPACE),
            ),
            ..Settings::default()
        },
        app,
    );
}

fn draw(universe: &Universe, gfx: &mut Graphics) {
    let mut y = OFFSET;
    gfx.clear(Color::BLACK);
    for i in 0..universe.height {
        for j in 0..universe.width {
            if universe.alive(i, j) {
                let rect = Rectangle::new(
                    Vector::new(OFFSET + (SIDE + SPACE) * j as i32, y),
                    Vector::new(SIDE, SIDE),
                );
                gfx.fill_rect(&rect, Color::GREEN);
            }
        }
        y += SIDE + SPACE;
    }
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let mut sw = Stopwatch::new();
    let mut started = false;
    let mut universe = Universe::new(WIDTH as usize, HEIGHT as usize);
    let mut rng = rand::thread_rng();

    universe.init(move |i, j| -> bool { rng.gen_range(0, i + j + 1) < (i + j) / 2 });

    draw(&universe, &mut gfx);
    gfx.present(&window)?;
    let mut cnt = 1;

    loop {
        if sw.elapsed_ms() > PAUSE_IN_MS * cnt {
            cnt += 1;
            universe.refresh();
            draw(&universe, &mut gfx);
            gfx.present(&window)?;
        }

        while let Some(event) = input.next_event().await {
            match event {
                KeyboardInput(key_event) => {
                    if key_event.is_down() && key_event.key() == Key::Space {
                        started = !started;
                        if started {
                            sw.start();
                        } else {
                            sw.stop();
                        }
                    }
                }
                _ => (), // println!("{:?}", event)
            }
        }
    }
}
