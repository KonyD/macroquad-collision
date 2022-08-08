use macroquad::prelude::*;

struct Player {
    x: f32,
    y: f32,
    vel: f32,
}
impl Player {
    fn update(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.y -= self.vel;
        }
        if is_key_down(KeyCode::Down) {
            self.y += self.vel;
        }
        if is_key_down(KeyCode::Right) {
            self.x += self.vel;
        }
        if is_key_down(KeyCode::Left) {
            self.x -= self.vel;
        }

        draw_rectangle(self.x, self.y, 100.0, 100.0, WHITE);
    }
}

struct Enemy {
    x: f32,
    y: f32,
}
impl Enemy {
    fn update(&mut self) {
        draw_rectangle(self.x, self.y, 100.0, 100.0, RED);
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut enemy = Enemy { x: 50.0, y: 50.0 };

    let mut player = Player {
        x: screen_height() / 2.0,
        y: screen_height() / 2.0,
        vel: 5.0,
    };

    loop {
        clear_background(Color {
            r: 0.25,
            g: 0.85,
            b: 0.45,
            a: 1.0,
        });

        enemy.update();
        player.update();

        if player.x + 100.0 >= enemy.x
            && player.x <= enemy.x + 100.0
            && player.y + 100.0 >= enemy.y
            && player.y <= enemy.y + 100.0
        {
            println!("Collided!");
        }

        next_frame().await
    }
}
