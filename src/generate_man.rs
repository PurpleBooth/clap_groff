use clap::App;

fn generate_manpage(app: &App) -> String {
    let mut lines = vec![];
    let version = version::version(app).unwrap();

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

        assert_eq!(line, Some(".TH App 1 \"v0.1.24\"".into()));
    }
}

mod version {
    use clap::App;

    pub(crate) fn version(app: &App) -> Option<String> {
        let mut version_buffer: Vec<u8> = vec![];

        app.write_version(&mut version_buffer).unwrap();
        let version = String::from_utf8(version_buffer.to_vec()).unwrap();
        let prefix = format!("{} ", app.get_name());
        version.strip_prefix(&prefix).map(String::from)
    }

    #[cfg(test)]
    mod tests {
        use crate::generate_man::version::version;
        use clap::App;

        #[test]
        fn i_can_get_the_version_without_app_name() {
            let app = App::new("App".to_string()).version("v0.1.24");
            let version = version(&app);

            assert_eq!(version, Some("v0.1.24".to_string()));
        }
    }
}
