pub fn rotate_text(text: &str, degrees: f64) -> String {
    let radians = degrees.to_radians();
    let cos = radians.cos();
    let sin = radians.sin();

    let lines: Vec<&str> = text.lines().collect();
    let max_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let max_height = lines.len();

    let mut rotated_text = String::new();

    for y in 0..max_height {
        for x in 0..max_width {
            let center_x = max_width as f64 / 2.0;
            let center_y = max_height as f64 / 2.0;

            // Calculate the new coordinates after rotation
            let new_x = (cos * (x as f64 - center_x) - sin * (y as f64 - center_y) + center_x).round() as usize;
            let new_y = (sin * (x as f64 - center_x) + cos * (y as f64 - center_y) + center_y).round() as usize;

            // Append the character to the rotated text if it's within bounds
            if new_x < max_width && new_y < max_height {
                let line = lines[new_y];
                let ch = line.chars().nth(new_x).unwrap_or(' ');
                rotated_text.push(ch);
            } else {
                rotated_text.push(' '); // Padding for out-of-bounds characters
            }
        }
        rotated_text.push('\n'); // Newline after each row
    }

    rotated_text
}