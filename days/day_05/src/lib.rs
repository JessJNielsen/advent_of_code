use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::{Itertools, zip_eq};

#[derive(Debug, Clone)]
struct Segment {
    start: Point,
    end: Point,
}

#[derive(Debug, Clone)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug, Clone)]
struct Diagram {
    grid: Vec<Vec<i16>>
}

impl Diagram {
    fn print(&self) {
        println!("\n## Diagram ##\n");
        for line in &self.grid {
            let string = line.iter().map(|&l| l.to_string())
                .collect::<Vec<String>>().join(" ");
            println!("{}", string);
        }
    }

    fn mark(&mut self, point: Point) {
        // println!("Marking {:?} on diagram", point);
        self.grid[point.y as usize][point.x as usize] += 1;
    }

    fn count_overlap(&self) -> usize {
        let mut count: Vec<usize> = Vec::new();
        for row in &self.grid {
            count.push(
                row.iter().filter(|&val| val > &1).count()
            );
        }
        count.iter().sum::<usize>()
    }
}

fn read_line_segments_from_file(path: &str) -> Result<Vec<Segment>, Box<dyn Error>> {
    let file = File::open(path).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut segments: Vec<Segment> = vec![];

    for line in lines {
        // Split at " -> "
        let parts: Vec<&str> = line.split(" -> ").collect();

        // Split each subpart at ",", and convert to u8
        let start: Vec<i16> = parts[0].split(',').map(|s| s.parse().unwrap()).collect();
        let end: Vec<i16> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();

        segments.push(Segment {
            start: Point { x: start[0], y: start[1], },
            end: Point { x: end[0], y: end[1], }
        });
    }

    Ok(segments)
}

fn calculate_points_in_segment(segment: &Segment) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    // Calculate list of points from segments.start x,y to segment.end x,y
    if segment.start.y == segment.end.y {
        // Vertical line on y-axis
        // Add point from smallest segment x to largest segment x with same y
        let start = cmp::min(segment.start.x, segment.end.x);
        let end = cmp::max(segment.start.x, segment.end.x);

        for i in start..=end {
            points.push(Point {
                x: i,
                y: segment.start.y
            });
        }
    } else if segment.start.x == segment.end.x {
        // Horizontal line on x-axis
        // Add point from smallest segment y to largest segment y with same x
        let start = cmp::min(segment.start.y, segment.end.y);
        let end = cmp::max(segment.start.y, segment.end.y);

        for i in start..=end {
            points.push(Point {
                x: segment.start.x,
                y: i,
            });
        }
    } else {
        // Diagonal lines
        let start_x = cmp::min(segment.start.x, segment.end.x);
        let end_x = cmp::max(segment.start.x, segment.end.x);
        let mut x_range = (start_x..=end_x).map(i16::from).collect::<Vec<i16>>();

        if start_x != segment.start.x {
            x_range.reverse();
        }

        let start_y = cmp::min(segment.start.y, segment.end.y);
        let end_y = cmp::max(segment.start.y, segment.end.y);
        let mut y_range = (start_y..=end_y).map(i16::from).collect::<Vec<i16>>();

        if start_y != segment.start.y {
            y_range.reverse();
        }

        // zip y and x into points
        for (x, y) in zip_eq(&x_range, &y_range) {
            points.push(Point { x: *x, y: *y, });
        }
    }

    // println!("points: {:?}", points);
    points
}

pub fn run() {
    println!("## Day 05: Hydrothermal Venture ##");

    let line_segments = read_line_segments_from_file("./days/day_05/data/lines.txt").unwrap();
    println!("Loaded {} segments", line_segments.len());

    let mut diagram = Diagram {
        grid: vec![vec![0; 1000]; 1000]
    };

    for segment in line_segments {
        // Calculate all points in segment
        let points = calculate_points_in_segment(&segment);

        // For each point, mark on diagram
        for point in points {
            diagram.mark(point);
        }
    }

    diagram.print();
    println!("\nOverlapping points: {}\n", diagram.count_overlap());
}