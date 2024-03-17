
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod parser;
use actix_cors::Cors;

#[derive(Debug, serde::Serialize)]
struct HealthResponse {
    status: String,
}


#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse { status: "UP".to_string() })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}



#[post("/parse")]
async fn parse(req_body: String) -> impl Responder {

    let mut workout = parser::Workout::new();
    workout.parse(&req_body);
    workout.printjson();

    // Give json back 
    HttpResponse::Ok().body(workout.to_json())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    const URL : &str = "127.0.0.1";
    println!("Starting the server at {}", URL);

    HttpServer::new(move || {
        let cors = Cors::default()
        .allowed_origin("http://127.0.0.1:5000")
        .allowed_methods(vec!["GET", "POST", "OPTIONS"])
        .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
        .allowed_header(actix_web::http::header::CONTENT_TYPE)
        .max_age(3600);
    
    // TODO: Implement cors so that everything works
    
        App::new()
            .wrap(cors)
            .service(health)
            .service(echo)
            .service(parse)
    })
    .bind((URL, 8080))?
    .run()
    .await
}










/* 
fn main() {
    // let inputs = ["Squat 3x10@2", "Squat 3x10"];
    // for input in inputs.iter() {
    //     let (_, exercise) = parser::parse_exercise_line(input).unwrap();
    //     println!("{:?}", exercise);
    // }

    // let input = "# 07.03.2024 LegDay";
    // let (_, exercise_day) = parser::parse_exercise_day_line(input).unwrap();
    // println!("{:?}", exercise_day);

    // let input = "* This is a comment";
    // let (_, comment) = parser::parse_comment_line(input).unwrap();
    // println!("{:?}", comment);


    // Open the file
  
    let file = File::open("gymlog.txt").expect("Failed to open file");
    let reader = io::BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        if line.is_empty() {
            continue;
        }
        // Try parsing the line as an exercise day
        if let Ok((_, exercise_day)) = parser::parse_exercise_day_line(&line) {
            println!("Exercise Day: {:?}", exercise_day);
        }
        // Try parsing the line as an exercise
        else if let Ok((_, exercise)) = parser::parse_exercise_line(&line) {
            println!("Exercise: {:?}", exercise);
        }
        // Try parsing the line as a comment
        else if let Ok((_, comment)) = parser::parse_comment_line(&line) {
            println!("Comment: {:?}", comment);
        }
        // If none of the parsers match, print the line as is
        else {
            println!("Unrecognized line: {}", line);
        }
    }
}
*/

