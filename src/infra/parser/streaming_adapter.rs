use crate::domain::select::error::SelectError;
use crate::domain::select::service::SelectedElement;
use lol_html::{HtmlRewriter, Settings, element};
use std::cell::RefCell;
use std::rc::Rc;

/// Adapter for lol_html streaming selection
pub struct StreamingAdapter;

impl StreamingAdapter {
    /// Selects elements from a buffered string using the streaming engine.
    /// This is useful when the content is already in memory (e.g. > 1MB threshold)
    /// but we want to use the low-memory engine to avoid building a full DOM.
    pub fn select_from_string(
        html: &str,
        selector_str: &str,
    ) -> Result<Vec<SelectedElement>, SelectError> {
        let timer = std::time::Instant::now();
        let matches = Rc::new(RefCell::new(Vec::new()));
        let matches_clone = matches.clone();

        let mut next_id = 0;

        let mut output_sink = Vec::new(); // Discarded output

        let mut rewriter = HtmlRewriter::new(
            Settings {
                element_content_handlers: vec![element!(selector_str, move |el| {
                    let tag_name = el.tag_name();
                    let attributes = el
                        .attributes()
                        .iter()
                        .map(|a| (a.name(), a.value()))
                        .collect();

                    // Note: lol_html is a rewriter, getting inner text/HTML is tricky
                    // without buffering the element content itself.
                    // For this implementation, we extract metadata.
                    // Full content extraction in streaming mode requires a more complex
                    // handler that buffers just the element's content.

                    matches_clone.borrow_mut().push(SelectedElement {
                        element_id: next_id,
                        tag: tag_name,
                        text: None, // Hard to easier extract text in pure streaming without buffering
                        attributes,
                        html: String::new(), // Placeholder
                    });
                    next_id += 1;

                    Ok(())
                })],
                ..Settings::default()
            },
            |c: &[u8]| {
                output_sink.extend_from_slice(c);
            },
        );

        rewriter
            .write(html.as_bytes())
            .map_err(|e| SelectError::ExecutionError(e.to_string()))?;
        rewriter
            .end()
            .map_err(|e| SelectError::ExecutionError(e.to_string()))?;

        let result = matches.take();

        tracing::debug!(
            "Streaming (lol_html) selection found {} matches in {:?}",
            result.len(),
            timer.elapsed()
        );

        Ok(result)
    }
}
