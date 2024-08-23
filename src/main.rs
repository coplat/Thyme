#[derive(Debug)]
struct FunctionLog {
    name: String,
    time: u8,
    event: Event,
}
#[derive(Debug)]
enum Event {
    Start,
    End,
}

fn main() {
    let logs = vec![
        FunctionLog {
            name: String::from("Whole"),
            time: 0,
            event: Event::Start,
        },
        FunctionLog {
            name: String::from("TaskA"),
            time: 0,
            event: Event::Start,
        },
        FunctionLog {
            name: String::from("TaskA"),
            time: 5,
            event: Event::End,
        },
        FunctionLog {
            name: String::from("TaskB"),
            time: 15,
            event: Event::Start,
        },
        FunctionLog {
            name: String::from("TaskB"),
            time: 30,
            event: Event::End,
        },
        FunctionLog {
            name: String::from("TaskC"),
            time: 40,
            event: Event::Start,
        },
        FunctionLog {
            name: String::from("TaskC"),
            time: 67,
            event: Event::End,
        },
        FunctionLog {
            name: String::from("Whole"),
            time: 80,
            event: Event::End,
        },
    ];

    let mut start_times = std::collections::HashMap::new();

    let mut execution_times = std::collections::HashMap::new();

    for log in logs {
        match log.event {
            Event::Start => {
                start_times.insert(log.name.clone(), log.time);
            }
            Event::End => {
                if let Some(start_time) = start_times.remove(&log.name) {
                    let execution_time = log.time as i32 - start_time as i32;
                    execution_times.insert(log.name.clone(), execution_time);
                } else {
                    println!("No start time found for end event of function '{}'", log.name);
                }
            }
        }
    }

    // Print execution times
    for (name, execution_time) in execution_times {
        println!("Total execution time for '{}': {} seconds", name, execution_time);
    }

}