#![allow(dead_code)]

use crate::args::{Cli, SetOneExercises, Sets};
use clap::Parser;
use crate::set_one::exercise_four::exercise_four;
use crate::set_one::exercise_six::exercise_six;

mod set_one;
mod args;

fn main() {
    let cli = Cli::parse();

    match cli.sets {
        Sets::SetOne { exercises } => {
            match exercises {
                SetOneExercises::ExerciseFour { path } => exercise_four(path),
                SetOneExercises::ExerciseSix { path } => exercise_six(path),
            }
        }
    }
}
