use crate::backend_types::Types;
use common_frontend::FrontendBox;
use imgui::DrawContext;

pub fn menu(
    draw_ctx: &mut DrawContext,
    file_reader: &mut file_reader::FileReader<Types>,
    frontend: &mut FrontendBox,
) {
    let ui = draw_ctx.ui();
    ui.menu("Open As", || {
        ui.menu("Nintendo", || {
            if ui.button("GBC (Game Boy Color)") {
                file_reader.read_file(Types::NintendoGBC, vec![("all", &["gbc", "gb"])]);
            }
        });
    });
    if ui.button("Stop Emulation") {
        frontend.swap(Box::new(crate::default_backend::EmptyFrontend::new()));
    }
}
