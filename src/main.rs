#[derive(Debug)]
enum Value {
    Int(i64),
    String(String),
    Bool(bool),
}

#[derive(Debug)]
enum Column {
    Int(Vec<i64>),
    String(Vec<String>),
    Bool(Vec<bool>),
}

impl Column {
    pub fn num_rows(&self) -> usize {
        match self {
            Column::Int(v) => v.len(),
            Column::String(v) => v.len(),
            Column::Bool(v) => v.len(),
        }
    }
}

#[derive(Debug)]
struct DataFrame {
    column_names: Vec<String>,
    column_data: Vec<Column>,
    num_rows: usize,
}

impl DataFrame {
    pub fn new() -> Self {
        Self {
            column_names: vec![],
            column_data: vec![],
            num_rows: 0,
        }
    }

    pub fn add_column(
        &mut self,
        name: String,
        data: Column,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.column_names.is_empty() || data.num_rows() == self.num_rows {
            // Everything matches, so we're good
            self.num_rows = data.num_rows();
            self.column_names.push(name);
            self.column_data.push(data);
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "mismatched number of rows",
            )))
        }
    }

    pub fn add_row(&mut self, row: &[Value]) -> Result<(), Box<dyn std::error::Error>> {
        if self.column_names.is_empty() {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "add_row needs initial column names",
            )))
        } else if self.column_names.len() != row.len() {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "add_row with a row that doesn't match number of columns",
            )))
        } else {
            // match the type as we go
            for (column, data) in self.column_data.iter_mut().zip(row) {
                match (column, data) {
                    (Column::Int(ci), Value::Int(i)) => ci.push(*i),
                    (Column::String(cs), Value::String(s)) => cs.push(s.clone()),
                    (Column::Bool(cb), Value::Bool(b)) => cb.push(*b),
                    _ => {
                        return Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "attempt to add row with mismatched types",
                        )))
                    }
                }
            }
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let mut frame = DataFrame::new();
    frame.add_column(
        "Name".into(),
        Column::String(vec!["Joe".into(), "Sally".into(), "Sam".into()]),
    )?;

    frame.add_column("Age".into(), Column::Int(vec![11, 100, 1]))?;

    frame.add_row(&[Value::String("Bob".into()), Value::Int(101)])?;

    println!("{:?}", frame);

    Ok(())
}
