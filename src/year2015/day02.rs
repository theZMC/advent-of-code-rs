use std::fmt::Display;

use anyhow::Result;

use crate::{challenge::Fetcher, year2015::YEAR};

pub fn solve(fetcher: &Fetcher) -> Result<(Box<dyn Display>, Box<dyn Display>)> {
    let challenge = fetcher.fetch_challenge(YEAR, 2)?;

    let mut area = 0;
    let mut ribbon_length = 0;

    challenge.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }

        let dimensions: Vec<&str> = line.split('x').collect();

        let length = dimensions[0].parse::<u32>().unwrap();
        let width = dimensions[1].parse::<u32>().unwrap();
        let height = dimensions[2].parse::<u32>().unwrap();
        let smallest_dimension = (length * width).min(width * height).min(height * length);

        let lwface = length * width;
        let whface = width * height;
        let hlface = height * length;
        let smallest_face = (length + width).min(width + height).min(height + length);

        area += 2 * lwface + 2 * whface + 2 * hlface + smallest_dimension;
        ribbon_length += 2 * smallest_face + length * width * height;
    });

    Ok((Box::new(area), Box::new(ribbon_length)))
}
