

#[cfg(test)]
mod tests {

    use gymparser::parser;


    #[test]
    fn test_parse_exercise_day_line_eu_format() {
        let input = "# 12-03-2023 ArmDay";
        let (_, exercise_day) = parser::parse_exercise_day_line(input).unwrap();

        assert_eq!(exercise_day.date, "12.03.2023");
        assert_eq!(exercise_day.name, "ArmDay");
    }

    #[test]
    fn test_parse_exercise_day_line_dot_separator() {
        let input = "# 12.03.2023 BackDay";
        let (_, exercise_day) = parser::parse_exercise_day_line(input).unwrap();

        assert_eq!(exercise_day.date, "12.03.2023");
        assert_eq!(exercise_day.name, "BackDay");
    }

    #[test]
    fn test_parse_exercise_day_line_multi_digit_date() {
        let input = "# 31-12-2023 NewYearsEve";
        let (_, exercise_day) = parser::parse_exercise_day_line(input).unwrap();

        assert_eq!(exercise_day.date, "31.12.2023");
        assert_eq!(exercise_day.name, "NewYearsEve");
    }
    /*
    #[test]
    fn test_parse_exercise_day_line_invalid_input() {
        let input = "# 123.45.6789 InvalidDate";
        let result = parser::parse_exercise_day_line(input);

        match result {
            Ok(_) => panic!("Expected an error, but got Ok(_)"),
            Err(e) => assert_eq!(e.to_string(), "Invalid date format"),
        }
    }

     */

    #[test]
    fn test_parse_exercise_line_without_rir() {
        let input = "Bench Press 5x5";
        let (_, exercise) = parser::parse_exercise_line(input).unwrap();
        assert_eq!(
            exercise,
            parser::Exercise {
                name: "Bench Press".to_string(),
                sets: 5,
                reps: 5,
                weight: 0,
                rir: None
            }
        );
    }

    #[test]
    fn test_parse_exercise_line_with_rir() {
        let input = "Squat 3x10@2";
        let (_, exercise) = parser::parse_exercise_line(input).unwrap();
        assert_eq!(
            exercise,
            parser::Exercise {
                name: "Squat".to_string(),
                sets: 3,
                reps: 10,
                rir: Some(2),
                weight: 0
            }
        );
    }

    #[test]
    fn test_parse_exercise_line_multiple_words() {
        let input = "Dumbbell Shoulder Press 4x8";
        let (_, exercise) = parser::parse_exercise_line(input).unwrap();
        assert_eq!(
            exercise,
            parser::Exercise {
                name: "Dumbbell Shoulder Press".to_string(),
                sets: 4,
                reps: 8,
                weight: 0,
                rir: None
            }
        );
    }
}

    // #[test]
    // fn test_parse_comment_line_single_word() {
    //     let input = "* Hello";
    //     let (_, comment) = parse_comment_line(input).unwrap();
    //     assert_eq!(comment, Content { text: "Hello".to_string() });
    // }

    // #[test]
    // fn test_parse_comment_line_multiple_words() {
    //     let input = "* This is a comment";
    //     let (_, comment) = parse_comment_line(input).unwrap();
    //     assert_eq!(comment, Content { text: "This is a comment".to_string() });
    // }

    // #[test]
    // fn test_parse_comment_line_sentence() {
    //     let input = "* This is a longer comment with multiple words.";
    //     let (_, comment) = parse_comment_line(input).unwrap();
    //     assert_eq!(
    //         comment,
    //         Content {
    //             text: "This is a longer comment with multiple words.".to_string()
    //         }
    //     )
    // }
