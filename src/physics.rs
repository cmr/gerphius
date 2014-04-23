//inherited mutability: player is mutable if let is mutable

//todo: Test in ticks
//      Implement player acceleration based on ticks between inputs.
//      Potentially ticks holding a button and ticks not
//      Rotation accel/vel
use std;
use game;

#[deriving(Eq)]
pub enum Direction {
    Forward,
    Backward,
    Still
}

fn get_input() -> char{
    let mut input = std::io::stdin();
    let key = input.read_line().unwrap();
    return key.char_at(0);
}

pub fn accel(p:&mut game::Player){ //mut player:&mut player would allow to play w/ pointer
    if p.velocity >= -0.15 && p.velocity <= 0.15{
        p.velocity += p.accel;
    }
    if p.velocity < -0.05{
        p.velocity = -0.05;
    }
    if p.velocity >= 0.05{
        p.velocity = 0.05
    }
    p.position += p.velocity;
    let (acc, amod) = accel_compute(p.dir, p.accel, p.accel_mod);
    p.accel = acc;
    p.accel_mod = amod;
}


pub fn accel_compute (dir: Direction, mut accel:f32, mut accel_mod:int) -> (f32, int) {//this will use accel/accel_mod to compute the rate of increase of acceleration.
    if dir == Forward {
        let bounds = [
            (-85, -75, 25),
            (-75, -60, 22),
            (-60, -41, 19),
            (-40, -15, 17),
            (0, 15, 12),
            (14, 40, 10),
            (40, 60, 8),
            (60, 75, 5),
            (75, 85, 2)
        ];

        if accel_mod == 0 {
            accel_mod = 15;
        } else if accel_mod >= -15 && accel_mod < 0 {
            accel_mod = 0;
        } else {
            for &(lower, upper, increment) in bounds.iter() {
                if accel_mod >= lower && accel_mod < upper {
                    accel_mod += increment;
                    break
                }
            }
        }

    }
    else if dir == Backward {
        let bounds = [
            (-86, -75, -2),
            (-75, -60, -5),
            (-60, -41, -8),
            (-15, 0, -12),
            (15, 40, 17),
            (40, 60, 19),
            (60, 75, -22),
            (75, 85, -25)
        ];

        if accel_mod == 0 {
            accel_mod = -15;
        } else if accel_mod <= 15 && accel_mod > 0 {
            accel_mod = 0;
        } else {
            for &(lower, upper, increment) in bounds.iter() {
                if accel_mod >= lower && accel_mod < upper {
                    accel_mod += increment;
                    break
                }
            }
        }
    }

    if accel <= 0.05 && accel >= -0.05{
        accel = accel + (0.0000003 * (accel_mod as f32));
    }

    (accel, accel_mod) //returns accel and accel mod
}
