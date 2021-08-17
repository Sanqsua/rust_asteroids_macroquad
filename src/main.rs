use std::collections::HashMap;


fn window_conf()-> macroquad::window::Conf{
    macroquad::window::Conf { 
        window_title: "test".to_string(), 
        window_width: 600, 
        window_height: 600,
        ..Default::default()
    }
}
struct Asteroid {
    pos_x: f32,
    pos_y: f32,
    speed: f32,
    radius: f32,
}
impl Asteroid {
    fn new(x: f32, y: f32,s: f32, r: f32) -> Self {
        Self {
            pos_x: x,
            pos_y: y,
            speed: s,
            radius: r,
        }
    }
}
#[derive(Debug)]
struct Bullet {
            pos_x: f32,
            pos_y: f32,
    speed: f32,
    radius: f32,
    angle: f32,
}

impl Bullet {
    fn new(x: f32, y:f32,s:f32,r: f32,a: f32) -> Self{
        Self{
            pos_x: x,
            pos_y: y,
            speed: s,
            radius: r,
            angle: a,
        }
    }
}
struct Ship {
    pos_x: f32,
    pos_y: f32,
    ship_radius: f32,
    ship_angle: f32,
}
impl Ship {
    fn new(x: f32, y: f32,r: f32, a: f32) -> Self {
        Self {
            pos_x: x,
            pos_y: y,
            ship_radius: r,
            ship_angle: a,
        }
    }
}
const WINDOW_WIDTH: usize = 600;
const WINDOW_HEIGHT: usize = 600;
const BULLET_TIMER_LIMIT : f32 = 0.5f32;

#[macroquad::main(window_conf)]
async fn main() {
    let mut ship = Ship::new((WINDOW_WIDTH / 2) as f32, (WINDOW_HEIGHT / 2) as f32, 20f32, 0f32);
    let ship_distance = 10f32; 
    let turn_speed = 0.1f32; 
    let mut bullets: std::collections::HashMap<usize,Bullet> = HashMap::new();
    let mut bullet_timer = 0f32;
    let bullet_speed = 3f32;
    let mut asteroid = Asteroid::new(100f32,100f32,3f32,10f32);
    loop {

        //drawing
        //borders
        macroquad::shapes::draw_rectangle_lines(0f32, 0f32, WINDOW_WIDTH as f32, WINDOW_WIDTH as f32, 10f32, macroquad::color::WHITE);
        //asteroid
        macroquad::shapes::draw_circle(asteroid.pos_x, asteroid.pos_y, asteroid.radius, macroquad::color::GREEN);

        //ship
        macroquad::shapes::draw_circle(
            ship.pos_x ,
            ship.pos_y ,
            ship.ship_radius,
            macroquad::color::YELLOW,
        );
        //bullets
        for bullet in &mut bullets {
            bullet.1.pos_x =bullet.1.pos_x +bullet.1.angle.cos()*bullet.1.speed;
            bullet.1.pos_y =bullet.1.pos_y +bullet.1.angle.sin()*bullet.1.speed;

            macroquad::shapes::draw_circle_lines(bullet.1.pos_x , bullet.1.pos_y, bullet.1.radius, 1f32, macroquad::color::BROWN);
            if bullet.1.pos_x - bullet.1.radius > WINDOW_WIDTH as f32{
                bullet.1.pos_x -= WINDOW_WIDTH as f32+bullet.1.radius 
            }
            if bullet.1.pos_x + bullet.1.radius < 0f32 as f32{
                bullet.1.pos_x += WINDOW_WIDTH as f32+bullet.1.radius 
            }
            if bullet.1.pos_y - bullet.1.radius > WINDOW_HEIGHT as f32{
                bullet.1.pos_y -= WINDOW_HEIGHT as f32+bullet.1.radius 
            }
            if bullet.1.pos_y + bullet.1.radius < 0f32 as f32{
                bullet.1.pos_y += WINDOW_HEIGHT as f32+bullet.1.radius 
            }

        }
        macroquad::shapes::draw_circle(
            ship.pos_x+ship.ship_angle.cos()*ship_distance,
            ship.pos_y+ship.ship_angle.sin()*ship_distance,
            2f32,
            macroquad::color::RED,
        );
        // shipangle (red dot )
        macroquad::shapes::draw_circle(
            ship.pos_x+ship.ship_angle.cos()*ship_distance,
            ship.pos_y+ship.ship_angle.sin()*ship_distance,
            2f32,
            macroquad::color::RED,
        );
        if macroquad::input::is_key_down(macroquad::input::KeyCode::Right) {
            ship.ship_angle += turn_speed;
        }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::Left) {
            ship.ship_angle -= turn_speed;
        }
        let ship_speed = 2.0f32;
        if macroquad::input::is_key_down(macroquad::input::KeyCode::Up) {
            ship.pos_x = ship.pos_x +ship.ship_angle.cos()*ship_speed;
            ship.pos_y = ship.pos_y + ship.ship_angle.sin()*ship_speed;

            if ship.pos_x-ship.ship_radius > WINDOW_WIDTH as f32 {
                ship.pos_x -= WINDOW_WIDTH as f32 + ship.ship_radius;
            }
            if ship.pos_x+ship.ship_radius < 0f32 {
                ship.pos_x += WINDOW_WIDTH as f32 +ship.ship_radius;
            }
            if ship.pos_y-ship.ship_radius > WINDOW_HEIGHT as f32 {
                ship.pos_y -= WINDOW_HEIGHT as f32 + ship.ship_radius;
            }
            if ship.pos_y+ship.ship_radius < 0f32 {
                ship.pos_y += WINDOW_HEIGHT as f32 +ship.ship_radius;
            }
        }
        if macroquad::input::is_key_down(macroquad::input::KeyCode::Space){
            if bullet_timer >= BULLET_TIMER_LIMIT {
                let rng_number = macroquad::rand::rand() as usize;
                while !bullets.contains_key(&rng_number){
                    bullets.insert(
                        rng_number,
                        Bullet::new(ship.pos_x+ship.ship_angle.cos()*ship_distance,ship.pos_y+ship.ship_angle.sin()*ship_distance,bullet_speed,5f32,ship.ship_angle,));
                }
                bullet_timer = 0f32;
            }
            println!("{:?}",bullets);
        }

        bullet_timer+= macroquad::time::get_frame_time();
        println!("{}",bullet_timer);
        if objects_colliding(&ship, &asteroid){
        }else{
        }
        macroquad::window::next_frame().await
    }
}

fn objects_colliding(ship: &Ship, asteroid: &Asteroid )-> bool{
    //a^2 + b^2 = c^2
    // a^2 ist absoluter wert von asteroid.x - ship.x 
    // b^2 ist das gleiche nur für y
    // c ist nurnoch ausrechnen
    let a_quad = (asteroid.pos_x-ship.pos_x).abs()*(asteroid.pos_x-ship.pos_x).abs();
    let b_quad = (asteroid.pos_y-ship.pos_y).abs()*(asteroid.pos_y-ship.pos_y).abs();
    let c = (a_quad+b_quad).sqrt();
    if c < ship.ship_radius + asteroid.radius{ 
        return true 
 } else { return false 
}

}

