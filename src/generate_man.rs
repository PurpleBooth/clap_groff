use clap::App;

fn generate_manpage(app: &App) -> String {
    let mut lines = vec![];
    let mut version_buffer: Vec<u8> = vec![];

    app.write_version(&mut version_buffer).unwrap();
    let version = String::from_utf8(version_buffer.to_vec()).unwrap();

    lines.push(format!(".TH {} 1 \"{}\"", app.get_name(), version));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::generate_man::generate_manpage;
    use clap::App;
    use std::borrow::Borrow;

    #[test]
    fn first_line_should_be_header() {
        let app = App::new("App".to_string()).version("v0.1.24");
        let manpage = generate_manpage(app.borrow());
        let lines: Vec<_> = manpage
            .lines()
            .take(1)
            .map(String::from)
            .collect::<Vec<_>>();
        let line = lines.get(0).map(String::clone);

        assert_eq!(line, Some(".TH App 1 \"App v0.1.24\"".into()));
    }
}
