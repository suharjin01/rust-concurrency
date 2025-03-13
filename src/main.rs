use std::thread;


// Current Thread di fungsi main
fn main() {
    let current_thread = thread::current();
    println!("{} : Hello, world!", current_thread.name().unwrap());
}

// Membuat Thread
#[cfg(test)]
mod tests {

    use std::thread;
    use std::time::Duration;
    use std::thread::JoinHandle;

    // Thread
    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            for i in 0..=5 {
                println!("Counter : {}", i);
                thread::sleep(Duration::from_secs(1));
            }
        });

        println!("Application finish");
        thread::sleep(Duration::from_secs(7));
    }

    // Join Thread
    #[test]
    fn test_join_thread() {
        let handle = thread::spawn(|| {
            let mut counter = 0;
            for i in 0..=5 {
                println!("Counter : {}", i);
                thread::sleep(Duration::from_secs(1));
                counter += 1;
            }

            return counter;
        });

        println!("Waiting Handle");

        let result = handle.join();
        match result {
            Ok(counter) => println!("Total counter : {} ", counter),
            Err(error) => println!("Error : {:?}", error)
        }

        println!("Application finish");
    }

    // Sequential
    fn calculate() -> i32 {
        let mut counter = 0;
        // current thread di fungsi calculate untuk mengetahui thread pada fungsi calculate berjalan di unit test mana saja
        let current_thread = thread::current();
        for i in 0..=5 {
            match current_thread.name() {
                None => {println!("{:?} : Counter : {}", current_thread.id(), i);}
                Some(name) => {println!("{:?} : Counter : {}", name, i);}
            }

            //println!("Counter : {}", i);
            thread::sleep(Duration::from_secs(1));
            counter += 1;
        }

        return counter;
    }

    #[test]
    fn test_sequential() {
        let result1 = calculate();
        let result2 = calculate();

        println!("Total counter 1 : {}", result1);
        println!("Total counter 2 : {}", result2);
        println!("Application finish!");
    }

    // Parallel
    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(|| calculate());
        let handle2 = thread::spawn(|| calculate());

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(counter) => {println!("Total Counter 1 : {}", counter)}
            Err(error) => {println!("Error : {:?}", error)}
        }

        match result2 {
            Ok(counter) => {println!("Total Counter 2 : {}", counter)}
            Err(error) => {println!("Error : {:?}", error)}
        }

        println!("Application Finish!");
    }

    // Move Keyword
    #[test]
    fn test_closure() {
        // current thread di fungsi unit test "test_closure" 
        let current_thread = thread::current();
        // cetak nama thread
        println!("{} : Ini adalah nama Thread", current_thread.name().unwrap());

        let name = String::from("Aqil");
        let closure = move || {
            thread::sleep(Duration::from_secs(2));
            println!("Hello {}", name);
        };

        //closure();
        let handle = thread::spawn(closure);
        handle.join().unwrap();
    }


    // Thread Factory
    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My Thread".to_string());

        let handle = factory.spawn(calculate).expect("Failed to cread a new thread");
        let total = handle.join().unwrap();

        println!("Total Counter : {}", total);
    }


    // Channel
    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            sender.send("Hello from thread".to_string());
        });

        let handle2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("{}", message)
        }); 

        let _ = handle1.join();
        let _ = handle2.join();
    }

    // Channel mengirim banyak data/ lebih dari satu data
    #[test]
    fn test_channel_queue() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for i in 0..5 { 
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello from thread".to_string());
            }
            sender.send("Exit".to_string())
        });

        let handle2 = thread::spawn(move || {
            loop {
                let message = receiver.recv().unwrap();
                if message == "Exit" {
                    break;
                }
                println!("{}", message)
            }
        }); 

        let _ = handle1.join();
        let _ = handle2.join();
    }


    // Channel life Cycle
    #[test]
    fn test_channel_iterator() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for i in 0..5 { 
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello from thread".to_string());
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("{}", value);
            }
        }); 

        let _ = handle1.join();
        let _ = handle2.join();
    }


    // Channel life Cycle
    #[test]
    fn test_channel_multi_sender() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let sender2 = sender.clone();

        let handle3 = thread::spawn(move || {
            for i in 0..5 { 
                thread::sleep(Duration::from_secs(1));
                sender2.send("Hello from sender 2".to_string());
            }
        });

        let handle1 = thread::spawn(move || {
            for i in 0..5 { 
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello from sender 1".to_string());
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("{}", value);
            }
        }); 

        let _ = handle1.join();
        let _ = handle2.join();
        let _ = handle3.join();
    }

}