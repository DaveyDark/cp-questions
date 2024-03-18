extern crate chrono;
extern crate scraper;
use chrono::{Datelike, Duration, Local, Month, NaiveDate};
use rusqlite::{Connection, Error};
use scraper::{Html, Selector};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

const PREFIX: &str = "# CP-Questions
This repo contains the solutions to various CP(competetive progrogramming) questions that I have solved on various websites like leetcode and codechef


# Log
";
const OUTFILE: &str = "README";
const INFILE: &str = "table.html";
const URL_PREFIX: &str = "https://github.com/DaveyDark/cp-questions/blob/master/";

fn read_table(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn to_iso_date(date: &str) -> String {
    date.split("/")
        .collect::<Vec<&str>>()
        .iter()
        .rev()
        .fold(String::new(), |acc, &x| acc + x + "-")
        .trim_matches('-')
        .to_string()
}

fn process_lines(input: &str) -> Vec<String> {
    input
        .trim()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|data| data.to_string())
        .collect()
}

fn import() -> Result<(), Error> {
    let table = match read_table(INFILE) {
        Ok(val) => val,
        Err(e) => {
            println!("Error reading table: {}", e);
            panic!()
        }
    };

    let fragment = Html::parse_fragment(&table);
    let mut data: Vec<Vec<String>> = Vec::new();

    for table in fragment.select(&Selector::parse("table").unwrap()) {
        for row in table.select(&Selector::parse("tr").unwrap()).skip(1) {
            let cols = row
                .select(&Selector::parse("td").unwrap())
                .collect::<Vec<_>>();
            let date = to_iso_date(&cols[0].text().collect::<String>());
            let lang = to_iso_date(&cols[3].text().collect::<String>());
            let links: Vec<String> = process_lines(&cols[1].text().collect::<String>());
            let problems: Vec<String> = process_lines(&cols[2].text().collect::<String>());
            for i in 0..links.len() {
                data.push(vec![
                    date.clone(),
                    links[i].clone(),
                    problems[i].clone(),
                    lang.clone(),
                ]);
            }
        }
    }

    println!("Finished reading data");

    insert_data(&data)
}

fn insert_data(data: &Vec<Vec<String>>) -> Result<(), Error> {
    let conn = Connection::open("log.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT,
            link TEXT,
            problem TEXT,
            language TEXT
        )",
        [],
    )?;

    let mut insert_stmt =
        conn.prepare("INSERT INTO log (date, link, problem, language) VALUES (?, ?, ?, ?)")?;
    for row in data {
        insert_stmt.execute([
            row[0].clone(),
            row[1].clone(),
            row[2].clone(),
            row[3].clone(),
        ])?;
    }

    println!("Finished writing to database");

    Ok(())
}

fn export_html() -> Result<(), Error> {
    let conn = Connection::open("log.db")?;

    let start_date_str =
        &conn.query_row("SELECT date FROM log ORDER BY date LIMIT 1", [], |row| {
            row.get::<_, String>(0)
        })?;
    let start_date = NaiveDate::parse_from_str(start_date_str, "%Y-%m-%d").unwrap();
    let end_date = Local::now();

    let mut current_date = start_date;
    let mut current_month: i32 = current_date
        .format("%m")
        .to_string()
        .parse::<i32>()
        .unwrap()
        - 1;

    let mut html = String::new();

    while current_date <= end_date.naive_local().date() {
        let month = current_date
            .format("%m")
            .to_string()
            .parse::<i32>()
            .unwrap();
        let current_date_str = current_date.format("%Y-%m-%d").to_string();

        if month != current_month {
            // switch month
            let month_name = format!(
                "{}, {}",
                Month::try_from(current_date.month() as u8)
                    .ok()
                    .unwrap()
                    .name(),
                current_date.year()
            );
            if !html.is_empty() {
                html += "</table></details>";
            }
            html += "<details>";
            html += format!("<summary><h2>{}</h2></summary>", month_name).as_str();
            html += "<table>";
            html += "<tr><th>Date</th><th>Question Id</th><th>Question Title</th><th>Language</th></tr>";
            current_month = month;
        }

        let mut stmt =
            conn.prepare("SELECT date, link, problem, language FROM log WHERE date=?")?;
        let rows = stmt
            .query_map([&current_date_str], |row| {
                Ok(vec![
                    row.get::<_, String>(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                ])
            })?
            .collect::<Result<Vec<Vec<String>>, Error>>()?;

        current_date += Duration::days(1);

        let mut links = String::new();
        let mut problems = String::new();
        let mut langs = String::new();

        if rows.is_empty() {
            links = "-".to_string();
            problems = "-".to_string();
            langs = "-".to_string();
        }

        for row in &rows {
            let mut link = URL_PREFIX.to_string();
            let problem_site = row[1].split("#").next().unwrap().to_ascii_lowercase();
            let problem_number = row[1].split("#").last().unwrap().to_ascii_lowercase();
            link += problem_site.as_str();
            link += "/";
            link += problem_number.as_str();
            link += ".";
            link += match row[3].as_str() {
                "Rust" => "rs",
                "C++" => "cpp",
                "TypeScript" => "ts",
                "Java" => "java",
                "SQL" => "sql",
                _ => "",
            };
            links += format!("<a href='{}'>{}</a><br>", link, row[1]).as_str();

            let mut problem_link = match problem_site.as_str() {
                "leetcode" => "https://leetcode.com/problems/".to_string(),
                "geeksforgeeks" => "https://www.geeksforgeeks.org/problems/".to_string(),
                _ => String::new(),
            };

            problem_link += row[2]
                .to_ascii_lowercase()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("-")
                .as_str();

            match problem_site.as_str() {
                "leetcode" => problem_link += "/",
                "geeksforgeeks" => problem_link += "/1/",
                _ => (),
            }

            problems += format!("<a href='{}'>{}</a><br>", problem_link, row[2]).as_str();
            langs += format!("{}<br>", row[3]).as_str();
        }
        html += "<tr>";
        html += format!("<td>{}</td>", current_date_str).as_str();
        html += format!("<td>{}</td>", links).as_str();
        html += format!("<td>{}</td>", problems).as_str();
        html += format!("<td>{}</td>", langs).as_str();
        html += "</tr>";
    }

    let mut file = File::create(OUTFILE.to_string() + ".html").unwrap();
    file.write_all(html.as_bytes()).unwrap();
    Ok(())
}

fn format() -> Result<(), std::io::Error> {
    let output = Command::new("prettier")
        .arg("--write")
        .arg(OUTFILE.to_string() + ".html")
        .output()?;

    if output.status.success() {
        println!("HTML formatted successfully.");
        Ok(())
    } else {
        println!("Failed to format HTML: {:?}", output);
        Err(std::io::Error::from_raw_os_error(
            output.status.code().unwrap_or(1),
        ))
    }
}

fn export_md() -> Result<(), std::io::Error> {
    let mut ifile = File::open(OUTFILE.to_string() + ".html")?;
    let mut contents = String::new();
    ifile.read_to_string(&mut contents)?;
    let mut ofile = File::create(OUTFILE.to_string() + ".md").unwrap();
    ofile
        .write_all((PREFIX.to_string() + contents.as_str()).as_bytes())
        .unwrap();
    Ok(())
}

fn update() -> Result<(), Error> {
    let (date, id, name, lang) = (
        get_input("Date"),
        get_input("Question ID"),
        get_input("Question Name"),
        get_input("Language"),
    );

    let conn = Connection::open("log.db")?;
    conn.execute(
        "INSERT INTO log (date, link, problem, language) VALUES (?,?,?,?)",
        [date, id, name, lang],
    )?;

    Ok(())
}

fn get_input(inp: &str) -> String {
    let mut input = String::new();
    println!("Enter {}: ", inp);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cpt <function>");
        return;
    }

    let func = &args[1];

    match func.as_str() {
        "export" => {
            export_html().expect("Error exporting database");
            format().expect("Error formatting html");
            export_md().expect("Error exporting to markdown");
        }
        "update" => {
            update().expect("Error adding entry to db");
        }
        "import" => {
            import().expect("Error importing html");
        }
        _ => println!("Inavlid function name"),
    }
}
