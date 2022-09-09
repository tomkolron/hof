use std::env;

pub fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("2 arguments needed");
    }
    // assert_eq!(args.len(), 3);
    vec![args[1].clone(), args[2].clone()]
}
