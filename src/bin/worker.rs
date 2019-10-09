use yew::agent::Threaded;
use online_tac_vm::runner;

fn main() {
    yew::initialize();
    runner::Runner::register();
    yew::run_loop();
}