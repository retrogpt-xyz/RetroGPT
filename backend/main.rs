use retro_gpt_backend::{get_cfg, run_server};

fn main() {
    get_cfg().map(|cfg| run_server(cfg));
}
