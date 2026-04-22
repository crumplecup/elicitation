//! Integration tests for `elicit_csv`.

#[cfg(test)]
mod tests {
    use elicit_csv::CsvPlugin;
    use elicitation::ElicitPlugin;

    #[test]
    fn plugin_creates_with_empty_context() {
        let plugin = CsvPlugin::new();
        // The plugin holds an Arc<CsvCtx>; verify the inner context starts empty.
        let ctx = &plugin.0;
        assert_eq!(ctx.mem_readers.lock().unwrap().len(), 0);
        assert_eq!(ctx.mem_writers.lock().unwrap().len(), 0);
    }

    #[test]
    fn plugin_lists_tools() {
        let plugin = CsvPlugin::new();
        let tools = plugin.list_tools();
        assert!(
            !tools.is_empty(),
            "CsvPlugin should expose at least one tool"
        );
    }
}
