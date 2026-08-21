use sfml::graphics::Color;
use sfml::system::{Vector2f, Vector3f};

#[derive(Default, Debug)]
pub struct VerletObject {
    pub radius: f32,
    pub color: Color,
    pub position: Vector2f,
    position_last: Vector2f,
    accel: Vector2f,
}

impl VerletObject {
    fn update(&mut self, dt: f32) {
        let displacement: Vector2f = self.position - self.position_last;
        self.position_last = self.position;
        self.position = self.position + displacement + self.accel * dt * dt;
        self.accel -= self.accel;
    }

    fn accelerate(&mut self, a: Vector2f) {
        self.accel += a;
    }

    fn set_velocity(&mut self, v: Vector2f, dt: f32) {
        self.position_last = self.position - (v * dt);
    }

    fn add_velocity(&mut self, v: Vector2f, dt: f32) {
        self.position_last -= v * dt;
    }

    fn get_velocity(&mut self, dt: f32) -> Vector2f {
        (self.position - self.position_last) / dt
    }
}

#[derive(Default)]
pub struct Solver {
    sub_steps: u32,
    gravity: Vector2f,
    constraint_center: Vector2f,
    constraint_radius: f32,
    objects: Vec<VerletObject>,
    time: f32,
    frame_dt: f32,
}

impl Solver {
    pub fn new(
        sub_steps: u32,
        gravity: f32,
        constraint_center: Vector2f,
        constraint_radius: f32,
        update_rate: u32,
    ) -> Self {
        Solver {
            sub_steps,
            constraint_center,
            constraint_radius,
            gravity: Vector2f::new(1., gravity),
            frame_dt: 1. / update_rate as f32,
            ..Default::default()
        }
    }

    pub fn get_constraint(&self) -> Vector3f {
        Vector3f::new(
            self.constraint_center.x,
            self.constraint_center.y,
            self.constraint_radius,
        )
    }
    pub fn get_objects(&mut self) -> &mut Vec<VerletObject> {
        &mut self.objects
    }
    pub fn add_object(&mut self, size: f32) {
        self.objects.push(VerletObject {
            position: self.constraint_center,
            position_last: self.constraint_center,
            accel: Default::default(),
            radius: size,
            color: Color::WHITE,
        })
    }

    pub fn update(&mut self) {
        self.time += self.frame_dt;
        let step_dt: f32 = self.get_step_dt();
        for _ in 0..self.sub_steps {
            self.apply_gravity();
            //            self.check_collisions(step_dt);
            self.apply_constraint();
            self.update_objects(step_dt);
        }
    }

    pub fn get_step_dt(&self) -> f32 {
        self.frame_dt / self.sub_steps as f32
    }

    fn apply_gravity(&mut self) {
        for obj in &mut self.objects {
            obj.accelerate(self.gravity);
        }
    }

    fn update_objects(&mut self, dt: f32) {
        for obj in &mut self.objects {
            obj.update(dt);
        }
    }

    fn apply_constraint(&mut self) {
        for object in &mut self.objects {
            let v = self.constraint_center - object.position;
            let dist = f32::sqrt(v.x * v.x + v.y * v.y);
            if dist > (self.constraint_radius - object.radius) {
                let n = v / dist;
                object.position =
                    self.constraint_center - n * (self.constraint_radius - object.radius);
            }
        }
    }
}
