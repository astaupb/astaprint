use std::ops::Sub;

#[derive(Debug, Clone)]

pub struct CounterOids
{
    pub total: Vec<u64>,
    pub print_black: Vec<u64>,
    pub print_color: Option<Vec<u64>>,
    pub copy_black: Vec<u64>,
    pub copy_color: Option<Vec<u64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct CounterValues
{
    pub total: u64,
    pub print_black: u64,
    pub print_color: Option<u64>,
    pub copy_black: u64,
    pub copy_color: Option<u64>,
}

impl Sub for CounterValues
{
    type Output = CounterValues;

    fn sub(self, other: CounterValues) -> CounterValues
    {
        let total = self.total - other.total;

        let print_black = self.print_black - other.print_black;

        let copy_black = self.copy_black - other.copy_black;

        let print_color = match (self.print_color, other.print_color) {
            (Some(some), Some(other)) => Some(some - other),
            _ => None,
        };

        let copy_color = match (self.copy_color, other.copy_color) {
            (Some(some), Some(other)) => Some(some - other),
            _ => None,
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
