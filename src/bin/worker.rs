use online_tac_vm::runner;
use yew::agent::Threaded;

fn main() {
    yew::initialize();
    runner::Runner::register();
    yew::run_loop();
}
