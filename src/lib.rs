pub fn abbreviate(phrase: &str) -> String {
    let no_punct = remove_punctuation_except_hyphen(phrase);
    let normalized = replace_hyphen_with_space(&no_punct);
    let raw_words = split_on_whitespace(&normalized);
    let expanded_words = expand_camel_case_list(&raw_words);
    collect_acronym_letters(&expanded_words)
}

fn remove_punctuation_except_hyphen(input: &str) -> String {
    match input.chars().next() {
        None => String::new(),
        Some(ch) => {
            if ch.is_alphanumeric() || ch.is_whitespace() || ch == '-' {
                let tail = &input[ch.len_utf8()..];
                format!("{}{}", ch, remove_punctuation_except_hyphen(tail))
            } else {
                remove_punctuation_except_hyphen(&input[ch.len_utf8()..])
            }
        }
    }
}

fn replace_hyphen_with_space(input: &str) -> String {
    match input.chars().next() {
        None => String::new(),
        Some(ch) => {
            let tail = &input[ch.len_utf8()..];
            if ch == '-' {
                format!(" {}", replace_hyphen_with_space(tail))
            } else {
                format!("{}{}", ch, replace_hyphen_with_space(tail))
            }
        }
    }
}

fn split_on_whitespace(input: &str) -> Vec<String> {
    let trimmed = trim_leading_whitespace(input);

    if trimmed.is_empty() {
        return vec![];
    }

    let (word, remainder) = take_while_not_whitespace(trimmed);
    let tail_words = split_on_whitespace(remainder);

    if word.is_empty() {
        tail_words
    } else {
        std::iter::once(word)
            .chain(tail_words.into_iter())
            .collect()
    }
}

fn trim_leading_whitespace(input: &str) -> &str {
    match input.chars().next() {
        None => input,
        Some(ch) if ch.is_whitespace() => {
            let tail = &input[ch.len_utf8()..];
            trim_leading_whitespace(tail)
        }
        Some(_) => input,
    }
}

fn take_while_not_whitespace(input: &str) -> (String, &str) {
    match input.chars().next() {
        None => (String::new(), ""),
        Some(ch) if ch.is_whitespace() => (String::new(), input),
        Some(ch) => {
            let tail = &input[ch.len_utf8()..];
            let (rest_word, rest_rem) = take_while_not_whitespace(tail);
            (format!("{}{}", ch, rest_word), rest_rem)
        }
    }
}

fn expand_camel_case_list(words: &[String]) -> Vec<String> {
    words
        .split_first()
        .map(|(first, rest)| {
            let expanded = split_camel_case(first);
            expanded
                .into_iter()
                .chain(expand_camel_case_list(rest).into_iter())
                .collect()
        })
        .unwrap_or_else(Vec::new)
}

fn is_all_uppercase_or_nonalpha(word: &str) -> bool {
    word.chars().all(|c| !c.is_alphabetic() || c.is_uppercase())
}

fn split_camel_case(word: &str) -> Vec<String> {
    if is_all_uppercase_or_nonalpha(word) {
        vec![word.to_string()]
    } else {
        split_camel_case_recursive(word, String::new())
    }
}

fn split_camel_case_recursive(input: &str, current: String) -> Vec<String> {
    match input.chars().next() {
        None => {
            if current.is_empty() {
                vec![]
            } else {
                vec![current]
            }
        }
        Some(ch) => {
            let tail = &input[ch.len_utf8()..];

            if ch.is_uppercase() && !current.is_empty() {
                vec![current]
                    .into_iter()
                    .chain(split_camel_case_recursive(tail, ch.to_string()))
                    .collect()
            } else {
                split_camel_case_recursive(tail, format!("{}{}", current, ch))
            }
        }
    }
}

fn collect_acronym_letters(words: &[String]) -> String {
    match words.split_first() {
        None => String::new(),
        Some((first, rest)) => {
            let first_char = match first.chars().next() {
                None => '\0',
                Some(ch) => ch.to_ascii_uppercase(),
            };

            let tail_acronym = collect_acronym_letters(rest);
            format!("{}{}", first_char, tail_acronym)
        }
    }
}
