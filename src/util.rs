use macroquad::prelude as mq;
use raycast::prelude as rc;

pub fn render_transition(out_tex: &mq::Texture2D, timediff: f32, total_time: f32) {
    let (top_x, top_y) = rc::scr_topleft();
    mq::draw_texture(&out_tex, top_x, top_y, mq::WHITE);

    if timediff < total_time / 2. {
        let mut color: mq::Color = mq::BLACK;
        color.a = timediff / (total_time / 2.);
        mq::draw_rectangle(top_x, top_y, rc::scrw() as f32, rc::scrh() as f32, color)
    } else if timediff < total_time {
        let mut color: mq::Color = mq::BLACK;
        color.a = 1. - (timediff - total_time / 2.) / (total_time / 2.);
        mq::draw_rectangle(top_x, top_y, rc::scrw() as f32, rc::scrh() as f32, color)
    }
}
