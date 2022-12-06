#![allow(dead_code)]

use crate::args::{*};
use clap::Parser;
use crate::set_one::exercise_eight::exercise_eight;
use crate::set_one::exercise_four::exercise_four;
use crate::set_one::exercise_seven::exercise_seven;
use crate::set_one::exercise_six::exercise_six;
use crate::set_two::exercise_ten::exercise_ten;

mod set_one;
mod set_two;
mod args;

fn main() {
    let cli = Cli::parse();

    match cli.sets {
        Sets::SetOne { exercises } => {
            match exercises {
                SetOneExercises::ExerciseFour { path } => exercise_four(path),
                SetOneExercises::ExerciseSix { path } => exercise_six(path),
                SetOneExercises::ExerciseSeven {path} => exercise_seven(path),
                SetOneExercises::ExerciseEight {path} => exercise_eight(path),
            }
        }
        Sets::SetTwo { exercises } => {
            match exercises {
                SetTwoExercises::ExerciseTen { path } => exercise_ten(path),
            }
        }
    }
}
