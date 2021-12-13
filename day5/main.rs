use std::fs;
use std::fmt;
use std::collections::HashMap;
use std::hash::{Hash,Hasher};
use std::cmp;
use std::convert::TryFrom;

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Eq)]
struct Point{
    x:u16,
    y:u16
}
struct Line(Point, Point);

impl Line{
    fn new(raw_data: &str) -> Self {
        let v: Vec<Point> = raw_data
            .split(" -> ")
            .map(|p| Point::from_string(p))
            .collect();

            return Self(v[0],v[1]);
    }    
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.x.hash(hasher);
        self.y.hash(hasher);
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}
impl Point{
    fn from_string(raw_data: &str) -> Self {
        let coordinates: Vec<u16> = raw_data
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect();
        return Self{x:coordinates[0],y:coordinates[1]};
    } 
    fn new(x: u16,y:u16) -> Self {
        return Self{x:x,y:y};
    }   
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} -> {})", self.0, self.1)
    }
}


struct Area{
    visited_area: HashMap<Point,u8>,
}


impl Area{
    fn new() -> Self {
        return Self{
            visited_area: HashMap::new()
        };
    }
    
    fn add(&mut self, point: Point) {
        //println!("adding {}", point); 
        let dict = &mut self.visited_area;
        let result = dict.get(&point); 
        match result{
            None => dict.insert(point, 1),
            Some(n) => dict.insert(point, n +1)
        };
    } 
}

fn main() {
    
    let content = fs::read_to_string("input.txt")
    .expect("war richtig");

    let lines= content
    .split("\r\n")
    .map(|x| Line::new(x));

    let mut area = Area::new();

    for line in lines {
        
        let (p1,p2) = (line.0,line.1);
        if p1.x == p2.x{
            
            for elem in cmp::min(p1.y,p2.y)..=cmp::max(p1.y,p2.y) {
                area.add(Point::new(p1.x, elem));
            }
        } else if p1.y == p2.y{
            for elem in cmp::min(p1.x,p2.x)..=cmp::max(p1.x,p2.x) {
                area.add(Point::new(elem, p1.y));
            }
        }  else{
            let distance = cmp::max(p1.y,p2.y) - cmp::min(p1.y,p2.y);
            let x_step:i16 = if p1.x < p2.x { 1 }else{-1};
            let y_step:i16 = if p1.y < p2.y { 1 }else{-1};
            for index in 0..=distance {
                let x:u16 =  u16::try_from((p1.x as i16) +  x_step * (index as i16)).unwrap();
                let y:u16 =  u16::try_from((p1.y as i16) +  y_step * (index as i16)).unwrap();
                area.add(Point::new(x,y));
            }
        }             
    }

    let count = area.visited_area.values().filter(|x| **x >= 2).count();
    assert_eq!(count,15463);
    println!("Count {}", count); 
}