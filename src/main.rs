use rand::Rng;
use simdnoise::NoiseBuilder;
use ggez::{event::EventHandler, graphics::{Color, Image, set_window_title}};

const MAP_SIZE:i32 = 500;

struct MainState {
    map: Image,
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use ggez::graphics;
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        
        graphics::draw(ctx, &self.map, graphics::DrawParam::default()).expect("error drawing map");
        graphics::present(ctx).expect("error presenting");
        Ok(())
    }
}



fn generate_map(seed:i32, octaves:u8, freq:f32, gain:f32, lacunarity:f32) -> Vec<u8> {
    let noise = NoiseBuilder::fbm_2d(MAP_SIZE as usize, MAP_SIZE as usize)
                .with_seed(seed)
                .with_freq(freq)
                .with_octaves(octaves)
                .with_gain(gain)
                .with_lacunarity(lacunarity)
                .generate_scaled(0.0, 255.0);

    let mut bytes: Vec<u8> = Vec::with_capacity((MAP_SIZE*MAP_SIZE*4) as usize);
    for _ in 0..MAP_SIZE*MAP_SIZE*4 {
        bytes.push(0);
    }

    let deepo_level:u8 = 50;
    let ocean_level:u8 = 70;
    let water_level:u8 = 100;
    let beach_level:u8 = 120;
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
            let height:u8 = noise[(x*MAP_SIZE+y) as usize].clone() as u8;
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

            if x>0 && height>=water_level && height < (noise[((x-1)*MAP_SIZE+y) as usize].clone() as u8) {
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
    bytes
}


fn main() -> ggez::GameResult {
    let mut rng = rand::thread_rng();

    // create a mutable reference to a `Context` and `EventsLoop`
    let (context, event_loop) = &mut ggez::ContextBuilder::new("Procedural", "Agustinso")
        .window_mode(ggez::conf::WindowMode::default().dimensions((MAP_SIZE+(MAP_SIZE/5)) as f32, MAP_SIZE as f32))
        .build()
        .unwrap();

    // Make a mutable reference to `MainState`

    set_window_title(&context, "Procedural 0.2");


    let octaves:u8 = 5;
    let freq:f32 = 0.05;
    let gain:f32 = 2.0;
    let lacunarity:f32 = 0.5;
    let seed:i32 = rng.gen();

    let img = Image::from_rgba8(context, MAP_SIZE as u16, MAP_SIZE as u16, &mut generate_map(seed, octaves, freq, gain, lacunarity)).expect("Error image generation");

    let main_state = &mut MainState {map: img};
    // Start the game
    ggez::event::run(context, event_loop, main_state)
}


/*

draw_text(&format!("Oct<{}>, Frq<{}>, Gin<{}>, Lac<{}>, Sed<{}>", octaves, freq, gain, lacunarity, seed).to_string(), 
                    0.0, (BUFFER_SIZE) as f32, 20.0, Color::from_rgba(0xf7, 0xf7, 0xff, 0xff));

if is_key_pressed(KeyCode::Space) {
    seed = rng.gen();
}
match get_last_key_pressed() {
    Some(key) => {
        update = true;
        if key == KeyCode::Q {
            octaves = octaves.saturating_sub(1);
        } else if key == KeyCode::W {
            octaves = octaves.saturating_add(1);
        } else if key == KeyCode::A {
            freq -= 0.01;
        } else if key == KeyCode::S {
            freq += 0.01;
        } else if key == KeyCode::Z {
            gain  -= 0.01;
        } else if key == KeyCode::X {
            gain  += 0.01;
        } else if key == KeyCode::E {
            lacunarity -= 0.01;
        } else if key == KeyCode::R {
            lacunarity += 0.01;
        } else if key == KeyCode::Enter {
            octaves = 5;
            freq = 0.05;
            gain = 2.0;
            lacunarity = 0.5;
            seed = rng.gen();
        } else if key == KeyCode::Escape {
            break;
        }

    },
    None => (),
}

if update {
    img = calc_buffer(seed, octaves, freq, gain, lacunarity);
    update = false;
}
*/