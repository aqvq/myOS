// os/src/trap/context.rs

use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
/// Trap Context
pub struct TrapContext {
    /// general regs[0..31]
    pub x: [usize; 32],
    /// CSR sstatus
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
}

impl TrapContext {
    /// set stack pointer to x_2 reg (sp)
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    /// init app context
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        // println!("entry: {:x}", entry);
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        // println!("[kernel] app_init_context Done!");
        cx
    }
}
