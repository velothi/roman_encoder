pub mod roman {
  pub trait Roman {
    /// Encodes unicode string into roman numerals
    /// # Examples
    /// ```
    /// println!("Hello, world! : {}", "Hello, world!".roman())
    /// /* LXXII CI CVIII CVIII CXI XLIV XXXII CXIX CXI CXIV CVIII C XXXIII */
    /// ```
    fn roman(&self) -> String;
    // TODO: unroman(&self) -> String
  }

  impl Roman for &str {
    fn roman(&self) -> String {
      let mut ord:               i64;
      let mut min_ord:        String = String::new();
      let mut encoded:   Vec<String> = Vec::new();
      let alphabet: Vec<(&str, i64)> =
          vec![("M'", 1_000_000), ("C'M'", 900_000),
               ("D'",   500_000), ("C'D'", 400_000),
               ("C'",   100_000), ("X'C'",  90_000),
               ("L'",    50_000), ("X'L'",  40_000),
               ("X'",    10_000), ("MX'",     9000),
               ("V'",      5000), ("MV'",     4000),
               ("M",       1000), ("CM",       900),
               ("D",        500), ("CD",       400),
               ("C",        100), ("XC",        90),
               ("L",         50), ("XL",        40),
               ("X",         10), ("IX",         9),
               ("V",          5), ("IV",         4),
               ("I",          1)];
      for c in self.chars() {
        ord = c as i64;
        for (symbol, value) in &alphabet {
          while value <= &ord {
            min_ord += symbol;
            ord     -= value;
          }
        }
        encoded.push(min_ord);
        min_ord = String::new();
      }
      return encoded.join(" ");
    }
  }
}