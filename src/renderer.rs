use crate::solver::{Solver, VerletObject};
use sfml::cpp::FBox;
use sfml::graphics::{CircleShape, Color, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::system::{Vector2f, Vector3f};

//TODO: Make target a generic RenderTarget Instead of a window
//TODO: Remove the need for FBox?

pub struct Renderer<'a> {
    target: &'a mut FBox<RenderWindow>,
}

impl<'a> Renderer<'a> {
    pub fn new(target: &'a mut FBox<RenderWindow>) -> Self {
        Renderer { target }
    }
    pub fn render(&mut self, solver: &mut Solver) {
        let constraint: Vector3f = solver.get_constraint();
        let mut constraint_background = CircleShape::default();
        constraint_background.set_radius(constraint.z);
        constraint_background.set_origin(Vector2f::new(constraint.z, constraint.z));
        constraint_background.set_fill_color(Color::BLACK);
        constraint_background.set_position(Vector2f::new(constraint.x, constraint.y));
        constraint_background.set_point_count(128);

        self.target.draw(&constraint_background);

        let mut circle: CircleShape = CircleShape::default();
        circle.set_radius(1.);
        circle.set_point_count(32);
        circle.set_origin(Vector2f::new(1., 1.));

        let objects: &mut Vec<VerletObject> = solver.get_objects();
        for object in objects {
            circle.set_position(object.position);
            circle.set_scale(Vector2f::new(object.radius, object.radius));
            circle.set_fill_color(object.color);
            self.target.draw(&circle);
        }
    }
}
