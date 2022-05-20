use crate::{geo::point::Point, cons, js};

/// represents a circle on the grid
pub struct Circle {
    center: Point,
    radius: f32,
}

impl Circle {

    pub fn new(center: Point, radius: f32) -> Circle {
        Circle {center, radius}
    }

    pub fn grid_fill(&self) -> Vec<Point> {
        let mut fill = Vec::new();

        let radius = self.radius;
        let center = &self.center;

        let size_y = ( radius * (0.5_f32).sqrt() ).floor() as i32;

        // TODO finish this!
        for dy in 0..=size_y {
            let fdy = dy as f32;
            let dx  = ((radius*radius - fdy * fdy) as f32).sqrt();
            let left  = (center.x as f32 - dx).ceil() as i32;
            let right = (center.x as f32 + dx).floor() as i32;

            for x in left..=right {
                fill.push(Point::new(x, center.y + dy));
            }
        }

        fill
    }

    pub fn grid_border(&self) -> Vec<Point> {
        let mut border = Vec::new();

        let radius = self.radius;
        let center = &self.center;

        let size_y = ( radius * (0.5_f32).sqrt() ).floor() as i32;

        for dy in 0..=size_y {
            let fdy = dy as f32;
            let dx  = ((radius*radius - fdy * fdy) as f32).sqrt().floor() as i32;
 
            if dy != 0 { // eliminate ortagonal duplicates
                border.push(Point::new(center.x - dx, center.y - dy));
                border.push(Point::new(center.x + dx, center.y + dy));
            }
            border.push(Point::new(center.x - dx, center.y + dy));
            border.push(Point::new(center.x + dx, center.y - dy));

            if dx == dy { continue } // eliminate diagonal duplicates

            if dy != 0 { // eliminate ortagonal duplicates
                border.push(Point::new(center.x - dy, center.y + dx));
                border.push(Point::new(center.x + dy, center.y - dx));
            }
            border.push(Point::new(center.x - dy, center.y - dx));
            border.push(Point::new(center.x + dy, center.y + dx));
        }

        border
    }

    /// from: angle in radians, 
    /// to: angle in radians,
    /// NOTE: this is not the cleanest approach
    /// - we first calculate a full circle 
    /// - we do stupid things with the angles, not foolproof
    pub fn grid_arc(&self, from: f32, to: f32) -> Vec<Point> {
        let circle = self.grid_border();

        js::print(&format!("angle range {} {}",from, to));
        
        let arc = circle.into_iter().filter(|p| {
            let angle = p.sub(&self.center).angle();
            js::print(&format!("angle range {}",angle));
            from < angle && angle <= to || 
            from < angle+cons::TWO_PI && angle+cons::TWO_PI <= to ||
            from < angle-cons::TWO_PI && angle-cons::TWO_PI <= to
        }).collect();

        arc
    } 
}
