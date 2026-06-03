pub struct Canvas {
    pub grid: Vec<Vec<char>>,
}

impl Canvas {

    pub fn new(
        width: usize,
        height: usize,
    ) -> Self {

        Self {
            grid: vec![
                vec![' '; width];
                height
            ],
        }
    }

    pub fn write_text(
        &mut self,
        x: usize,
        y: usize,
        text: &str,
    ) {

        for (i, ch) in text.chars().enumerate() {

            if y < self.grid.len()
                && x + i < self.grid[0].len()
            {
                self.grid[y][x + i] = ch;
            }
        }
    }

    pub fn draw_char(
        &mut self,
        x: usize,
        y: usize,
        ch: char,
    ) {

        if y < self.grid.len()
            && x < self.grid[0].len()
        {
            self.grid[y][x] = ch;
        }
    }

    pub fn render(&self) {

        for row in &self.grid {

            let line: String =
                row.iter().collect();

            println!("{}", line);
        }
    }
}
