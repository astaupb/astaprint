use std::ops::Sub;

#[derive(Debug, Clone)]
pub struct CounterOids {
    pub total: Vec<u64>,
    pub print_black: Vec<u64>,
    pub print_color: Option<Vec<u64>>,
    pub copy_black: Vec<u64>,
    pub copy_color: Option<Vec<u64>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterValues {
    pub total: u64,
    pub print_black: u64,
    pub print_color: Option<u64>,
    pub copy_black: u64,
    pub copy_color: Option<u64>,
}

impl Sub for CounterValues {
    type Output = CounterValues;
    fn sub(self, other: CounterValues) -> CounterValues {
        let total = self.total - other.total;
        let print_black = self.print_black - other.print_black;
        let copy_black = self.copy_black - other.copy_black;
        let print_color = match self.print_color.is_some() && other.print_color.is_some() {
            true => Some(self.print_color.unwrap() - other.print_color.unwrap()),
            false => None,
        };
        let copy_color = match self.copy_color.is_some() && other.copy_color.is_some() {
            true => Some(self.copy_color.unwrap() - other.copy_color.unwrap()),
            false => None,
        };
        CounterValues {
            total,
            print_black,
            copy_black,
            print_color,
            copy_color,
        }
    }
}