use retro_gpt_backend::{get_cfg, log, run_server};

fn main() {
    match get_cfg() {
        Some(cfg) => {
            log("Successfully got backend config");
            run_server(cfg)
        }
        None => log("Unable to get backend config. Aborting..."),
    }
}
