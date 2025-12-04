use regex::Regex;

pub struct RobotFactory {
    x: usize,
    y: usize,
    re: regex::Regex,
}

impl RobotFactory {
    pub fn new(x: usize, y: usize) -> RobotFactory {
        RobotFactory {
            x,
            y,
            re: Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap(),
        }
    }

    pub fn new_robot(&self, line: &str) -> Robot {
        let caps = self.re.captures(line).unwrap();
        Robot {
            px: caps[1].parse().unwrap(),
            py: caps[2].parse().unwrap(),
            vx: caps[3].parse().unwrap(),
            vy: caps[4].parse().unwrap(),
            x: &self.x,
            y: &self.y,
        }
    }
}

#[derive(Debug)]
pub struct Robot<'a> {
    pub px: i32,
    pub py: i32,
    pub vx: i32,
    pub vy: i32,
    pub x: &'a usize,
    pub y: &'a usize,
}

impl<'a> Robot<'a> {
    pub fn walk(&mut self, seconds: usize) {
        self.px = (self.px + self.vx * seconds as i32) % *self.x as i32;
        if self.px < 0 {
            self.px += *self.x as i32;
        }
        self.py = (self.py + self.vy * seconds as i32) % *self.y as i32;
        if self.py < 0 {
            self.py += *self.y as i32;
        }
    }

    pub fn quadrant(&self) -> usize {
        if self.px < (*self.x as i32) / 2 && self.py < (*self.y as i32) / 2 {
            1
        } else if self.px > (*self.x as i32) / 2 && self.py < (*self.y as i32) / 2 {
            2
        } else if self.px < (*self.x as i32) / 2 && self.py > (*self.y as i32) / 2 {
            3
        } else if self.px > (*self.x as i32) / 2 && self.py > (*self.y as i32) / 2 {
            4
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robot_from_line() {
        let line = "p=0,4 v=3,-3";
        let rf = RobotFactory::new(11, 7);
        let r = rf.new_robot(line);
        assert_eq!(r.px, 0);
        assert_eq!(r.py, 4);
        assert_eq!(r.vx, 3);
        assert_eq!(r.vy, -3);
        assert_eq!(r.x, &11);
        assert_eq!(r.y, &7);
    }

    #[test]
    fn robot_walk() {
        let line = "p=0,4 v=3,-3";
        let rf = RobotFactory::new(11, 7);
        let mut r = rf.new_robot(line);
        r.walk(1);
        assert_eq!(r.px, 3);
        assert_eq!(r.py, 1);

        r.walk(1);
        assert_eq!(r.px, 6);
        assert_eq!(r.py, 5);

        r.walk(2);
        assert_eq!(r.px, 1);
        assert_eq!(r.py, 6);
    }

    #[test]
    fn robot_region() {
        let rf = RobotFactory::new(11, 7);
        let line = "p=0,0 v=3,-3";
        let r = rf.new_robot(line);
        assert_eq!(r.quadrant(), 1);
        let line = "p=10,0 v=3,-3";
        let r = rf.new_robot(line);
        assert_eq!(r.quadrant(), 2);
        let line = "p=5,3 v=3,-3";
        let r = rf.new_robot(line);
        assert_eq!(r.quadrant(), 0);
    }
}
