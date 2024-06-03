#[cfg(test)]
mod tests {
    use crate::App;

    #[test]
    fn render() {
        let app = App::default();
        assert_eq!(2 + 2, 4);
    }
}
