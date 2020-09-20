extern crate sdl2;

mod tetrimino;
use crate::tetrimino::Tetrimino;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use std::fs::File;
use std::io::{self, Read, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const WIDTH: u32 = 660;
const HEIGHT: u32 = 660;
const NUM_TEXTURES: usize = 7;
const TETRIS_HEIGHT: usize = 40;
const HIGHSCORE_FILE: &'static str = "scores.txt";
const LEVEL_TIMES: [u32; 10] = [1000, 850, 700, 600, 500, 400, 300, 250, 221, 190];
const LEVEL_LINES: [u32; 10] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];
const NB_HIGHSCORES: usize = 5;

struct Tetris {
    game_over: bool,
    game_map: Vec<Vec<u8>>,
    current_level: u32,
    score: u32,
    nb_lines: u32,
    current_piece: Option<Tetrimino>,
    next_piece: Option<Tetrimino>,
}

impl Tetris {
    fn new() -> Tetris {
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
        Tetris {
            game_over: false,
            game_map: game_map,
            current_level: 1,
            score: 0,
            nb_lines: 0,
            current_piece: None,
            next_piece: None,
        }
    }

    fn update_score(&mut self, to_add: u32) {
        self.score += to_add;
    }

    fn increase_level(&mut self) {
        self.current_level += 1;
    }

    fn increase_line(&mut self) {
        self.nb_lines += 1;
        if self.nb_lines > LEVEL_LINES[self.current_level as usize - 1] {
            self.increase_level();
        }
    }

    fn check_lines(&mut self) {
        let mut y = 0;
        let mut score_add = 0;

        while y < self.game_map.len() {
            let mut complete = true;

            for x in &self.game_map[y] {
                if *x == 0 {
                    complete = false;
                    break;
                }
            }
            if complete == true {
                score_add += self.current_level;
                self.game_map.remove(y);
                y -= 1;
            }
            y += 1;
        }
        if self.game_map.len() == 0 {
            // A "tetris"!
            score_add += 1000;
        }
        self.update_score(score_add);
        while self.game_map.len() < 16 {
            self.increase_line();
            self.game_map.insert(0, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        }
    }

    fn make_permanent(&mut self) {
        let mut to_add = 0;
        if let Some(ref mut piece) = self.current_piece {
            let mut shift_y = 0;

            while shift_y < piece.states[piece.current_state as usize].len()
                && piece.y + shift_y < self.game_map.len()
            {
                let mut shift_x = 0;

                while shift_x < piece.states[piece.current_state as usize][shift_y].len()
                    && (piece.x + shift_x as isize)
                        < self.game_map[piece.y + shift_y].len() as isize
                {
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {
                        let x = piece.x + shift_x as isize;
                        self.game_map[piece.y + shift_y][x as usize] =
                            piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
            to_add += self.current_level;
        }
        self.update_score(to_add);
        self.check_lines();
        self.current_piece = None;
    }
}

fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    r: u8,
    g: u8,
    b: u8,
    width: u32,
    height: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) = texture_creator.create_texture_target(None, width, height) {
        canvas
            .with_texture_canvas(&mut square_texture, |texture| {
                texture.set_draw_color(Color::RGB(r, g, b));
                texture.clear();
            })
            .expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

fn create_texture_from_text<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    text: &str,
    r: u8,
    g: u8,
    b: u8,
) -> Option<Texture<'a>> {
    if let Ok(surface) = font.render(text).blended(Color::RGB(r, g, b)) {
        texture_creator.create_texture_from_surface(&surface).ok()
    } else {
        None
    }
}

fn get_rect_from_text(text: &str, x: i32, y: i32) -> Option<Rect> {
    Some(Rect::new(x, y, text.len() as u32 * 20, 30))
}

enum Cmd {
    Escape,
    Quit,
    Restart,
}

fn handle_events(
    tetris: &mut Tetris,
    cmd: &mut Option<Cmd>,
    timer: &mut SystemTime,
    event_pump: &mut sdl2::EventPump,
) -> bool {
    let mut make_permanent = false;
    if let Some(ref mut piece) = tetris.current_piece {
        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    *cmd = Some(Cmd::Quit);
                    break;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    *cmd = Some(Cmd::Escape);
                    break;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    *timer = SystemTime::now();
                    tmp_y += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    tmp_x += 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    tmp_x -= 1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    piece.rotate(&tetris.game_map);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let x = piece.x;
                    let mut y = piece.y;
                    while piece.change_position(&tetris.game_map, x, y + 1) == true {
                        y += 1;
                    }
                    make_permanent = true;
                }
                _ => {}
            }
        }
        if !make_permanent {
            if piece.change_position(&tetris.game_map, tmp_x, tmp_y) == false && tmp_y != piece.y {
                make_permanent = true;
            }
        }
    } else {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    *cmd = Some(Cmd::Quit);
                    break;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::F1),
                    ..
                } => {
                    *cmd = Some(Cmd::Restart);
                    break;
                }
                _ => {}
            }
        }
    }
    if make_permanent {
        tetris.make_permanent();
        *timer = SystemTime::now();
    }
    make_permanent
}

fn write_into_file(content: &str, file_name: &str) -> io::Result<()> {
    let mut f = File::create(file_name)?;
    f.write_all(content.as_bytes())
}

fn read_from_file(file_name: &str) -> io::Result<String> {
    let mut f = File::open(file_name)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn slice_to_string(slice: &[u32]) -> String {
    slice
        .iter()
        .map(|highscore| highscore.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn save_highscores_and_lines(highscores: &[u32], number_of_lines: &[u32]) -> bool {
    let s_highscores = slice_to_string(highscores);
    let s_number_of_lines = slice_to_string(number_of_lines);
    write_into_file(
        &format!("{}\n{}\n", s_highscores, s_number_of_lines),
        HIGHSCORE_FILE,
    )
    .is_ok()
}

fn line_to_slice(line: &str) -> Vec<u32> {
    line.split(" ")
        .filter_map(|nb| nb.parse::<u32>().ok())
        .collect()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file(HIGHSCORE_FILE) {
        let mut lines = content
            .splitn(3, "\n")
            .map(|line| line_to_slice(line))
            .collect::<Vec<_>>();
        if lines.len() == 3 {
            lines.pop();
            let (lines_sent, highscores) = (lines.pop().unwrap(), lines.pop().unwrap());
            Some((highscores, lines_sent))
        } else {
            None
        }
    } else {
        None
    }
}

fn update_vec(v: &mut Vec<u32>, value: u32) -> bool {
    if v.len() < NB_HIGHSCORES {
        v.push(value);
        true
    } else {
        for entry in v.iter_mut() {
            if value > *entry {
                *entry = value;
                return true;
            }
        }
        false
    }
}

fn print_game_information(tetris: &Tetris) {
    let mut new_highest_highscore = true;
    let mut new_highest_lines_sent = true;
    if let Some((mut highscores, mut lines_sent)) = load_highscores_and_lines() {
        new_highest_highscore = update_vec(&mut highscores, tetris.score);
        new_highest_lines_sent = update_vec(&mut lines_sent, tetris.nb_lines);
        if new_highest_highscore || new_highest_lines_sent {
            save_highscores_and_lines(&highscores, &lines_sent);
        }
    } else {
        save_highscores_and_lines(&[tetris.score], &[tetris.nb_lines]);
    }
    println!("Game over...");
    println!(
        "Score:           {}{}",
        tetris.score,
        if new_highest_highscore {
            " [NEW HIGHSCORE]"
        } else {
            ""
        }
    );
    println!(
        "Number of lines: {}{}",
        tetris.nb_lines,
        if new_highest_lines_sent {
            " [NEW LINES_SENT]"
        } else {
            ""
        }
    );
    println!("Current level:   {}", tetris.current_level);
}

fn display_game_information<'a>(
    tetris: &Tetris,
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    start_x_point: i32,
) {
    let score_text = format!("Score:{}", tetris.score);
    let lines_sent_text = format!("Lines:{}", tetris.nb_lines);
    let level_text = format!("Level:{}", tetris.current_level);

    let score = create_texture_from_text(&texture_creator, &font, &score_text, 255, 255, 255)
        .expect("Cannot render text");
    let lines_sent =
        create_texture_from_text(&texture_creator, &font, &lines_sent_text, 255, 255, 255)
            .expect("Cannot render text");
    let level = create_texture_from_text(&texture_creator, &font, &level_text, 255, 255, 255)
        .expect("Cannot render text");

    if tetris.game_over {
        let game_over_text = format!("Game Over");
        let restart_text = format!("F1 to restart");
        let game_over =
            create_texture_from_text(&texture_creator, &font, &game_over_text, 255, 255, 255)
                .expect("Cannot render text");
        let restart =
            create_texture_from_text(&texture_creator, &font, &restart_text, 255, 255, 255)
                .expect("Cannot render text");
        canvas
            .copy(
                &game_over,
                None,
                get_rect_from_text(&game_over_text, start_x_point, 20),
            )
            .expect("Couldn't copy text");
        canvas
            .copy(
                &restart,
                None,
                get_rect_from_text(&game_over_text, start_x_point, 55),
            )
            .expect("Couldn't copy text");
    } else {
        let keyhint_text = format!("Esc to end");
        let keyhint =
            create_texture_from_text(&texture_creator, &font, &keyhint_text, 255, 255, 255)
                .expect("Cannot render text");
        canvas
            .copy(
                &keyhint,
                None,
                get_rect_from_text(&keyhint_text, start_x_point, 20),
            )
            .expect("Couldn't copy text");
    }
    canvas
        .copy(
            &score,
            None,
            get_rect_from_text(&score_text, start_x_point, 90),
        )
        .expect("Couldn't copy text");
    canvas
        .copy(
            &lines_sent,
            None,
            get_rect_from_text(&score_text, start_x_point, 125),
        )
        .expect("Couldn't copy text");
    canvas
        .copy(
            &level,
            None,
            get_rect_from_text(&score_text, start_x_point, 160),
        )
        .expect("Couldn't copy text");
}

/// if texture is Some, it overrides tetrimino's texture
fn display_tetrimino(
    tetrimino: &tetrimino::Tetrimino,
    offs_x: i32,
    piece_x: isize,
    offs_y: i32,
    piece_y: usize,
    textures: &[sdl2::render::Texture<'_>; NUM_TEXTURES],
    texture: Option<&sdl2::render::Texture<'_>>,
    canvas: &mut Canvas<Window>,
) {
    for (line_nb, line) in tetrimino.states[tetrimino.current_state as usize]
        .iter()
        .enumerate()
    {
        for (case_nb, case) in line.iter().enumerate() {
            if *case == 0 {
                continue;
            }
            canvas
                .copy(
                    if let Some(texture) = texture {
                        texture
                    } else {
                        &textures[*case as usize - 1]
                    },
                    None,
                    Rect::new(
                        offs_x + (piece_x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32,
                        offs_y + (piece_y + line_nb) as i32 * TETRIS_HEIGHT as i32,
                        TETRIS_HEIGHT as u32,
                        TETRIS_HEIGHT as u32,
                    ),
                )
                .expect("Couldn't copy texture into window");
        }
    }
}

fn display_next_piece<'a>(
    tetris: &Tetris,
    width: u32,
    textures: &[sdl2::render::Texture<'_>; NUM_TEXTURES],
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    start_x_point: i32,
) {
    let next_text = format!("Next:");

    let next = create_texture_from_text(&texture_creator, &font, &next_text, 255, 255, 255)
        .expect("Cannot render text");

    canvas
        .copy(
            &next,
            None,
            get_rect_from_text(&next_text, start_x_point, 195),
        )
        .expect("Couldn't copy text");
    if let Some(ref tetrimino) = tetris.next_piece {
        display_tetrimino(
            tetrimino,
            width as i32 - 180 + 10,
            0,
            240 + 10,
            0,
            textures,
            None,
            canvas,
        );
    }
}

fn is_time_over(tetris: &Tetris, timer: &SystemTime) -> bool {
    match timer.elapsed() {
        Ok(elapsed) => {
            let millis = elapsed.as_secs() as u32 * 1000 + elapsed.subsec_nanos() / 1_000_000;
            millis > LEVEL_TIMES[tetris.current_level as usize - 1]
        }
        Err(_) => false,
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");
    let width = WIDTH;
    let height = HEIGHT;
    let mut timer = SystemTime::now();
    let mut event_pump = sdl_context.event_pump().expect(
        "Failed to get
          SDL event pump",
    );

    let grid_x = 10;
    let grid_y = (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2;
    let mut tetris = Tetris::new();
    tetris.next_piece = Some(Tetrimino::create_new_tetrimino());

    let window = video_subsystem
        .window("Tetris", width, height)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization failed");
    let mut font = ttf_context
        .load_font("assets/lucon.ttf", 128)
        .expect("Couldn't load the font");

    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let grey = create_texture_rect(
        &mut canvas,
        &texture_creator,
        128,
        128,
        128,
        TETRIS_HEIGHT as u32 * 10,
        TETRIS_HEIGHT as u32 * 16,
    )
    .expect(
        "Failed to create
                 a texture",
    );

    let grid = create_texture_rect(
        &mut canvas,
        &texture_creator,
        0,
        0,
        0,
        TETRIS_HEIGHT as u32 * 10,
        TETRIS_HEIGHT as u32 * 16,
    )
    .expect(
        "Failed to create
                 a texture",
    );

    let border = create_texture_rect(
        &mut canvas,
        &texture_creator,
        255,
        255,
        255,
        TETRIS_HEIGHT as u32 * 10 + 20,
        TETRIS_HEIGHT as u32 * 16 + 20,
    )
    .expect(
        "Failed to create
                 a texture",
    );

    macro_rules! texture {
        ($r:expr, $g:expr, $b:expr) => {
            create_texture_rect(
                &mut canvas,
                &texture_creator,
                $r,
                $g,
                $b,
                TETRIS_HEIGHT as u32,
                TETRIS_HEIGHT as u32,
            )
            .unwrap()
        };
    }

    let textures = [
        texture!(255, 69, 69),
        texture!(255, 220, 69),
        texture!(237, 150, 37),
        texture!(171, 99, 237),
        texture!(77, 149, 239),
        texture!(39, 218, 225),
        texture!(45, 216, 47),
    ];

    loop {
        if is_time_over(&tetris, &timer) {
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;
                make_permanent = !piece.change_position(&tetris.game_map, x, y);
            }
            if make_permanent {
                tetris.make_permanent();
            }
            timer = SystemTime::now();
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        canvas
            .copy(
                &border,
                None,
                Rect::new(
                    grid_x - 10,
                    grid_y - 10,
                    TETRIS_HEIGHT as u32 * 10 + 20,
                    TETRIS_HEIGHT as u32 * 16 + 20,
                ),
            )
            .expect("Couldn't copy texture into window");
        canvas
            .copy(
                &grid,
                None,
                Rect::new(
                    grid_x,
                    grid_y,
                    TETRIS_HEIGHT as u32 * 10,
                    TETRIS_HEIGHT as u32 * 16,
                ),
            )
            .expect("Couldn't copy texture into window");

        canvas
            .copy(
                &border,
                None,
                Rect::new(
                    width as i32 - 180,
                    240,
                    TETRIS_HEIGHT as u32 * 4 + 20,
                    TETRIS_HEIGHT as u32 * 4 + 20,
                ),
            )
            .expect("Couldn't copy texture into window");
        canvas
            .copy(
                &grid,
                None,
                Rect::new(
                    width as i32 - 180 + 10,
                    240 + 10,
                    TETRIS_HEIGHT as u32 * 4,
                    TETRIS_HEIGHT as u32 * 4,
                ),
            )
            .expect("Couldn't copy texture into window");

        if !tetris.game_over && tetris.current_piece.is_none() {
            // we need to take ownership of the option value, to move it to tetris.current_piece
            if let Some(current_piece) = tetris.next_piece.take() {
                if !current_piece.test_current_position(&tetris.game_map) {
                    print_game_information(&tetris);
                    tetris.game_over = true;
                }
                if !tetris.game_over {
                    // consume next piece
                    tetris.current_piece = Some(current_piece);
                    tetris.next_piece = Some(Tetrimino::create_new_tetrimino());
                } else {
                    // restore next piece
                    tetris.next_piece = Some(current_piece);
                }
            }
        }
        let mut cmd = None;
        if !handle_events(&mut tetris, &mut cmd, &mut timer, &mut event_pump) {
            if let Some(ref mut tetrimino) = tetris.current_piece {
                display_tetrimino(
                    tetrimino,
                    grid_x,
                    tetrimino.x,
                    grid_y,
                    tetrimino.y,
                    &textures,
                    None,
                    &mut canvas,
                );
                let x = tetrimino.x;
                let mut y = tetrimino.y;
                while tetrimino.test_position(
                    &tetris.game_map,
                    tetrimino.current_state as usize,
                    x,
                    y + 1,
                ) == true
                {
                    y += 1;
                }
                if y > tetrimino.y {
                    display_tetrimino(
                        tetrimino,
                        grid_x,
                        x,
                        grid_y,
                        y,
                        &textures,
                        Some(&grey),
                        &mut canvas,
                    );
                }
            }
        }
        if let Some(cmd) = cmd {
            match cmd {
                Cmd::Quit => {
                    break;
                }
                Cmd::Escape => {
                    if tetris.game_over {
                        break;
                    } else {
                        print_game_information(&tetris);
                        tetris.current_piece = None;
                        tetris.game_over = true;
                    }
                }
                Cmd::Restart => {
                    if tetris.game_over {
                        tetris = Tetris::new();
                        tetris.next_piece = Some(Tetrimino::create_new_tetrimino());
                    }
                }
            }
        }

        for (line_nb, line) in tetris.game_map.iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue;
                }
                canvas
                    .copy(
                        &textures[*case as usize - 1],
                        None,
                        Rect::new(
                            grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                            grid_y + line_nb as i32 * TETRIS_HEIGHT as i32,
                            TETRIS_HEIGHT as u32,
                            TETRIS_HEIGHT as u32,
                        ),
                    )
                    .expect("Couldn't copy texture into window");
            }
        }

        display_game_information(
            &tetris,
            &mut canvas,
            &texture_creator,
            &font,
            TETRIS_HEIGHT as i32 * 10 + 20,
        );

        display_next_piece(
            &tetris,
            width,
            &textures,
            &mut canvas,
            &texture_creator,
            &font,
            width as i32 - grid_x - 180 + 15,
        );

        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
