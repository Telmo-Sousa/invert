fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        print_usage_and_exit();
    }
    let infile = args.remove(0);
    let outfile = args.remove(args.len() - 1);
    let mut img = image::open(&infile).expect("Failed to open INFILE.");

    while !args.is_empty() {
        let operation = args.remove(0);
        match operation.as_str() {
            "invert" => {
                img.invert();
            }
            _ => {
                print_usage_and_exit();
            }
        }
    }

    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    std::process::exit(-1);
}

