use std::collections::HashMap;

pub struct Layer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<u8>,
}

impl Layer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height],
        }
    }
    pub fn from_data(width: usize, height: usize, data: &[u8]) -> Self {
        Self {
            width,
            height,
            pixels: data.to_owned(),
        }
    }

    pub fn digit_count(&self) -> HashMap<u8, usize> {
        let mut map = HashMap::new();
        for &pixel in &self.pixels {
            map.entry(pixel).and_modify(|e| *e += 1).or_insert(1);
        }
        map
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.pixels[y * self.width + x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, value: u8) {
        self.pixels[y * self.width + x] = value;
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Layer> {
    let width = 25;
    let height = 6;
    input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>()
        .chunks_exact(width * height)
        .map(|chunk| Layer::from_data(width, height, chunk))
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &[Layer]) -> usize {
    let map = input
        .iter()
        .map(Layer::digit_count)
        .min_by_key(|l| l.get(&0).unwrap_or(&0).to_owned())
        .unwrap();
    map.get(&1).unwrap_or(&0) * map.get(&2).unwrap_or(&0)
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &[Layer]) -> String {
    let mut drawing = Layer::new(input[0].width, input[0].height);
    for x in 0..drawing.width {
        for y in 0..drawing.height {
            let pixel = input
                .iter()
                .map(|layer| layer.get_pixel(x, y))
                .find(|&pixel| pixel != 2)
                .unwrap();
            drawing.set_pixel(x, y, pixel);
        }
    }
    let mut render = String::new();
    render.push('\n');
    for y in 0..drawing.height {
        render.push_str(&format!(
            "{}\n",
            (0..drawing.width)
                .map(|x| drawing.get_pixel(x, y))
                .map(|p| if p == 0 { ' ' } else { '■' })
                .collect::<String>()
        ));
    }
    render
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_part2() {
        let layers = vec![
            Layer::from_data(2, 2, &[0, 2, 2, 2]),
            Layer::from_data(2, 2, &[1, 1, 2, 2]),
            Layer::from_data(2, 2, &[2, 2, 1, 2]),
            Layer::from_data(2, 2, &[0, 0, 0, 0]),
        ];
        assert_eq!("\n ■\n■ \n", solve_part2(&layers));
    }
}