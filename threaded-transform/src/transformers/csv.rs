use std::fmt;

/// A structure that represent a single row (can be header) in the Csv table.
pub struct Row<'a> {
    pub cell_width: usize,
    pub column_sep: char,
    pub data: &'a Vec<String>,
}

impl fmt::Display for Row<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        _ = write!(f, "{}", self.column_sep);
        for s in self.data {
            let diff = self.cell_width - s.len();
            let mut cell = s.clone();
            let mut i = 0;
            loop {
                if i == diff {
                    break;
                }
                cell.insert(cell.len(), ' ');
                i+=1;
            }
            _ = write!(f, "{}{}", cell, self.column_sep);
        }
        write!(f, "\n")
    }
}


/// A structure that encapsulate the logic for building a Cvs table.
pub struct Csv {
    pub header: Vec<String>,
    pub header_sep: char,
    pub rows: Vec<Vec<String>>,
    max_cell_width: usize,
    table_width: usize,
}

impl Csv {
    /// Construct and initialize a new instance from Csv.
    pub fn new(header: Vec<String>, rows: Vec<Vec<String>>, header_sep: char) -> Csv {
        let mut csv = Csv {
            header: header,
            rows: rows,
            header_sep: header_sep,
            max_cell_width: 0,
            table_width: 0,
        };

        csv.init();

        csv
    }

    fn init(&mut self) {
        // calculate the max cell width
        let mut max_cell_width: usize = 0;

        for cell in &self.header {
            if max_cell_width > cell.len() {
                continue;
            }
            max_cell_width = cell.len();
        }

        for row in &self.rows {
            for cell in row {
                if max_cell_width > cell.len() {
                    continue;
                }
                max_cell_width = cell.len();
            }
        }
        self.max_cell_width = max_cell_width;
        self.table_width = (max_cell_width * &self.header.len()) + &self.header.len() + 1;
    }
}

impl fmt::Display for Csv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {        
        // rendering the header top line
        for _ in 1..=self.table_width {
            _ = write!(f, "{}", self.header_sep);
        }
        _ = write!(f, "\n");

        // rendering the header row
        let r = Row { cell_width: self.max_cell_width, column_sep: '|', data: &self.header };
        _ = r.fmt(f);
        
        // rendering the header bottom line
        for _ in 1..=self.table_width {
            _ = write!(f, "{}", self.header_sep);
        }
        _ = write!(f, "\n");

        // rendering the data rows
        for row in &self.rows {
            let r = Row {cell_width: self.max_cell_width, column_sep: '|', data: &row };
            _ = r.fmt(f);
        }

        // rendering the table bottom line
        for _ in 1..=self.table_width {
            _ = write!(f, "{}", self.header_sep);
        }
        _ = write!(f, "\n");
        Ok(())
    }
}