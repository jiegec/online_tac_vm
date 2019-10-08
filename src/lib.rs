#![recursion_limit = "256"]

mod runner;

use std::cell::RefCell;
use std::io::{self, Cursor, Write};
use std::rc::Rc;
use stdweb::web::Date;
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
}

pub enum Msg {
    Input(InputData),
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(input) => {
                self.code = input.value;
            }
        }
        let vm_input = Box::new(Cursor::new(Vec::new()));
        let vm_output_buffer = OutputBuffer::new();
        let vm_output = Box::new(vm_output_buffer.clone());
        let info_output_buffer = OutputBuffer::new();
        let info_output = Box::new(info_output_buffer.clone());
        let result = tacvm::work(
            &self.code,
            1000,
            1000,
            true,
            true,
            vm_input,
            vm_output,
            info_output,
        );
        match result {
            Ok(()) => {
                self.status = format!("Running code success");
                self.console.log("success");
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
                <textarea oninput=|content| Msg::Input(content)></textarea>
                <p>{ &self.stdout } </p>
                <p>{ &self.stderr } </p>
                <p>{ &self.status } </p>
            </div>
        }
    }
}
