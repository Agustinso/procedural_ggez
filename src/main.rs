use rand::Rng;
use simdnoise::NoiseBuilder;
use ggez::{Context, event::{EventHandler, KeyMods}, graphics::{Color, Image, set_window_title}, input::keyboard};

const MAP_SIZE:i32 = 500;

struct Map {
    data: Vec<u8>,
    octaves:u8,
    freq:f32,
    gain:f32,
    lacunarity:f32,
    seed:i32,
}

impl Map {
    fn new(seed:i32) -> Self {
        Map{ 
            data: [0].to_vec(), 
            octaves:  5, 
            freq:  0.05, 
            gain:  2.0, 
            lacunarity:  0.5, 
            seed 
        }
    }

    fn generate_map(&mut self){
        let noise = NoiseBuilder::fbm_2d(MAP_SIZE as usize, MAP_SIZE as usize)
                    .with_seed(self.seed)
                    .with_freq(self.freq)
                    .with_octaves(self.octaves)
                    .with_gain(self.gain)
                    .with_lacunarity(self.lacunarity)
                    .generate_scaled(0.0, 1.0);

        let mut bytes: Vec<u8> = Vec::with_capacity((MAP_SIZE*MAP_SIZE*4) as usize);
        for _ in 0..MAP_SIZE*MAP_SIZE*4 {
            bytes.push(0);
        }

        let deepo_level:u8 = 50-45;
        let ocean_level:u8 = 70-45;
        let water_level:u8 = 100-45;
        let beach_level:u8 = 120-45;
        let pltau_level:u8 = 140;
        let mount_level:u8 = 170;
        let peaky_level:u8 = 210;
        let snowy_level:u8 = 240;

        let deepo_color = (18u8,47u8,95u8);
        let ocean_color = (23u8,60u8,114u8);
        let water_color = (23u8,60u8,114u8);
        let beach_color = (249u8,217u8,134u8);
        let pltau_color = (69u8,127u8,31u8);
        let mount_color = (36u8,35u8,36u8);
        let peaky_color = (63u8,51u8,45u8);
        let snowy_color = (255u8,255u8,255u8);

        let mut i = 0;
        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                let height:u8 = (noise[(x*MAP_SIZE+y) as usize].clone().powf(4.0)*255.0) as u8;
                let mut next = height;
                if x>0 {
                    next = (noise[((x-1)*MAP_SIZE+y) as usize].clone().powf(4.0)*255.0) as u8;
                }
                let mut color = (104u8,154u8,25u8);

                if height>=deepo_level {
                    if height>=ocean_level {
                        if height>=water_level {
                            if height>=beach_level {
                                if height>=pltau_level {
                                    if height>=mount_level {
                                        if height>=peaky_level {
                                            if height>=snowy_level {
                                                color = snowy_color;
                                            } else {
                                                color = mount_color;
                                            }
                                        } else {
                                            color = peaky_color;
                                        }
                                        
                                    }
                                } else {
                                    color = pltau_color;
                                }
                            } else {
                                color = beach_color;
                            }
                        } else {
                            color = water_color;
                        }
                    } else {
                        color = ocean_color;
                    }
                } else {
                    color = deepo_color;
                }


                if x>0 && height>=water_level && height < next {
                    color.0 = (0.7*(color.0 as f32)) as u8;
                    color.1 = (0.7*(color.1 as f32)) as u8;
                    color.2 = (0.7*(color.2 as f32)) as u8;
                }

                bytes[i] = color.0;
                bytes[i+1] = color.1;
                bytes[i+2] = color.2;
                bytes[i+3] = 255;
                i += 4;
            }
        }
        self.data = bytes;
    }

    fn to_image(&mut self, context: &mut Context, map_size: u16) -> Image {
        Image::from_rgba8(context, map_size, map_size, &mut self.data).expect("Error image generation")
    }
}

struct MainState {
    map: Map,
    img: Image,
    update_map: bool,
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut mult = 1;
        if keyboard::is_mod_active(ctx, KeyMods::SHIFT) {
            mult = 5;
        }

        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Q) {
            self.map.octaves = self.map.octaves.saturating_sub(1*mult);
            self.update_map = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            self.map.octaves = self.map.octaves.saturating_add(1*mult);
            self.update_map = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::A) {
            self.map.freq -= 0.01*(mult as f32);
            self.update_map = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S) {
            self.map.freq += 0.01*(mult as f32);
            self.update_map = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Z) {
            self.map.gain  -= 0.01*(mult as f32);
            self.update_map = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::X) {
            self.map.gain  += 0.01*(mult as f32);
            self.update_map = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::E) {
            self.map.lacunarity -= 0.01*(mult as f32);
            self.update_map = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::R) {
            self.map.lacunarity += 0.01*(mult as f32);
            self.update_map = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Return) {
            let mut rng = rand::thread_rng();
            self.map.seed = rng.gen();
            self.map.octaves = 5;
            self.map.freq = 0.05;
            self.map.gain = 2.0;
            self.map.lacunarity = 0.5;
            self.update_map = true;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Space) {
            let mut rng = rand::thread_rng();
            self.map.seed = rng.gen();
            self.update_map = true;
        }

        if self.update_map {
            self.map.generate_map();
            self.img = self.map.to_image(ctx, MAP_SIZE as u16);
            self.update_map = false;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use ggez::graphics;
        graphics::clear(ctx, Color::from_rgb(0x12, 0x12, 0x12));
        
        graphics::draw(ctx, &self.img, graphics::DrawParam::default()).expect("error drawing map");

        let params_text = 
           graphics::Text::new(format!(
            "Value:         [+][-]\nOctaves: \t{}[Q][W]\nFrquency: {}[A][S]\nGain: {}[Z][X]\nLacunarity: {}[E][R]\nSeed: {}[Space]\n[Enter]Restarts values\n[Shift] Changes x5", 
            self.map.octaves, self.map.freq, self.map.gain, self.map.lacunarity, self.map.seed));
        let params = graphics::DrawParam::default().dest([(MAP_SIZE as f32)+1.0, 0.0]).color(Color::from_rgb(0xf7, 0xf7, 0xff));
        graphics::draw(ctx, &params_text, params).expect("error drawing scoreboard text");

        graphics::present(ctx).expect("error presenting");
        Ok(())
    }
}


fn main() -> ggez::GameResult {
    let mut rng = rand::thread_rng();

    // create a mutable reference to a `Context` and `EventsLoop`
    let (context, event_loop) = &mut ggez::ContextBuilder::new("Procedural", "Agustinso")
        .window_mode(ggez::conf::WindowMode::default().dimensions((MAP_SIZE+(MAP_SIZE/3)) as f32, MAP_SIZE as f32))
        .build()
        .unwrap();

    // Make a mutable reference to `MainState`

    set_window_title(&context, "Procedural 0.3");
    let mut map = Map::new(rng.gen());
    map.generate_map();
    let img = map.to_image(context, MAP_SIZE as u16);
    let main_state = &mut MainState {map, img, update_map: false};
    // Start the game
    ggez::event::run(context, event_loop, main_state)
}