#![recursion_limit = "256"]

mod runner;

use std::cell::RefCell;
use std::io::{self, Cursor, Write};
use std::rc::Rc;
use tacvm;
use yew::html::InputData;
use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    console: ConsoleService,
    code: String,
    stdout: String,
    stderr: String,
    status: String,
    inst_limit: u32,
    stack_limit: u32,
}

pub enum Msg {
    InputCode(InputData),
    InputInst(InputData),
    InputStack(InputData),
}

#[derive(Debug, Clone)]
struct OutputBuffer {
    buffer: Rc<RefCell<Vec<u8>>>,
}

impl OutputBuffer {
    fn new() -> Self {
        OutputBuffer {
            buffer: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn data(&self) -> Vec<u8> {
        self.buffer.borrow().clone()
    }
}

impl Write for OutputBuffer {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        let mut buffer = self.buffer.borrow_mut();
        buffer.extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            code: String::new(),
            stdout: String::new(),
            stderr: String::new(),
            status: String::new(),
            inst_limit: 1000,
            stack_limit: 1000,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputCode(input) => {
                self.code = input.value;
            }
            Msg::InputInst(input) => {
                self.inst_limit = input.value.parse().unwrap_or(1000);
            }
            Msg::InputStack(input) => {
                self.stack_limit = input.value.parse().unwrap_or(1000);
            }
        }
        let vm_input = Box::new(Cursor::new(Vec::new()));
        let vm_output_buffer = OutputBuffer::new();
        let vm_output = Box::new(vm_output_buffer.clone());
        let info_output_buffer = OutputBuffer::new();
        let info_output = Box::new(info_output_buffer.clone());
        let result = tacvm::work(
            &self.code,
            self.inst_limit,
            self.stack_limit,
            true,
            true,
            vm_input,
            vm_output,
            info_output,
        );
        match result {
            Ok(()) => {
                self.status = format!("Running code succeeded");
                match String::from_utf8(vm_output_buffer.data()) {
                    Ok(output) => {
                        self.stdout = output;
                    }
                    Err(err) => {
                        self.stdout = format!("{:?}", err);
                    }
                };
                match String::from_utf8(info_output_buffer.data()) {
                    Ok(output) => {
                        self.stderr = output;
                    }
                    Err(err) => {
                        self.stderr = format!("{:?}", err);
                    }
                };
            }
            Err(err) => {
                self.status = format!("Running code failed with {:?}", err);
            }
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
                    <input name="inst" oninput=|content| Msg::InputInst(content)></input>
                    <label for="stack"> { "Stack Limit" }</label>
                    <input name="stack" oninput=|content| Msg::InputStack(content)></input>
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
