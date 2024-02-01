mod page1;

#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn init() {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "debug");
        }

        env_logger::init();
    }
}
