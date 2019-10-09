#![recursion_limit = "256"]

mod common;
pub mod runner;

use common::{Msg, Request};
use yew::agent::{Bridge, Bridged};
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

const LIMIT: u32 = 1000;

pub struct Model {
    console: ConsoleService,
    runner: Box<dyn Bridge<runner::Runner>>,
    code: String,
    stdout: String,
    stderr: String,
    status: String,
    inst_limit: u32,
    stack_limit: u32,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|resp| Msg::RunnerResp(resp));
        Model {
            console: ConsoleService::new(),
            runner: runner::Runner::bridge(callback),
            code: String::new(),
            stdout: String::new(),
            stderr: String::new(),
            status: String::new(),
            inst_limit: LIMIT,
            stack_limit: LIMIT,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let should_run = match msg {
            Msg::InputCode(input) => {
                self.code = input.value;
                true
            }
            Msg::InputInst(input) => {
                self.inst_limit = input.value.parse().unwrap_or(LIMIT);
                true
            }
            Msg::InputStack(input) => {
                self.stack_limit = input.value.parse().unwrap_or(LIMIT);
                true
            }
            Msg::RunnerResp(resp) => {
                self.stdout = resp.stdout;
                self.stderr = resp.stderr;
                self.status = resp.status;
                false
            }
        };

        if should_run {
            self.status = format!("Running...");
            self.runner.send(Request {
                code: self.code.clone(),
                inst_limit: self.inst_limit,
                stack_limit: self.stack_limit,
            });
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1> { "Online Tac VM" }</h1>
                <h3> { "VM Input" } </h3>
                <form>
                    <label for="code"> { "Code" }</label>
                    <textarea style="height: 50vh" name="code" oninput=|content| Msg::InputCode(content)></textarea>
                    <label for="inst"> { "Instruction Limit" }</label>
                    <input name="inst" placeholder=LIMIT.to_string() oninput=|content| Msg::InputInst(content)></input>
                    <label for="stack"> { "Stack Limit" }</label>
                    <input name="stack" placeholder=LIMIT.to_string() oninput=|content| Msg::InputStack(content)></input>
                </form>
                <h3> { "VM Output" } </h3>
                <pre>{ &self.status } </pre>
                <h3> { "Standard Output" } </h3>
                <pre>{ &self.stdout } </pre>
                <h3> { "Standard Error" } </h3>
                <pre>{ &self.stderr } </pre>
            </div>
        }
    }
}
