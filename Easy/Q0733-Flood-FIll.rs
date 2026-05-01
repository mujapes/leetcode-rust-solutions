use std::collections::VecDeque;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        struct Pixel { r: i32, c: i32 };
        let target_color = image[sr as usize][sc as usize];
        if target_color == color {return image;}
        let mut target_pixels = VecDeque::new();
        target_pixels.push_back(Pixel { r: sr, c: sc });
        while let Some(pixel) = target_pixels.pop_front() {
            let Pixel { r, c } = pixel;
            let (r, c) = (r as usize, c as usize);
            image[r][c] = color;
            if r != 0 && r-1 >= 0 && image[r-1][c] == target_color {target_pixels.push_back(Pixel { r: r as i32 - 1, c: c as i32 });}
            if r+1 < image.len() && image[r+1][c] == target_color {target_pixels.push_back(Pixel { r: r as i32 + 1, c: c as i32 });}
            if c != 0 && c-1 >= 0 && image[r][c-1] == target_color {target_pixels.push_back(Pixel { r: r as i32, c: c as i32 - 1 });}
            if c+1 < image[0].len() && image[r][c+1] == target_color {target_pixels.push_back(Pixel { r: r as i32, c: c as i32 + 1 });}
        }
        image
    }
}

// Runtime: 0 ms, Beats 100.00%
// Memory: 2.26 MB, Beats 56.10%
