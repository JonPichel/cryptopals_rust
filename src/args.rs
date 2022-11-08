use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Cryptopals Rust")]
#[command(author = "Jonathan Pichel Carrera <jonathanpc@protonmail.com>")]
#[command(version = "1.0")]
#[command(about = "Cryptopals solutions written in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub sets: Sets,
}

#[derive(Subcommand, Debug)]
pub enum Sets {
    SetOne {
        #[command(subcommand)]
        exercises: SetOneExercises
    }
}

#[derive(Subcommand, Debug)]
pub enum SetOneExercises {
    ExerciseFour {
        path: PathBuf,
    },
    ExerciseSix {
        path: PathBuf,
    }
}