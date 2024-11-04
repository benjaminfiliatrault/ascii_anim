use std::{thread, time};

use rand::random;

fn main() -> ! {
    let height = 30;
    let width = 123;
    let delay = time::Duration::from_millis(300);

    assert!(width % 2 == 1, "Width must be odd thanks");

    let mut screen = Screen::create(width, height);

    loop {
        screen.update();
        screen.display();

        clear_terminal_screen();

        thread::sleep(delay);
    }
}

#[derive(Debug, Clone, Copy)]
struct Particle {
    char: char,
    weigth: i32,
    life_time: i32,
}

#[derive(Clone)]
struct Screen {
    particles: Vec<Particle>,
    row: i32,
    width: i32,
    height: i32,
}

impl Screen {
    pub fn create(width: i32, height: i32) -> Screen {
        let mut screen = Screen {
            particles: Vec::with_capacity((width * height).try_into().unwrap()),
            width,
            height,
            row: 0,
        };

        screen.fill();
        return screen;
    }

    fn fill(&mut self) {
        for _ in 0..self.height {
            for _ in 0..self.width {
                self.particles.push(Particle {
                    char: ' ',
                    weigth: 0,
                    life_time: self.height + 1,
                })
            }
        }
    }

    pub fn update(&mut self) {
        self.update_char();
    }

    fn update_char(&mut self) {
        if self.row == self.height {
            self.row = 0;
        };

        for w in (0..self.width).rev() {
            let cursor = self.row * self.width;
            let index = (cursor + w) as usize;
            let random: u8 = random();
            let random_bool = random % 11 == 0;

            let mut current_particle = self.particles[index];
            let mut current_particle = self.particles[index];

            current_particle.char = if random_bool { '*' } else { ' ' };

            self.particles[index] = current_particle;
        }

        self.row += 1;
    }

    pub fn display(&self) {
        let mut offset = 0;
        let mut cursor = self.width;
        let mut row_string = String::new();

        for _ in 0..self.height {
            for x in offset..cursor {
                let c = self.particles[x as usize];
                row_string.push(c.char)
            }

            row_string.push('\n');

            cursor += self.width;
            offset += self.width;
        }

        print!("{}", row_string);
    }
}

/// Clear (wipe) the terminal screen
pub fn clear_terminal_screen() {
    print!("{esc}c", esc = 27 as char);
}
