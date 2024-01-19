mod bfs;
mod binary_search;
mod bubble_sort;
mod dfs;
mod dijkstra;
mod dynamic_programing;
mod greedy;
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
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "debug");
        }

        env_logger::init();
    }
}
