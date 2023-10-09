mod audio;
mod shooter;
mod context;
mod util;

use context::Context;
use shooter::ShooterGame;
use raycast::prelude as rc;
use macroquad::prelude as mq;

#[macroquad::main(window_conf)]
async fn main() {
    rc::util::set_scrw_scrh(800, 800);
    let mut ctx: Context = Context::new().await;

    let mut game: ShooterGame = ShooterGame::new();
    game.run(&mut ctx).await;
}

fn window_conf() -> mq::Conf {
    mq::Conf {
        window_title: String::from("School shooter"),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}
