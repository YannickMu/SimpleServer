use std::net::{TcpListener, TcpStream};
use std::env;
use std::io::{BufReader, prelude::*, Result, Lines};
use server::{SHUTDOWN, ARGS, NOT_FOUND_ERROR, BASE_PATH, API_PATH, HTML_PATH};
use std::process::Command;
use std::path::Path;
use std::process;

mod threadpool;
use threadpool::ThreadPool;

fn main() -> () {
	{
	let args: Vec<String> = env::args().collect();
	for i in args {
		ARGS.lock().unwrap().push(i);
	}
	if !Path::new(&format!("{BASE_PATH}")).is_dir() {
		eprintln!("\x1b[31mError: Directory {} doesn't exist\x1b[0m", BASE_PATH);
		process::exit(1);
	} else if ARGS.lock().unwrap().len() == 2 && &ARGS.lock().unwrap()[1] == "api" && !Path::new(&format!("{BASE_PATH}{API_PATH}")).is_dir() {
		eprintln!("\x1b[31mError: Directory {}{} doesn't exist\x1b[0m", BASE_PATH, API_PATH);
		process::exit(1);
	} else if ARGS.lock().unwrap().len() == 2 && &ARGS.lock().unwrap()[1] == "html" && !Path::new(&format!("{BASE_PATH}{HTML_PATH}")).is_dir() {
		eprintln!("\x1b[31mError: Directory {}{} doesn't exist\x1b[0m", BASE_PATH, HTML_PATH);
		process::exit(1);
	} else if ARGS.lock().unwrap().len() == 1 {
		eprintln!("\x1b[31mError: Argument required \x1b[0mapi\x1b[31m for api server or \x1b[0mhtml\x1b[31m for html server!\x1b[0m");
		process::exit(1);
	}
	}
	let listener: TcpListener = TcpListener::bind("0.0.0.0:8080").unwrap();
	let pool: ThreadPool = ThreadPool::new(4);

	for stream in listener.incoming() {
		let stream: TcpStream = stream.unwrap();

		pool.execute(|| {
			handle_connection(stream);
		});

		if SHUTDOWN.lock().unwrap().len() == 1 {
			break;
		}
	}
	drop(pool);
	process::exit(0);
}

fn handle_connection(mut stream: TcpStream) {
	let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
	let mut response: String = String::from("");
	let mut path: &str = "";
	let mut req_type: &str = "";

	let lines: Lines<BufReader<&TcpStream>> = buf_reader.lines();

	let lines: Vec<String> = get_lines(lines).unwrap();

	if lines.len() == 0 {
		let content = String::from("Oh no, internal server error...");
		let length = content.len();
		response = format!("HTTP/1.1 500 INTERNAL SERVER\nContent-Length: {length}\n\n{content}");
	} else {
		let request_line = &lines[0];
		path = request_line.split_whitespace().nth(1).unwrap_or("/");
		req_type = request_line.split_whitespace().nth(0).unwrap_or("GET");
	}
	if ARGS.lock().unwrap().len() == 2 && &ARGS.lock().unwrap()[1] == "api" {
		let mut output: String;
		let pathargs: String = format!("{path}?args=none");
		let realpath: &str = pathargs.split('?').nth(0).unwrap();
		if realpath == "/" {
			let executable: String = format!("{BASE_PATH}{API_PATH}/main");
			if Path::new(&executable).is_file() {
				output = String::from_utf8(Command::new(executable).arg(&pathargs.split('?').nth(1).unwrap()).arg(&req_type).output().unwrap().stdout).unwrap_or("std".to_string());
			} else {
				output = NOT_FOUND_ERROR.to_string();
			}
		} else {
			let executable: String = format!("{BASE_PATH}{API_PATH}{realpath}");
			if Path::new(&executable).is_file() {
				output = String::from_utf8(Command::new(executable).arg(&pathargs.split('?').nth(1).unwrap()).arg(&req_type).output().unwrap().stdout).unwrap_or("std".to_string());
			} else {
				output = NOT_FOUND_ERROR.to_string();
			}
		}
		let length: usize = output.len();
		response = format!("HTTP/1.1 200 ACCEPTED\nContent-Length: {length}\n\n{output}");
	} else if response.len() == 0 {
		let output: String = String::from_utf8(Command::new("/home/smuely/projects/RUST/hello_world/target/debug/hello_world").output().unwrap().stdout).unwrap_or("sdf".to_string());
		dbg!(output);
		response = format!("HTTP/1.1 500 INTERNAL SERVER\nContent-Length: 3\n\n...");
	}
	stream.write_all(response.as_bytes()).unwrap();
}

fn get_lines<B: std::io::BufRead>(linesor: Lines<B>) -> Result<Vec<String>> {
	let mut lines: Vec<String> = Vec::new();
	for line in linesor {
		let cur = line?;
		if cur.is_empty() {
			break;
		}
		lines.push(cur);
	}
	Ok(lines)
}
