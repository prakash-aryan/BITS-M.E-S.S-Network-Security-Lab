#[cfg(windows)]
use winres::WindowsResource;

fn main() {
    #[cfg(windows)]
    {
        let mut res = WindowsResource::new();
        res.set_icon("app_icon.ico");
        res.compile().unwrap();
    }
}