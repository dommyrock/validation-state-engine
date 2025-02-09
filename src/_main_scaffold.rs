/*
Chat https://chatgpt.com/share/67a7c043-4120-8010-9e30-d47b636102ea

        1 Ignore the first message response
        2 start plannign from 2-3, than
        3 include the async runtime from last 2 respones (there configurationService was lost from ctx, so make sure to include it back)


        -----v1
        > use genrics instead of fyn dispatch

        struct BusinessRuleService<T: IConfigurationService> {
            config_service: T, // Concrete instance instead of a trait object
        }

        impl<T: IConfigurationService> BusinessRuleService<T> {
            fn new(config_service: T) -> Self {
                BusinessRuleService { config_service }
            }

            fn get_rule(&self) -> String {
                self.config_service.get_rule_definition()
            }
        }

    fn main() {
            let config_service = ConfigurationService;
            let business_rule_service = BusinessRuleService::new(config_service);
            let rule = business_rule_service.get_rule();
            println!("Business Rule: {}", rule);
    }


        -------v2
        > Add file watching logic

        We'll introduce:

            A watcher thread inside ConfigurationService to detect XML file changes.
            Channels to notify BusinessRuleService when the file updates.

        See >>>> "1. Adding File Watcher in ConfigurationService" for full service layout

    fn main() {
        let (tx, rx) = mpsc::channel(); // Channel for config updates

        // Shared configuration service instance
        let config_service = Arc::new(Mutex::new(ConfigurationService::new("config.xml".to_string(), tx)));

        // BusinessRuleService receives config updates
        let business_rule_service = BusinessRuleService::new(Arc::clone(&config_service), rx);

        // Simulating a usage scenario
        println!("Initial Rule: {}", business_rule_service.get());

        // Keep the program running so the file watcher keeps working
        loop {
            thread::sleep(Duration::from_secs(10));
        }
    }

    ------v3
    > Async Tokio version

    #[tokio::main]
    async fn main() {
        let config_service = ConfigurationService::new("config.xml".to_string()).await;
        let business_rule_service = BusinessRuleService::new(Arc::clone(&config_service)).await;

        << Compute current conrule configs **** For the first tiem here >>

        // Print initial rule
        println!("Initial Rule: {}", business_rule_service.get().await);

        // Print initial rule 2
        println!("SecondRule: {}", business_rule_service.get().await);

        // Continuously check for updates
        let mut rx = business_rule_service.rx.clone();
        while let Ok(_) = rx.changed().await {

            <<Compute Rules again on each config file update >>

            println!("Updated Rule 1: {}", rx.borrow());
            println!("Updated Rule 2: {}", rx.borrow());
            //...
        }
    }

    -----v4 (2nd to last response )
    > RWlock is actually good for this (ype of lock allows a number of readers or at most one writer at any point in time...)
    > https://doc.rust-lang.org/std/sync/struct.RwLock.html

    let initial_rules = vec!["Rule1".to_string(), "Rule2".to_string()];
    let service = BusinessRuleService {
        rules: Arc::new(RwLock::new(initial_rules)),
    };
    //....

    -----v5
    > Using tokio::task::JoinSet instead of RwLock
*/

mod library;
use library::BusinessRuleService::BusinessRuleService;
use library::RuleType;

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task;

#[tokio::main]
async fn main() {
    //Mark what rules we eval here: (there vould be 5x active , but we want to eval 3 of 5)
    let initial_rules = vec![
        RuleType::SideJobPrevention,
        RuleType::ExhaustionPrevention,
        RuleType::LastMinuteBookingPrevention,
    ];

    // intit the 'BusinessRuleService' which will be shared among Parallel tokio tasks
    let service = BusinessRuleService {
        //rules: Arc::new(RwLock::new(initial_rules)), TODO: Inner service
    };
    let service = Arc::new(service);

    // Spawn multiple tasks that use the service concurrently (JoinHandle<()> should be custom BusinessRule Error type instead )
    let mut tasks:Vec<tokio::task::JoinHandle<()>> = Vec::with_capacity(initial_rules.len());

    for (_i,rule) in initial_rules.into_iter().enumerate() { //move rules here since we don't expect to have to use then bellow this point
    
        let service_clone = Arc::clone(&service);
        let handle = task::spawn(async move {
            // Each task adds a new rule
            //service_clone.eval_rule_code() ...

            // Each task retrieves the current rules
            //let rules = service_clone.get().await;
        });
        tasks.push(handle);
    }
    
    // Wait for all tasks to complete 
    // If you do not care about the ordering of the outputs (JoinSet > https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html  )
    for handle in tasks {
        handle.await.unwrap();
    }
}

/* Rough pseudo code

    --------- async tokio tasks
    deps : 1 Tokio
            tokio::spawn https://docs.rs/tokio/latest/tokio/task/fn.spawn.html


        >>>>Example<<<<

        # To run multiple tasks in parallel and receive their results, join handles can be stored in a vector.

        async fn my_background_op(id: i32) -> String {
            let s = format!("Starting background task {}.", id);
            println!("{}", s);
            s
        }

        let ops = vec![1, 2, 3];
        let mut tasks = Vec::with_capacity(ops.len());
        for op in ops {
            // This call will make them start running in the background
            // immediately.
            tasks.push(tokio::spawn(my_background_op(op)));
        }

        let mut outputs = Vec::with_capacity(tasks.len());
        for task in tasks {
            outputs.push(task.await.unwrap());
        }
        println!("{:?}", outputs);

        >>>>Example<<<<

        This example pushes the tasks to outputs in the order they were started in. If you do not care about the ordering of the outputs, then you can also use a JoinSet.


           2 Notify (cross platform filesystem notification lib in rust)
                https://github.com/notify-rs/notify
                https://github.com/notify-rs/notify/blob/main/examples/async_monitor.rs (async watcher example)


*/

/*
    Idea here is to reuse rules from BusinessRules and make a small 'DEMO engine' out of them using:

        1) Async Rust 
            Axum / Tokio DI example (dyn dispatch vs Generics)
                1 https://github.com/tokio-rs/axum/blob/main/examples/dependency-injection/src/main.rs
                2 https://github.com/tokio-rs/axum/blob/main/examples/prometheus-metrics/src/main.rs (Prometheus metrics example)
                3 https://github.com/tokio-rs/axum/blob/main/examples/sse/src/main.rs (Server-Sent Events example)
s       

        2) Rust Errors --> custom Erorr type (Representing BR Error class)
            https://github.com/tokio-rs/axum/blob/main/examples/validator/src/main.rs (Validation example - maybe you can kinda see Error handling here) 
        
        2.1) XML READER
            https://crates.io/crates/quick-xml

        3) Type State pattern (where we parse the incoming + fetched data into known TYPE that we can reason abotut in few descriptive ways/branches)
        We basicaly want state machine (state of our collected BRErors) in our typesystem

        4) Multythreaded (for fun) -> Making this sequentiall seems like a waste of time and since we can split queries it's doable.
        Mutex on the heap (Section on writing correct Mutex to work with shared state and multiple threads)
        https://github.com/dommyrock/rs_design_patterns/blob/main/FirehoseOfRust.md#a-mutex-on-the-heap

        5) Typestate pattern
        https://cliffle.com/blog/rust-typestate/
        https://zerotomastery.io/blog/rust-typestate-patterns/

        6) Experiment with that cloudflare 
            1 Durable objects 
            2 voice -- agent model / that uses claude mcp i think (or just plug in voice srevice + custom tool caling ageny)
            https://www.youtube.com/watch?v=TcOytsfva0o
                - MCP server https://developers.cloudflare.com/agents/capabilities/mcp-server/ (i could set up SSE on this too)
                - Websockets https://developers.cloudflare.com/workers/runtime-apis/websockets

*/