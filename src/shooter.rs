use crate::context::Context;
use crate::util;
use raycast::prelude as rc;
use macroquad::prelude as mq;
use glam::{Vec2, IVec2, Vec3};
use std::f32::consts::PI;

pub struct ShooterGame {
    shooting_gun_tex: mq::Texture2D,
    items: Vec<rc::Item>,
    top_x: f32,
    top_y: f32,
    inv_ammo: i32,
    loaded_ammo: i32,
    reload_begin: f32,
    reload_unequipped: bool,
    exit_spawned: bool,
    children_alive: usize,
    show_fps: bool,
    trashed_gun: bool,
    back_outside: bool,
    police: Vec<rc::Entity>,
    win_dialogue: &'static [&'static str],
    lose_dialogue: &'static [&'static str],
    dialogue_index: usize,
}

impl ShooterGame {
    pub fn new() -> Self {
        let shooting_gun_tex: mq::Texture2D = mq::Texture2D::from_file_with_format(include_bytes!("res/gun-shoot.png"), Some(mq::ImageFormat::Png));
        let mut items: Vec<rc::Item> = vec![rc::Item::new("gun", include_bytes!("res/gun.png"))];
        rc::equip_item(&mut items, "gun");

        Self {
            shooting_gun_tex, items,
            top_x: rc::scr_topleft().0,
            top_y: rc::scr_topleft().1,
            inv_ammo: 128,
            loaded_ammo: 16,
            reload_begin: -100.,
            reload_unequipped: false,
            exit_spawned: false,
            children_alive: 0,
            show_fps: false,
            trashed_gun: false,
            back_outside: false,
            police: Vec::new(),
            win_dialogue: &[
                "You weren't the one that just shot this school up right?",
                "* I don't have a gun so I couldn't have done that.",
                "Ok, move along then.",
            ],
            lose_dialogue: &[
                "...",
                "I guess it's pretty clear who the shooter was.",
                "* I should've thrown the gun away before leaving.",
            ],
            dialogue_index: 0,
        }
    }

    pub async fn run(&mut self, ctx: &mut Context) {
        ctx.audio.play_sound("equip");

        loop {
            if mq::is_key_pressed(mq::KeyCode::Escape) {
                ctx.grabbed = !ctx.grabbed;
                mq::set_cursor_grab(ctx.grabbed);
                mq::show_mouse(!ctx.grabbed);
            }

            (self.top_x, self.top_y) = rc::scr_topleft();
            self.children_alive = ctx.children.iter().filter(|x| x.texture != 'd').collect::<Vec<&rc::Entity>>().len();

            self.update(ctx).await;
            self.render(ctx).await;
        }
    }

    async fn update(&mut self, ctx: &mut Context) {
        if !self.back_outside {
            // Controls
            rc::util::fps_camera_controls(&ctx.map, &mut ctx.cam, 2.);
            rc::util::fps_camera_rotation(&mut ctx.cam, &mut ctx.prev_mpos, 0.5);
        }

        // Game logic
        if self.children_alive == 0 && !self.exit_spawned {
            self.exit_spawned = true;
            ctx.door_ents.push(rc::Entity::new(Vec2::new(31. * 50. + 25., 9. * 50. + 25.), 'e', (0., 0.)));
            ctx.doors.insert(IVec2::new(31, 9), IVec2::new(31, 7));
        }

        for ent in ctx.door_ents.iter() {
            if ctx.cam.orig.distance(ent.pos) < 25. {
                if mq::is_key_pressed(mq::KeyCode::E) {
                    ctx.audio.play_sound("door");
                    let begin_time: f64 = mq::get_time();
                    let mut switched_pos: bool = false;
                    let ent_gpos: IVec2 = ctx.map.gpos(ent.pos);
                    let target_gpos: IVec2 = *ctx.doors.get(&ent_gpos).unwrap();

                    loop {
                        mq::clear_background(ctx.bg_color);
                        let timediff: f32 = (mq::get_time() - begin_time) as f32;
                        util::render_transition(&ctx.out_tex, timediff, 1.4);

                        if !matches!(ctx.fog, rc::Fog::None) {
                            mq::draw_texture(&ctx.flashlight_tex, self.top_x, self.top_y, mq::WHITE);
                        }

                        if timediff > 0.7 && !switched_pos {
                            switched_pos = true;
                            ctx.cam.orig = Vec2::new(target_gpos.x as f32 * 50. + 25., target_gpos.y as f32 * 50. + 25.);

                            // Initial enter ent, need to change setting
                            if ent_gpos == IVec2::new(31, 7) {
                                ctx.audio.loop_sound("bg");
                                ctx.map.floor_tex(rc::Surface::Color(mq::BEIGE.into()));
                                ctx.map.ceil_tex(rc::Surface::Color(mq::GRAY.into()));
                                ctx.bg_color = mq::BLACK;
                                ctx.fog = rc::Fog::Point(300.);
                                self.police.push(rc::Entity::new(Vec2::new(30. * 50. + 25., 5. * 50. + 25.), 'P', (30., 40.)));
                                self.police.push(rc::Entity::new(Vec2::new(30.5 * 50. + 25., 4.5 * 50. + 25.), 'P', (30., 40.)));
                                self.police.push(rc::Entity::new(Vec2::new(31. * 50. + 25., 4. * 50. + 25.), 'P', (30., 40.)));
                                self.police.push(rc::Entity::new(Vec2::new(31.5 * 50. + 25., 4.5 * 50. + 25.), 'P', (30., 40.)));
                                self.police.push(rc::Entity::new(Vec2::new(32. * 50. + 25., 5. * 50. + 25.), 'P', (30., 40.)));
                            }

                            // Final exit ent, change setting back
                            if ent_gpos == IVec2::new(31, 9) {
                                ctx.audio.stop_sound("bg");
                                ctx.map.floor_tex(rc::Surface::Color(mq::DARKGREEN.into()));
                                ctx.map.ceil_tex(rc::Surface::Color(mq::BLUE.into()));
                                ctx.bg_color = mq::BLUE;
                                ctx.fog = rc::Fog::None;
                                self.back_outside = true;
                                ctx.cam.angle = -PI / 2.;
                                ctx.cam.vangle = 0.;
                                ctx.audio.loop_sound("siren");
                            }

                            ctx.out_img.bytes.fill(0);
                            rc::render(&ctx.map, ctx.children.iter().chain(ctx.props.iter()).chain(self.police.iter()), ctx.cam, ctx.fog, &mut ctx.out_img);
                            ctx.out_tex.update(&ctx.out_img);
                        }

                        if timediff > 1.4 {
                            break;
                        }

                        mq::next_frame().await;
                    }
                }
            }
        }

        if !self.trashed_gun {
            if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
                if self.loaded_ammo == 0 {
                    ctx.audio.play_sound("empty shot");
                } else {
                    ctx.audio.play_sound("gunshot");
                    self.loaded_ammo -= 1;
                    self.items[0].texswap(&self.shooting_gun_tex, 0.1);

                    let ins: rc::Intersection = rc::cast_ray(
                        &ctx.map,
                        ctx.children.iter(),
                        &['d'], ctx.cam
                    );

                    match ins.itype {
                        rc::IntersectionType::Entity { index, .. } => {
                            ctx.children[index].texture = 'd';
                            ctx.audio.play_sound("flesh");
                        }
                        rc::IntersectionType::Wall { gpos, .. } => {
                            if ins.distance < 100. {
                                let wall_char: char = ctx.map.at(gpos.x, gpos.y);
                                if wall_char == 'L' {
                                    ctx.audio.play_sound("lock");
                                    self.create_door_entrances(ctx, gpos);
                                    ctx.map.set(gpos.x, gpos.y, 'l');
                                }
                            }
                        }
                    }
                }
            }

            for (i, child) in ctx.children.iter_mut().enumerate() {
                if child.texture == 'c' {
                    let cam_to_child: Vec2 = child.pos - ctx.cam.orig;
                    if cam_to_child.length() > 300. {
                        continue;
                    }

                    let right_or_left: f32 = -Vec3::new(cam_to_child.x, cam_to_child.y, 1.).cross(Vec3::new(ctx.cam.dir().x, ctx.cam.dir().y, 1.)).z.signum();
                    let dir: Vec2 = ctx.cam.dir().rotate(Vec2::new(mq::rand::gen_range(-3., 3.), right_or_left)) * ctx.children_speeds[i];
                    child.pos = ctx.map.move_collidable(child.pos, child.pos + dir);
                }
            }
        }

        for prop in ctx.props.iter() {
            if prop.texture == 'T' {
                if ctx.cam.orig.distance(prop.pos) < 25. && mq::is_key_pressed(mq::KeyCode::E) {
                    self.trashed_gun = true;
                    ctx.audio.play_sound("trash");
                }
            }
        }

        if !self.trashed_gun {
            if mq::is_key_pressed(mq::KeyCode::R) && mq::get_time() as f32 - self.reload_begin > 3. {
                self.reload_begin = mq::get_time() as f32;
            }

            let reload_time: f32 = mq::get_time() as f32 - self.reload_begin;
            if reload_time < 2. {
                if !self.reload_unequipped {
                    self.items[0].unequip();
                    self.reload_unequipped = true;
                    ctx.audio.play_sound("reload");
                }
            } else if reload_time < 3. {
                if self.reload_unequipped {
                    self.items[0].equip();
                    self.reload_unequipped = false;
                    self.inv_ammo -= 16 - self.loaded_ammo;
                    self.loaded_ammo = 16;

                    if self.inv_ammo < 0 {
                        self.loaded_ammo += self.inv_ammo;
                        self.inv_ammo = 0;
                    }
                }
            }
        }
    }

    fn create_door_entrances(&mut self, ctx: &mut Context, door_gpos: IVec2) {
        if ctx.map.at(door_gpos.x - 1, door_gpos.y) == '.' {
            // Horizontally oriented
            ctx.door_ents.push(rc::Entity::new(
                Vec2::new(
                    (door_gpos.x - 1) as f32 * 50. + 25.,
                    door_gpos.y as f32 * 50. + 25.
                ),
                'e', (0., 0.)
            ));
            ctx.door_ents.push(rc::Entity::new(
                Vec2::new(
                    (door_gpos.x + 1) as f32 * 50. + 25.,
                    door_gpos.y as f32 * 50. + 25.
                ),
                'e', (0., 0.)
            ));

            ctx.doors.insert(IVec2::new(door_gpos.x - 1, door_gpos.y), IVec2::new(door_gpos.x + 1, door_gpos.y));
            ctx.doors.insert(IVec2::new(door_gpos.x + 1, door_gpos.y), IVec2::new(door_gpos.x - 1, door_gpos.y));
        } else {
            // Vertically oriented
            ctx.door_ents.push(rc::Entity::new(
                Vec2::new(
                    door_gpos.x as f32 * 50. + 25.,
                    (door_gpos.y - 1) as f32 * 50. + 25.
                ),
                'e', (0., 0.)
            ));
            ctx.door_ents.push(rc::Entity::new(
                Vec2::new(
                    door_gpos.x as f32 * 50. + 25.,
                    (door_gpos.y + 1) as f32 * 50. + 25.
                ),
                'e', (0., 0.)
            ));

            ctx.doors.insert(IVec2::new(door_gpos.x, door_gpos.y - 1), IVec2::new(door_gpos.x, door_gpos.y + 1));
            ctx.doors.insert(IVec2::new(door_gpos.x, door_gpos.y + 1), IVec2::new(door_gpos.x, door_gpos.y - 1));
        }
    }

    async fn render(&mut self, ctx: &mut Context) {
        mq::clear_background(ctx.bg_color);

        // Raycast rendering
        ctx.out_img.bytes.fill(0);
        rc::render(&ctx.map, ctx.children.iter().chain(ctx.props.iter()).chain(self.police.iter()), ctx.cam, ctx.fog, &mut ctx.out_img);
        ctx.out_tex.update(&ctx.out_img);

        mq::draw_texture(&ctx.out_tex, self.top_x, self.top_y, mq::WHITE);

        if !matches!(ctx.fog, rc::Fog::None) {
            mq::draw_texture(&ctx.flashlight_tex, self.top_x, self.top_y, mq::WHITE);
        }

        if !self.trashed_gun {
            rc::render_item(&mut self.items);
        }

        // Game ui rendering
        for ent in ctx.door_ents.iter() {
            if ctx.cam.orig.distance(ent.pos) < 25. {
                mq::draw_text("[E] to enter door", self.top_x + 10., self.top_y + 20., 24., mq::WHITE);
            }
        }

        if !self.trashed_gun {
            mq::draw_line(
                self.top_x + rc::scrw() as f32 / 2. - 10.,
                self.top_y + rc::scrh() as f32 / 2.,
                self.top_x + rc::scrw() as f32 / 2. + 10.,
                self.top_y + rc::scrh() as f32 / 2.,
                2., mq::WHITE
            );

            mq::draw_line(
                self.top_x + rc::scrw() as f32 / 2.,
                self.top_y + rc::scrh() as f32 / 2. - 10.,
                self.top_x + rc::scrw() as f32 / 2.,
                self.top_y + rc::scrh() as f32 / 2. + 10.,
                2., mq::WHITE
            );
        }

        // Locked doors info
        let ins: rc::Intersection = rc::cast_ray(&ctx.map, [].iter(), &[], ctx.cam);
        match ins.itype {
            rc::IntersectionType::Wall { gpos, .. } => {
                if ctx.map.at(gpos.x, gpos.y) == 'L' && ins.distance < 40. {
                    mq::draw_text(
                        if self.trashed_gun {
                            "You need a gun to unlock this door"
                        } else {
                            "Shoot the lock (using left mouse button) to unlock the door"
                        }, self.top_x + 10., self.top_y + 20., 24., mq::WHITE
                    );
                }
            }
            _ => (),
        }

        if !self.trashed_gun {
            mq::draw_text(format!("Loaded bullets:    {}", self.loaded_ammo).as_str(), self.top_x + 10., self.top_y + rc::scrh() as f32 - 40., 24., mq::WHITE);
            mq::draw_text(format!("Inventory bullets: {}", self.inv_ammo).as_str(), self.top_x + 10., self.top_y + rc::scrh() as f32 - 20., 24., mq::WHITE);

            if self.loaded_ammo == 0 {
                mq::draw_text("[R] to reload", self.top_x + 10., self.top_y + rc::scrh() as f32 - 60., 24., mq::WHITE);
            }
        }

        if !matches!(ctx.fog, rc::Fog::None) {
            let children_left: String = if self.children_alive > 0 { format!("{} left", self.children_alive) } else { String::from("Exit the school") };
            let width: f32 = mq::measure_text(children_left.as_str(), None, 24, 1.).width;
            mq::draw_text(children_left.as_str(), self.top_x + rc::scrw() as f32 / 2. - width / 2., self.top_y + rc::scrh() as f32 - 20., 24., mq::WHITE);
        }

        if mq::is_key_pressed(mq::KeyCode::F) {
            self.show_fps = !self.show_fps;
        }

        if self.show_fps {
            mq::draw_text(format!("FPS {}", mq::get_fps()).as_str(), self.top_x + rc::scrw() as f32 - 80., self.top_y + 20., 24., mq::WHITE);
        }

        for prop in ctx.props.iter() {
            if !self.trashed_gun && prop.texture == 'T' && ctx.cam.orig.distance(prop.pos) < 25. {
                mq::draw_text("[E] to throw away gun and ammo", self.top_x + 10., self.top_y + 20., 24., mq::WHITE);
            }
        }

        if self.back_outside {
            let dialogue_vec: &[&str] = if self.trashed_gun { self.win_dialogue } else { self.lose_dialogue };
            if self.dialogue_index >= dialogue_vec.len() {
                let text: &str = if self.trashed_gun { "GOOD ENDING" } else { "BAD ENDING" };
                let measure = mq::measure_text(text, None, 64, 1.);
                mq::draw_rectangle(0., 0., rc::scrw() as f32, rc::scrh() as f32, {
                    let mut color = mq::BLACK;
                    color.a = 0.6;
                    color
                });
                mq::draw_text(text, self.top_x + rc::scrw() as f32 / 2. - measure.width / 2., self.top_y + rc::scrh() as f32 / 2. - measure.height / 2., 64., mq::WHITE);
            } else {
                mq::draw_rectangle(self.top_x + 10., self.top_y + 3. * rc::scrh() as f32 / 4., rc::scrw() as f32 - 20., rc::scrh() as f32 / 4. - 20., mq::BLACK);
                let dialogue: &str = dialogue_vec[self.dialogue_index];
                mq::draw_text(
                    dialogue.replace("* ", "").as_str(),
                    self.top_x + 20.,
                    self.top_y + 3. * rc::scrh() as f32 / 4. + 20.,
                    24., mq::WHITE
                );

                mq::draw_rectangle(self.top_x + 10., self.top_y + 3. * rc::scrh() as f32 / 4. - 30., 100., 30., mq::BLACK);
                mq::draw_text(if dialogue.contains("* ") { "You" } else { "Police" }, self.top_x + 20., self.top_y + 3. * rc::scrh() as f32 / 4. - 10., 24., mq::WHITE);
                mq::draw_text("[Space] to advance dialogue", self.top_x + 20., self.top_y + rc::scrh() as f32 - 30., 24., mq::WHITE);

                if mq::is_key_pressed(mq::KeyCode::Space) {
                    self.dialogue_index += 1;
                }
            }
        }

        mq::next_frame().await;
    }
}
