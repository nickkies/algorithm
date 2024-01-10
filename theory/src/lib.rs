mod binary_search;
mod bubble_sort;
mod dfs;
mod dynamic_programing;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod recursive;
mod selection_sort;
mod sequential_search;

#[cfg(test)]
mod tests {
    #[ctor::ctor]
    fn logger_init() {
        env_logger::init();
    }
}
