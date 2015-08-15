extern crate getopts;

extern crate otp_cop;

use std::{env};

use otp_cop::service::{ServiceFactory};


fn main() {
    let service_factories = vec![
        Box::new(otp_cop::SlackServiceFactory) as Box<ServiceFactory>,
        Box::new(otp_cop::GithubServiceFactory) as Box<ServiceFactory>,
    ];

    let mut opts = getopts::Options::new();

    for factory in service_factories.iter() {
        factory.add_options(&mut opts);
    }

    let matches = match opts.parse(env::args().skip(1)) {
        Ok(matches) => matches,
        Err(e) => panic!(e.to_string()),
    };

    let services = service_factories.iter().map(|factory| factory.create_service(&matches));
    for service in services {
        let result = service.get_users();
        println!("{}", result.service_name);
        println!("{}", "=".chars().cycle().take(result.service_name.len()).collect::<String>());
        println!("");
        for user in result.users {
            println!("{} ({}){}", user.name, user.email, match user.details {
                Some(details) => format!(" -- {}", details),
                None => "".to_string(),
            });
        }
        println!("");
        println!("");
    }
}