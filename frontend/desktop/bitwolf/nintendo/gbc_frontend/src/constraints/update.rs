use crate::{backend::debug::messages::CtoF, GBC};
use common_frontend::constraints::update::Update;

impl Update for GBC {
    fn update(&mut self) {
        while let Some(msg) = self.msgq.try_recv() {
            match msg {
                CtoF::RegisterFile(reg_file) => self.state.reg_file = reg_file,
                CtoF::Control(ctrl) => self.state.ctrl = ctrl,
                CtoF::Disassembly(disasm) => self.state.disasm = disasm,
            }
        }
    }
}
