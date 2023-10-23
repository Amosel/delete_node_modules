use crate::app::App;
use crate::item::Item;
use crate::list::StatefulList;

pub fn dummy_app() -> App {
    App {
        running: true,
        list: StatefulList::with_items(vec![
            Item {
                path: std::path::PathBuf::from(
                    "./mobile/WIP/expo-rn-module/Xholonym-module/example/node_modules",
                ),
                size_mb: 374f64,
                is_on: false,
            },
            Item {
                path: std::path::PathBuf::from(
                    "./mobile/WIP/expo-rn-module/Xholonym-module/node_modules",
                ),
                size_mb: 438f64,
                is_on: false,
            },
            Item {
                path: std::path::PathBuf::from("./mobile/tmp/module-test/node_modules"),
                size_mb: 304f64,
                is_on: false,
            },
            Item {
                path: std::path::PathBuf::from("./mobile/tmp/expo-keypad/node_modules"),
                size_mb: 424f64,
                is_on: false,
            },
            Item {
                path: std::path::PathBuf::from("./node/zk-escrow/node_modules"),
                size_mb: 238f64,
                is_on: false,
            },
            Item {
                path: std::path::PathBuf::from("./node/simple-escrow-client/node_modules"),
                size_mb: 234f64,
                is_on: false,
            },
            Item {
                path: std::path::PathBuf::from("test"),
                size_mb: 0.0,
                is_on: false,
            },
            Item {
                path: std::path::PathBuf::from("test 2"),
                size_mb: 20.0,
                is_on: false,
            },
        ]),
        loading: true,
    }
}
