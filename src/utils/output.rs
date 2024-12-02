pub struct Output {
    part_1: String,
    part_2: String,
}

impl Output {
   pub fn new(part_1: &str, part_2: &str) -> Self{
        Self {
            part_1: part_1.to_string(),
            part_2: part_2.to_string(),
        }
   }

   pub fn print(&self) {
       println!("The solution to part one is: {}", self.part_1);
       println!("The solution to part two is: {}", self.part_2);
   }
}

