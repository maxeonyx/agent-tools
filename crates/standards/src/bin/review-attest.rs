fn main() {
    let mut args = std::env::args();
    let program = args.next().unwrap_or_else(|| "review-attest".to_string());

    let Some(action) = args.next() else {
        eprintln!("{}", standards::review_attest::usage(&program));
        std::process::exit(2);
    };
    let Some(target) = args.next() else {
        eprintln!("{}", standards::review_attest::usage(&program));
        std::process::exit(2);
    };
    let Some(concern) = args.next() else {
        eprintln!("{}", standards::review_attest::usage(&program));
        std::process::exit(2);
    };

    let action = match action.as_str() {
        "prompt" => standards::review_attest::ReviewAction::Prompt,
        "record" => standards::review_attest::ReviewAction::Record,
        _ => {
            eprintln!("{}", standards::review_attest::usage(&program));
            std::process::exit(2);
        }
    };

    match standards::review_attest::perform(action, &target, &concern) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("review-attest failed: {error}");
            std::process::exit(1);
        }
    }
}
