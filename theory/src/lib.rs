mod bubble_sort;

#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn logger_init() {
        env_logger::init();
    }
}
