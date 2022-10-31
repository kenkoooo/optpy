pub trait StripMargin {
    fn strip_margin(&self) -> String;
}

impl<S> StripMargin for S
where
    S: AsRef<str>,
{
    fn strip_margin(&self) -> String {
        let mut lines = vec![];
        for line in self.as_ref().split('\n') {
            lines.push(
                line.chars()
                    .skip_while(|&c| c != '|')
                    .skip(1)
                    .collect::<String>(),
            )
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strip_margin() {
        let x = r"
        |abcd
        |efgh
        "
        .strip_margin();
        assert_eq!(x, "\nabcd\nefgh\n");
    }
}
