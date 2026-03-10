/// A trait for applying terminal styling to different types of text data.
///
/// `Stylable` allows formatting functions to operate on single strings,
/// flat vectors, or even 2D vectors (grids) of strings.
pub trait Stylable {
    /// The resulting type after styling is applied.
    type Output;

    /// Applies a non-failing styling closure to each element of the data.
    fn apply<F>(&self, f: F) -> Self::Output
    where
        F: Fn(&str) -> String;

    /// Applies a fallible styling closure to each element of the data.
    ///
    /// # Errors
    ///
    /// Returns an error if any single application of the closure fails.
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
