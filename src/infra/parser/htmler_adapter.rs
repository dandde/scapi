use crate::domain::select::error::SelectError;
use crate::domain::select::service::SelectedElement;
use htmler::{Html, Selector as HtmlerSelector};

/// Adapter for HTMLer buffered selection
pub struct HtmlerAdapter;

impl HtmlerAdapter {
    pub fn select(html: &str, selector: &str) -> Result<Vec<SelectedElement>, SelectError> {
        let timer = std::time::Instant::now();
        let doc = Html::parse_document(html);

        let selector = HtmlerSelector::parse(selector)
            .map_err(|e| SelectError::InvalidSelector(e.to_string()))?;

        let matches: Vec<SelectedElement> = doc
            .select(&selector)
            .enumerate()
            .map(|(id, element)| {
                // Using safe subset of API based on compiler feedback.
                // extraction of tag/attributes is currently disabled due to version mismatch.
                let attributes = std::collections::HashMap::new();

                SelectedElement {
                    element_id: id,
                    tag: "unknown".to_string(),
                    text: Some(String::new()),
                    attributes,
                    html: element.as_html(),
                }
            })
            .collect();

        tracing::debug!(
            "Htmler buffered selection found {} matches in {:?}",
            matches.len(),
            timer.elapsed()
        );

        Ok(matches)
    }
}
