use crate::audio::Audio;
use raycast::prelude as rc;
use macroquad::prelude as mq;
use glam::{Vec2, IVec2};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct Context {
    pub map: rc::Map,
    pub children: Vec<rc::Entity>,
    pub props: Vec<rc::Entity>,
    pub children_speeds: Vec<f32>,
    pub door_ents: Vec<rc::Entity>,
    pub cam: rc::Ray,
    pub prev_mpos: (f32, f32),
    pub grabbed: bool,
    pub out_img: mq::Image,
    pub out_tex: mq::Texture2D,
    pub fog: rc::Fog,
    pub bg_color: mq::Color,
    pub audio: Audio,
    pub doors: HashMap<IVec2, IVec2>,
    pub flashlight_tex: mq::Texture2D,
}

impl Context {
    pub async fn new() -> Self {
        // Map setup
        let mut map: rc::Map = {
            let mut textures: HashMap<char, mq::Image> = HashMap::new();
            textures.insert('0', mq::Image::from_file_with_format(include_bytes!("res/wall.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('1', mq::Image::from_file_with_format(include_bytes!("res/wall-door.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('L', mq::Image::from_file_with_format(include_bytes!("res/wall-door-locked.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('l', mq::Image::from_file_with_format(include_bytes!("res/wall-door-unlocked.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('#', mq::Image::from_file_with_format(include_bytes!("res/fence.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('c', mq::Image::from_file_with_format(include_bytes!("res/child.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('d', mq::Image::from_file_with_format(include_bytes!("res/dead-child.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('e', mq::Image::from_file_with_format(include_bytes!("res/child.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('_', mq::Image::from_file_with_format(include_bytes!("res/chair.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('^', mq::Image::from_file_with_format(include_bytes!("res/blackboard-left.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('v', mq::Image::from_file_with_format(include_bytes!("res/blackboard-right.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('b', mq::Image::from_file_with_format(include_bytes!("res/bookshelf.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('H', mq::Image::from_file_with_format(include_bytes!("res/basketball-hoop.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('p', mq::Image::from_file_with_format(include_bytes!("res/plant.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('o', mq::Image::from_file_with_format(include_bytes!("res/locker.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('T', mq::Image::from_file_with_format(include_bytes!("res/trashcan.png"), Some(mq::ImageFormat::Png)).unwrap());
            textures.insert('P', mq::Image::from_file_with_format(include_bytes!("res/police.png"), Some(mq::ImageFormat::Png)).unwrap());

            rc::Map::from_bytes(include_bytes!("res/map"), textures)
        };
        map.floor_tex(rc::Surface::Color(mq::DARKGREEN.into()));
        map.ceil_tex(rc::Surface::Color(mq::BLUE.into()));
        map.wall_height('#', 2.);
        map.wall_height('0', 2.);
        map.wall_height('1', 2.);
        map.wall_height('L', 2.);
        map.wall_height('l', 2.);
        map.wall_height('^', 2.);
        map.wall_height('v', 2.);
        map.wall_height('b', 2.);
        map.wall_height('o', 2.);

        // Entities setup
        let children: Vec<rc::Entity> = map.filter_entities(&['c'], &[(15., 25.)]);
        let props: Vec<rc::Entity> = map.filter_entities(&['_', 'H', 'p', 'T'], &[(24., 24.), (50., 100.), (20., 50.), (20., 20.)]);

        let children_speeds: Vec<f32> = children.iter().map(|_| mq::rand::gen_range(0.2, 1.5) as f32).collect();

        let door_ents: Vec<rc::Entity> = map.filter_entities(&['e'], &[(0., 0.)]);
        let cam: rc::Ray = rc::Ray::new(Vec2::new(31. * 50., 3. * 50.), PI / 2.);

        // Mouse setup
        let prev_mpos: (f32, f32) = mq::mouse_position();
        let grabbed: bool = true;
        mq::set_cursor_grab(true);
        mq::show_mouse(false);

        // Rendering setup
        let out_img: mq::Image = mq::Image::gen_image_color(
            rc::scrw() as u16, rc::scrh() as u16, mq::BLACK
        );
        let out_tex: mq::Texture2D = mq::Texture2D::from_image(&out_img);

        let fog: rc::Fog = rc::Fog::None;
        let bg_color: mq::Color = mq::BLUE;

        // Game misc setup
        let audio: Audio = Audio::new().await;

        let mut doors: HashMap<IVec2, IVec2> = HashMap::new();
        doors.insert(IVec2::new(31, 7), IVec2::new(31, 9));

        let flashlight_tex: mq::Texture2D = mq::Texture2D::from_file_with_format(include_bytes!("res/flashlight.png"), Some(mq::ImageFormat::Png));
        Self {
            map,
            children,
            props,
            children_speeds,
            door_ents,
            cam,
            prev_mpos,
            grabbed,
            out_img,
            out_tex,
            fog,
            bg_color,
            audio,
            doors,
            flashlight_tex,
        }
    }
}
