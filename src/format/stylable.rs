// stylable.rs
pub trait Stylable {
    type Output;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String;

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug;
}

// --- Implement Stylable ---

impl Stylable for String {
    type Output = String;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String,
    {
        f(self)
    }

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug,
    {
        f(self)
    }
}

impl Stylable for &str {
    type Output = String;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String,
    {
        f(self)
    }

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug,
    {
        f(self)
    }
}

// Implement Stylable for &String
impl Stylable for &String {
    type Output = String;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String,
    {
        f(self.as_str())
    }

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug,
    {
        f(self.as_str())
    }
}

impl Stylable for Vec<String> {
    type Output = Vec<String>;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String,
    {
        self.iter().map(|s| f(s)).collect()
    }

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug,
    {
        let mut out = Vec::with_capacity(self.len());
        for s in self {
            out.push(f(s)?);
        }
        Ok(out)
    }
}

impl Stylable for Vec<Vec<String>> {
    type Output = Vec<Vec<String>>;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String,
    {
        self.iter()
            .map(|row| row.iter().map(|s| f(s)).collect())
            .collect()
    }

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug,
    {
        let mut out = Vec::with_capacity(self.len());
        for row in self {
            let mut new_row = Vec::with_capacity(row.len());
            for s in row {
                new_row.push(f(s)?);
            }
            out.push(new_row);
        }
        Ok(out)
    }
}

// Forward &Vec<String> to Vec<String>
impl Stylable for &Vec<String> {
    type Output = Vec<String>;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String,
    {
        (*self).apply(f)
    }

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug,
    {
        (*self).apply_result(f)
    }
}

// Forward &Vec<Vec<String>> to Vec<Vec<String>>
impl Stylable for &Vec<Vec<String>> {
    type Output = Vec<Vec<String>>;

    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String,
    {
        (*self).apply(f)
    }

    fn apply_result<F, E>(&self, f: F) -> Result<Self::Output, E>
    where
        F: Fn(&str) -> Result<String, E>,
        E: std::fmt::Debug,
    {
        (*self).apply_result(f)
    }
}

