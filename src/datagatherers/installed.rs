use crate::app::App;

pub fn gather_installed() -> Vec<App> {
    let mut installed_apps: Vec<App> = Vec::new();

    let apps = installed::list().unwrap();
    for app in apps {
        let name = app.name();
        let version = app.version();
        let publisher = app.publisher();

        if name.is_empty() {
            continue;
        }

        if version.is_empty() {
            continue;
        }

        if publisher.is_empty() {
            continue;
        }

        let app = App {
            name: name.to_string(),
            version: version.to_string(),
            author: publisher.to_string(),
        };

        installed_apps.push(app);
    }
    
    installed_apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    
    installed_apps
} 