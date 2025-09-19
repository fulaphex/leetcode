use std::collections::HashMap;

struct Spreadsheet {
    values: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        *self.values.entry(cell).or_insert(0) = value;
    }

    fn reset_cell(&mut self, cell: String) {
        self.values.remove(&cell);
    }

    fn get_value(&self, formula: String) -> i32 {
        let fields = formula[1..].split('+').collect::<Vec<_>>();
        assert_eq!(fields.len(), 2);

        return fields
            .into_iter()
            .map(|f| {
                if f.chars().next().unwrap().is_alphabetic() {
                    *self.values.get(&String::from(f)).unwrap_or(&0)
                } else {
                    f.parse::<i32>().unwrap()
                }
            })
            .sum();
    }
}

/**
 * Your Spreadsheet object will be instantiated and called as such:
 * let obj = Spreadsheet::new(rows);
 * obj.set_cell(cell, value);
 * obj.reset_cell(cell);
 * let ret_3: i32 = obj.get_value(formula);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut spreadsheet = Spreadsheet::new(3);
        assert_eq!(spreadsheet.get_value(String::from("=5+7")), 12);
        spreadsheet.set_cell(String::from("A1"), 10);
        assert_eq!(spreadsheet.get_value(String::from("=A1+6")), 16);
        spreadsheet.set_cell(String::from("B2"), 15);
        assert_eq!(spreadsheet.get_value(String::from("=A1+B2")), 25);
        spreadsheet.reset_cell(String::from("A1"));
        assert_eq!(spreadsheet.get_value(String::from("=A1+B2")), 15);
    }
}
