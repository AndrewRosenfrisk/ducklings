use core::fmt;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    terminal::{
        size, Clear,
        ClearType::{All, Purge},
        DisableLineWrap,
    },
};
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};
fn main() -> Result<(), std::io::Error> {
    execute!(
        stdout(),
        Clear(Purge),
        Clear(All),
        MoveTo(0, 0),
        DisableLineWrap,
        Hide
    )?;

    let (columns, _rows) = size().unwrap();

    let mut lanes: Vec<Option<Duckling>> = vec![];

    for _ in 1..columns / 5 {
        lanes.push(None);
    }

    let mut rng = thread_rng();
    let mut counter = 0;

    'main: loop {
        for lane in 0..lanes.len() {
            let option_duckling = lanes[lane];
            if option_duckling.is_none() && rng.gen_bool(0.1) {
                lanes[lane] = Some(Duckling::new());
            }
            if option_duckling.is_some() {
                let mut duckling = option_duckling.unwrap();
                print!("{}", duckling);

                duckling.next_part();
                if duckling.next_part.is_none() {
                    lanes[lane] = None;
                } else {
                    lanes[lane] = Some(duckling)
                }
            } else {
                print!("{}", " ".repeat(5))
            }
        }
        print!("\n");
        stdout().flush()?;

        thread::sleep(Duration::from_secs_f32(0.2));
        counter += 1;
        if counter >= 1000 {
            println!("Arbitrary enpoint");
            break 'main;
        }
    }
    Ok(())
}

#[derive(PartialEq, Clone, Copy)]
enum Mouth {
    OPEN,
    CLOSED,
}
impl Distribution<Mouth> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Mouth {
        if rng.gen_bool(0.5) {
            Mouth::OPEN
        } else {
            Mouth::CLOSED
        }
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Eyes {
    BEADY,
    WIDE,
    HAPPY,
    ALOOF,
}
impl Distribution<Eyes> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Eyes {
        match rng.gen_range(0..=3) {
            0 => Eyes::BEADY,
            1 => Eyes::WIDE,
            2 => Eyes::HAPPY,
            _ => Eyes::ALOOF,
        }
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT,
}
impl Distribution<Direction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        if rng.gen_bool(0.5) {
            Direction::LEFT
        } else {
            Direction::RIGHT
        }
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Body {
    CHUBBY,
    VeryChubby,
}
impl Distribution<Body> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Body {
        if rng.gen_bool(0.5) {
            Body::CHUBBY
        } else {
            Body::VeryChubby
        }
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Wing {
    OUT,
    UP,
    DOWN,
}
impl Distribution<Wing> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Wing {
        match rng.gen_range(0..=2) {
            0 => Wing::OUT,
            1 => Wing::UP,
            _ => Wing::DOWN,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Part {
    HEAD,
    BODY,
    FEET,
}

#[derive(Clone, Copy)]
struct Duckling {
    direction: Direction,
    body: Body,
    mouth: Mouth,
    wing: Wing,
    eyes: Eyes,
    next_part: Option<Part>,
}
impl Duckling {
    //General methods
    fn new() -> Duckling {
        let body: Body = rand::random();
        let eyes = match body {
            Body::CHUBBY => Eyes::BEADY,
            _ => rand::random(),
        };
        Duckling {
            direction: rand::random(),
            body: body,
            mouth: rand::random(),
            wing: rand::random(),
            eyes: eyes,
            next_part: Some(Part::HEAD),
        }
    }
    fn get_next_part(&self) -> Option<String> {
        match self.next_part {
            Some(Part::HEAD) => Some(self.get_head_string()),
            Some(Part::BODY) => Some(self.get_body_string()),
            Some(Part::FEET) => Some(self.get_feet_string()),
            None => None,
        }
    }
    fn next_part(&mut self) {
        let next = match self.next_part {
            Some(Part::HEAD) => Some(Part::BODY),
            Some(Part::BODY) => Some(Part::FEET),
            Some(Part::FEET) | None => None,
        };
        self.next_part = next;
    }
    fn adjust_string_for_body(&self) -> String {
        match self.body {
            Body::CHUBBY => " ".to_string(),
            Body::VeryChubby => "".to_string(),
        }
    }
    //Head methods
    fn get_mouth(&self) -> String {
        match self.mouth {
            Mouth::CLOSED => "=".to_string(),
            Mouth::OPEN => {
                if self.direction == Direction::LEFT {
                    ">".to_string()
                } else {
                    "<".to_string()
                }
            }
        }
    }
    fn get_eyes(&self) -> String {
        match self.eyes {
            Eyes::WIDE => "''".to_string(),
            Eyes::HAPPY => "^^".to_string(),
            Eyes::ALOOF => "``".to_string(),
            Eyes::BEADY => {
                if self.body == Body::VeryChubby && self.direction == Direction::LEFT {
                    "\" ".to_string()
                } else if self.body == Body::VeryChubby && self.direction == Direction::RIGHT {
                    " \"".to_string()
                } else {
                    "\"".to_string()
                }
            }
        }
    }
    fn get_head_string(&self) -> String {
        match self.direction {
            Direction::LEFT => {
                format!(
                    "{}{}) {}",
                    &self.get_mouth(),
                    &self.get_eyes(),
                    &self.adjust_string_for_body()
                )
            }
            Direction::RIGHT => {
                format!(
                    " ({}{}{}",
                    &self.get_eyes(),
                    &self.get_mouth(),
                    &self.adjust_string_for_body()
                )
            }
        }
    }
    //Body methods
    fn get_body(&self) -> String {
        match self.body {
            Body::CHUBBY => " ".to_string(),
            Body::VeryChubby => "  ".to_string(),
        }
    }
    fn get_wing(&self) -> String {
        match self.wing {
            Wing::UP => "^".to_string(),
            Wing::DOWN => "v".to_string(),
            Wing::OUT => {
                if self.direction == Direction::LEFT {
                    ">".to_string()
                } else {
                    "<".to_string()
                }
            }
        }
    }
    fn get_body_string(&self) -> String {
        match self.direction {
            Direction::LEFT => {
                format!(
                    "({}{}){}",
                    self.get_body(),
                    self.get_wing(),
                    &self.adjust_string_for_body()
                )
            }
            Direction::RIGHT => {
                format!(
                    "({}{}){}",
                    self.get_wing(),
                    self.get_body(),
                    &self.adjust_string_for_body()
                )
            }
        }
    }
    //Feet methods
    fn get_feet_string(&self) -> String {
        match self.body {
            Body::CHUBBY => " ^^  ".to_string(),
            Body::VeryChubby => " ^ ^ ".to_string(),
        }
    }
}
impl fmt::Display for Duckling {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        if self.get_next_part().is_some() {
            string = self.get_next_part().unwrap();
        }
        write!(f, "{}", string)
    }
}
