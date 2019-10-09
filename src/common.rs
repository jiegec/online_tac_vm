use serde::*;
use yew::html::InputData;
use yew::worker::Transferable;
pub enum Msg {
    InputCode(InputData),
    InputInst(InputData),
    InputStack(InputData),
    RunnerResp(Response),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub code: String,
    pub inst_limit: u32,
    pub stack_limit: u32,
}

impl Transferable for Request {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub stdout: String,
    pub stderr: String,
    pub status: String,
}

impl Transferable for Response {}
