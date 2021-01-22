pub struct Field {
    pub field: Vec<Vec<char>>,
}
impl Field {
    pub fn new(field: Vec<&str>) -> Self {
        Field {
            field: field.iter().map(|line| line.chars().collect()).collect(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.field.get(y).and_then(|line| line.get(x))
    }
}
