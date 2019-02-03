use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

enum JobStatus {
    Running,
    Queued,
    Held,
    Undetermined
}

struct JobInfo {
    id: String,
    status: JobStatus
}

fn add_args(process: &mut Command, args: &Vec<&str>) -> () {
    for arg in args {
        process.arg(arg);
    }
}

fn main() {
    let mut squeue_template = Command::new("squeue");

    add_args(&mut squeue_template, &vec!("-a"));

    let mut squeue_process = squeue_template
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdout_stream = squeue_process.stdout.as_mut().unwrap();
    let stream_reader = BufReader::new(stdout_stream);

    for (i, line) in stream_reader.lines().enumerate() {
        println!("{}", line.unwrap());
    }

    match squeue_process.try_wait() {
        Ok(Some(status)) => println!("Exited with: {}", status),
        Ok(None) => {
            println!("Waiting");
            let result = squeue_process.wait();
            println!("Result {:?}", result);
        },
        Err(e) => println!("Error: {}", e)
    }
}