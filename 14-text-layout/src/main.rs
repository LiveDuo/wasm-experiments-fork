fn main() {
    let text = "हिन्दी विकिपीडिया
पर आपका स्वागत है
एक मुक्त ज्ञानकोश, जिसे कोई भी संपादित कर सकता है।
हिन्दी में 1,59,865 लेख हैं
विकिपीडिया सभी विषयों पर प्रामाणिक और उपयोग, परिवर्तन व पुनर्वितरण के लिए स्वतन्त्र ज्ञानकोश बनाने का एक बहुभाषीय प्रकल्प है। यह यथासम्भव निष्पक्ष दृष्टिकोण वाली सूचना प्रसारित करने के लिए कृतसंकल्प है। सर्वप्रथम अंग्रेज़ी विकिपीडिया जनवरी 2001 में आरम्भ किया गया था, और हिन्दी विकिपीडिया का शुभारम्भ जुलाई 2003 में हुआ। सहायता पृष्ठ पर जाएं और प्रयोगस्थल में प्रयोग करके देखें कि आप स्वयं किसी लेख को कैसे परिवर्तित कर सकते हैं।";
    // let text = "संपादित";
    let words = azul_text_layout::text_layout::split_text_into_words(text);
    dbg!(&words);

    let font = include_bytes!("tiro.ttf");
    let parsed_font = azul_text_layout::text_layout::parse_font(font, 0, false).unwrap();
    show_font(&parsed_font);
    let scaled_words = azul_text_layout::text_layout::shape_words(&words, &parsed_font);
    dbg!(&scaled_words);

    let total_width: usize = scaled_words.items.iter().map(|i| i.word_width).sum();
    println!(
        "Total width of the text: {}em",
        (total_width as f32) / (scaled_words.font_metrics_units_per_em as f32)
    )
}

fn show_font(font: &azul_text_layout::text_shaping::ParsedFont) {
    dbg!(&font.font_metrics);
    dbg!(&font.num_glyphs);
    // dbg!(&font.hhea_table);
    // dbg!(&font.hmtx_data);
    // dbg!(&font.maxp_table);
    // dbg!(&font.gsub_cache);
    // dbg!(&font.gpos_cache);
    // dbg!(&font.opt_gdef_table);
    // dbg!(&font.glyph_records_decoded);
    dbg!(&font.space_width);
    // dbg!(&font.cmap_subtable);
}