// Ignore unused imports
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};

use nom::{
    IResult,
    sequence::{preceded, tuple, separated_pair},
    character::complete::{char, digit1, space1, alpha1, alphanumeric1, multispace1, multispace0},
    combinator::opt,
    bytes::complete::tag,
    branch::alt,
};

#[derive(Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Exercise {
    pub name: String,
    pub sets: u32,
    pub reps: u32,
    pub rir: Option<u32>,
    pub weight: u32,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq, Eq)]
pub struct ExerciseDay {
    pub date: String,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Comment {
    content: String,
}

// Workout 
#[derive(Debug, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Workout {
    exercise_days: Vec<ExerciseDay>,
    exercises: Vec<Exercise>,
    comments: Vec<Comment>,
}




pub fn parse_exercise_day_line(input: &str) -> IResult<&str, ExerciseDay> {
    let (input, (_,_,day,sep1,month,sep2,year,_,name)) = tuple((
        char('#'),
        space1,
        digit1,
        alt((char('-'), char('.'), char('\\'))), // Separates the date and the name
        digit1,
        alt((char('-'), char('.'), char('\\'))), // Separates the date and the name
        digit1,
        space1, // Separates the date and the name
        alphanumeric1, // Parses the name which can include alphanumeric characters
    ))(input)?;

    let _date_us = format!("{}-{}-{}", month, day, year); // US format: MM-DD-YY
    let date_eu = format!("{}.{}.{}", day, month, year); // European format: DD-MM-YY

    /*
        match parse_result {
        Ok((input, (_,_,day,sep1,month,sep2,year,_,name))) => {
            let date_us = format!("{}-{}-{}", month, day, year); // US format: MM-DD-YY
            let date_eu = format!("{}.{}.{}", day, month, year); // European format: DD-MM-YY

            // Add your date validation logic here. If the date is not valid, return an error.
            if !is_valid_date(&date_eu) {
                return Err("Invalid date format");
            }

            Ok((input, ExerciseDay {
                date: date_eu.to_string(),
                name: name.to_string(),
            }))
        },
        Err(_) => Err("Failed to parse the line"),
    }
}

     */


    Ok((input, ExerciseDay {
        date: date_eu.to_string(),
        name: name.to_string(),
    }))


}

pub fn parse_exercise_line(input: &str) -> IResult<&str, Exercise> {
    let (input, (name,_, sets, reps,weight, rir)) = tuple((
        nom::multi::many1(tuple((
            opt(multispace1),
            alpha1,
        ))),
        opt(preceded(multispace0, char(':'))), // Optional colon after exercise name, mb remove later
        preceded( // Parse the sets 
            multispace0,
            digit1,
        ),
        preceded( // Parse the reps
            tag("x"),
            digit1,
        ),
        preceded(tag("@"), digit1), // Weight
        opt(preceded(tag("@"), digit1)), // Optional RIR parsing
    ))(input)?;

    let exercise_name: String = name
        .iter()
        .map(|(_, word)| word.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    Ok((input, Exercise {
        name: exercise_name,
        sets: sets.parse().unwrap(),
        reps: reps.parse().unwrap(),
        weight: weight.parse().unwrap(),
        rir: rir.map(|s| s.parse().unwrap()),
    }))
}

// pub fn parse_exercise_line(input: &str) -> IResult<&str, Exercise> {
//     let (input, (name, _, sets, _, reps, rir)) = tuple((
//         alpha1,
//         multispace1,
//         digit1,
//         tag("x"),
//         digit1,
//         opt(preceded(tag("@"), digit1)), // Optional RIR parsing
//     ))(input)?;

//     Ok((input, Exercise {
//         name: name.to_string(),
//         sets: sets.parse().unwrap(),
//         reps: reps.parse().unwrap(),
//         rir: rir.map(|s| s.parse().unwrap()),

//     }))
// }

pub fn parse_comment_line(input: &str) -> IResult<&str, Comment> {
    let (input, (_, comment)) = tuple((
        tag("*"),
        tuple((
            multispace0,
            nom::multi::many1(tuple((
                opt(multispace1),
                alphanumeric1,
            ))),
        )),
    ))(input)?;

    let comment_text: String = comment
        .1
        .iter()
        .map(|(_, word)| word.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    Ok((input, Comment { content: comment_text }))
}


// Parse a multiline string into a vector of ExerciseDay, Exercise, and Comment structs
pub fn parse_workout(input: &str) -> Vec<Result<ExerciseDay, Result<Exercise, Comment>>> {
    let mut results = Vec::new();
    for line in input.lines() {
        if let Ok((_, exercise_day)) = parse_exercise_day_line(line) {
            results.push(Ok(exercise_day));
        }
        else if let Ok((_, exercise)) = parse_exercise_line(line) {
            results.push(Err(Ok(exercise)));
        }
        else if let Ok((_, comment)) = parse_comment_line(line) {
            results.push(Err(Err(comment)));
        }
    }
    results
}



impl Workout {
    pub fn new() -> Workout {
        Workout {
            exercise_days: Vec::new(),
            exercises: Vec::new(),
            comments: Vec::new(),
        }
    }

    // to json
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // implement print
    pub fn print(&self) {
        println!("Exercise Days: {:?}", self.exercise_days);
        println!("Exercises: {:?}", self.exercises);
        println!("Comments: {:?}", self.comments);
    }

    pub fn printjson(&self) {
        println!("{}", serde_json::to_string(self).unwrap());
    }

    pub fn parse(&mut self, input: &str) {
        for line in input.lines() {
            if let Ok((_, exercise_day)) = parse_exercise_day_line(line) {
                self.exercise_days.push(exercise_day);
            }
            else if let Ok((_, exercise)) = parse_exercise_line(line) {
                self.exercises.push(exercise);
            }
            else if let Ok((_, comment)) = parse_comment_line(line) {
                self.comments.push(comment);
            }
        }
    }
}
