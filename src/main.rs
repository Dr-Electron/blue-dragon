use blue_dragon::banner::print_banner;
use blue_dragon::cli::Opt;

fn main() {
    print_banner();

    let opts = Opt::new();

    blue_dragon::run(opts)
}