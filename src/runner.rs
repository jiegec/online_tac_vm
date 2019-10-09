use crate::common::{Msg, Request, Response};
use std::cell::RefCell;
use std::io::{self, Cursor, Write};
use std::rc::Rc;
use tacvm;
use yew::worker::*;

pub struct Runner {
    link: AgentLink<Runner>,
}

impl Agent for Runner {
    type Reach = Context;
    type Message = Msg;
    type Input = Request;
    type Output = Response;
    fn create(link: AgentLink<Self>) -> Self {
        Runner { link }
    }
    fn update(&mut self, _msg: Self::Message) {}
    fn handle(&mut self, msg: Self::Input, id: HandlerId) {
        let vm_input = Box::new(Cursor::new(Vec::new()));
        let vm_output_buffer = OutputBuffer::new();
        let vm_output = Box::new(vm_output_buffer.clone());
        let info_output_buffer = OutputBuffer::new();
        let info_output = Box::new(info_output_buffer.clone());
        let result = tacvm::work(
            &msg.code,
            msg.inst_limit,
            msg.stack_limit,
            true,
            true,
            vm_input,
            vm_output,
            info_output,
        );
        let (status, stdout, stderr) = match result {
            Ok(()) => {
                let status = format!("Running code succeeded");
                let stdout = match String::from_utf8(vm_output_buffer.data()) {
                    Ok(output) => output,
                    Err(err) => format!("{:?}", err),
                };
                let stderr = match String::from_utf8(info_output_buffer.data()) {
                    Ok(output) => output,
                    Err(err) => format!("{:?}", err),
                };
                (status, stdout, stderr)
            }
            Err(err) => (
                format!("Running code failed with {:?}", err),
                String::new(),
                String::new(),
            ),
        };
        self.link.response(
            id,
            Response {
                stdout,
                stderr,
                status,
            },
        );
    }
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
