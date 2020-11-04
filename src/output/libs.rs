use crate::cli::{libs::Libs, APP_NAME, APP_VERSION, DEPENDENCIES};

pub fn libs(_: Libs) {
    println!("{} v{}\n{}", APP_NAME, APP_VERSION, DEPENDENCIES);
}
