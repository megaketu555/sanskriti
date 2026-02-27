use std::collections::HashMap;

/// Translates Sanskrit keywords in the given source contents into
/// their Lox equivalents so the rest of the pipeline can operate
/// on standard Lox syntax.
pub fn translate_file_contents(contents: &str) -> miette::Result<String> {
    let mut replacements = HashMap::new();
    replacements.insert("श्रेणी", "class");
    replacements.insert("अथ्वा", "else");
    replacements.insert("असत्य", "false");
    replacements.insert("पुरा", "for");
    replacements.insert("विनियोग", "fun");
    replacements.insert("यदि", "if");
    replacements.insert("नेति", "nil");
    replacements.insert("विकल्प", "or");
    replacements.insert("कथय", "print");
    replacements.insert("देयम", "return");
    replacements.insert("महा", "super");
    replacements.insert("यह", "this");
    replacements.insert("सत्य", "true");
    replacements.insert("चर", "var");
    replacements.insert("यावद", "while");

    let mut output = contents.to_string();
    for (from, to) in replacements {
        output = output.replace(from, to);
    }

    Ok(output)
}