pub(crate) mod debug;

mod access;
mod dma;
mod palette;
mod regs;
mod rendering;
mod shift_register;
mod sprites;
mod states;

pub(crate) use access::PPUReg;
pub(crate) use rendering::lcd;

use crate::interfaces::VideoInterface;

pub(crate) struct PPU {
    pub(crate) if_stat: bool,
    pub(crate) if_vblank: bool,
    regs: regs::Regs,
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    bg_win_sr: shift_register::ShiftRegister,
    sprite_sr: shift_register::ShiftRegister,
    fetcher: rendering::fetcher::Fetcher,
    sprite_buffer: sprites::SpriteBuffer,
    frame: crate::Texture,
    frame_state: states::FrameState,
    scanline_state: states::ScanlineState,
    video_interface: VideoInterface,
}

impl PPU {
    pub fn new(video_interface: VideoInterface) -> Self {
        Self {
            if_stat: false,
            if_vblank: false,
            vram: [0; 0x2000],
            oam: [0; 0xA0],
            regs: regs::Regs::new(),
            bg_win_sr: shift_register::ShiftRegister::new(),
            sprite_sr: shift_register::ShiftRegister::new(),
            fetcher: rendering::fetcher::Fetcher::new(),
            sprite_buffer: sprites::SpriteBuffer::new(),
            frame: crate::Texture::default(),
            frame_state: states::FrameState::new(),
            scanline_state: states::ScanlineState::new(),
            video_interface,
        }
    }

    fn reset(&mut self) {
        self.regs.lcds.mode = rendering::scanline::Mode::HBlank;
        self.scanline_state.reset();
        self.frame_state.reset();
        self.bg_win_sr.clear();
        self.sprite_sr.clear();
        self.frame = crate::Texture::default();
    }
}
