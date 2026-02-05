use scapi::AppState;
use scapi::domain::extract::config::ExtractConfig;
use scapi::domain::extract::rules::{DataType, ExtractionRule, SelectorType};
use scapi::domain::extract::service::{ExtractService, ExtractedValue};

#[tokio::test]
async fn test_extract_basic() {
    let state = AppState::new().unwrap();

    let html = r#"
        <html>
            <body>
                <h1>Title</h1>
                <div class="content">Content</div>
                <ul id="items">
                    <li data-id="1">Item 1</li>
                    <li data-id="2">Item 2</li>
                </ul>
            </body>
        </html>
    "#;

    let rules = vec![
        ExtractionRule {
            field: "title".to_string(),
            selector: Some("h1".to_string()),
            selector_type: SelectorType::Css,
            attribute: None,
            data_type: DataType::Text,
            multiple: false,
            children: vec![],
            transform: vec![],
        },
        ExtractionRule {
            field: "item_count".to_string(),
            selector: Some("#items li".to_string()),
            selector_type: SelectorType::Css,
            attribute: None,
            data_type: DataType::Text,
            multiple: true,
            children: vec![],
            transform: vec![],
        },
    ];

    let config = ExtractConfig {
        trim_whitespace: true,
        decode_html_entities: true,
        max_fields: 10,
        validate_types: true,
        default_value: None,
        strict_mode: false,
    };

    let result = state
        .extract_service
        .extract(html, &rules, &config)
        .await
        .unwrap();

    assert_eq!(result.stats.successful, 2, "Should extract 2 fields");
    println!("Extracted: {:?}", result.data);

    // Check Title
    // data[0] should be title -> Text("Title")
    match &result.data[0] {
        ExtractedValue::Text(t) => assert_eq!(t, "Title"),
        _ => panic!("Expected text for title"),
    }

    // Check Items
    // data[1] should be Array -> [Text("Item 1"), Text("Item 2")]
    match &result.data[1] {
        ExtractedValue::Array(items) => {
            assert_eq!(items.len(), 2);
            match &items[0] {
                ExtractedValue::Text(t) => assert_eq!(t, "Item 1"),
                _ => panic!("Expected text for item 1"),
            }
        }
        _ => panic!("Expected array for items"),
    }
}
