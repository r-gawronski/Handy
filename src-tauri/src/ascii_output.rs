//! Optional ASCII folding of the text Handy pastes.
//!
//! Some targets (game chats such as World of Warcraft, old terminals, legacy
//! apps) ship fonts without glyphs for Polish letters or typographic
//! punctuation, so `zażółć` shows up as `za□□□ć`. When the
//! `ascii_output` setting is on, the pasted text has accented Latin letters
//! replaced with their base letter (`ą → a`, `ł → l`, `ß → ss`) and
//! typographic punctuation replaced with plain ASCII (`„…” → "..."`).
//!
//! Only the pasted text is folded; history keeps the original transcription.
//! Characters with no sensible Latin fallback (Cyrillic, CJK, emoji, …) are
//! left untouched rather than dropped, so nothing is silently lost.

/// ASCII replacement for a single character, or `None` to keep it as is.
fn fold_char(c: char) -> Option<&'static str> {
    let s = match c {
        // Polish
        'ą' => "a",
        'Ą' => "A",
        'ć' => "c",
        'Ć' => "C",
        'ę' => "e",
        'Ę' => "E",
        'ł' => "l",
        'Ł' => "L",
        'ń' => "n",
        'Ń' => "N",
        'ó' => "o",
        'Ó' => "O",
        'ś' => "s",
        'Ś' => "S",
        'ź' | 'ż' => "z",
        'Ź' | 'Ż' => "Z",

        // Other common Latin letters (Latin-1 Supplement / Latin Extended-A)
        'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' => "a",
        'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' | 'Ā' | 'Ă' => "A",
        'æ' => "ae",
        'Æ' => "AE",
        'ç' | 'č' | 'ĉ' | 'ċ' => "c",
        'Ç' | 'Č' | 'Ĉ' | 'Ċ' => "C",
        'ď' | 'đ' | 'ð' => "d",
        'Ď' | 'Đ' | 'Ð' => "D",
        'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ė' | 'ě' => "e",
        'È' | 'É' | 'Ê' | 'Ë' | 'Ē' | 'Ė' | 'Ě' => "E",
        'ğ' | 'ģ' => "g",
        'Ğ' | 'Ģ' => "G",
        'ì' | 'í' | 'î' | 'ï' | 'ī' | 'į' | 'ı' => "i",
        'Ì' | 'Í' | 'Î' | 'Ï' | 'Ī' | 'Į' | 'İ' => "I",
        'ķ' => "k",
        'Ķ' => "K",
        'ĺ' | 'ľ' | 'ļ' => "l",
        'Ĺ' | 'Ľ' | 'Ļ' => "L",
        'ñ' | 'ň' | 'ņ' => "n",
        'Ñ' | 'Ň' | 'Ņ' => "N",
        'ò' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ő' => "o",
        'Ò' | 'Ô' | 'Õ' | 'Ö' | 'Ø' | 'Ō' | 'Ő' => "O",
        'œ' => "oe",
        'Œ' => "OE",
        'ŕ' | 'ř' => "r",
        'Ŕ' | 'Ř' => "R",
        'š' | 'ş' | 'ș' => "s",
        'Š' | 'Ş' | 'Ș' => "S",
        'ß' => "ss",
        'ť' | 'ţ' | 'ț' => "t",
        'Ť' | 'Ţ' | 'Ț' => "T",
        'þ' => "th",
        'Þ' => "Th",
        'ù' | 'ú' | 'û' | 'ü' | 'ū' | 'ů' | 'ű' | 'ų' => "u",
        'Ù' | 'Ú' | 'Û' | 'Ü' | 'Ū' | 'Ů' | 'Ű' | 'Ų' => "U",
        'ý' | 'ÿ' => "y",
        'Ý' | 'Ÿ' => "Y",
        'ž' => "z",
        'Ž' => "Z",

        // Typographic punctuation and spaces
        '„' | '“' | '”' | '‟' | '«' | '»' | '″' => "\"",
        '‚' | '‘' | '’' | '‛' | '‹' | '›' | '′' => "'",
        '–' | '—' | '―' | '‐' | '‑' | '−' => "-",
        '…' => "...",
        '\u{00A0}' | '\u{2007}' | '\u{202F}' | '\u{2009}' | '\u{200A}' => " ",
        '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' => "",
        _ => return None,
    };
    Some(s)
}

/// Replaces accented Latin letters and typographic punctuation with plain
/// ASCII equivalents. Characters with no mapping are kept unchanged.
pub fn fold_to_ascii(text: &str) -> String {
    if text.is_ascii() {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match fold_char(c) {
            Some(replacement) => out.push_str(replacement),
            None => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::fold_to_ascii;

    #[test]
    fn folds_all_polish_letters() {
        assert_eq!(fold_to_ascii("ąćęłńóśźż ĄĆĘŁŃÓŚŹŻ"), "acelnoszz ACELNOSZZ");
    }

    #[test]
    fn folds_polish_sentence() {
        assert_eq!(
            fold_to_ascii("Zażółć gęślą jaźń. Idę na raida, będę za 5 minut!"),
            "Zazolc gesla jazn. Ide na raida, bede za 5 minut!"
        );
    }

    #[test]
    fn folds_typographic_punctuation() {
        assert_eq!(
            fold_to_ascii("Powiedział „hej” – i ‚poszedł’…"),
            "Powiedzial \"hej\" - i 'poszedl'..."
        );
    }

    #[test]
    fn folds_other_european_letters() {
        assert_eq!(
            fold_to_ascii("Straße, café, Škoda, Ærø"),
            "Strasse, cafe, Skoda, AEro"
        );
    }

    #[test]
    fn keeps_ascii_and_unmapped_text() {
        assert_eq!(fold_to_ascii("plain text 123"), "plain text 123");
        assert_eq!(fold_to_ascii("Привет 你好"), "Привет 你好");
    }

    #[test]
    fn normalizes_special_spaces() {
        assert_eq!(fold_to_ascii("10\u{00A0}zł\u{200B}"), "10 zl");
    }

    #[test]
    fn output_is_ascii_for_polish_text() {
        let folded = fold_to_ascii("Źdźbło, żółw, łódź — „Ćma” i węże…");
        assert!(folded.is_ascii(), "{folded}");
    }
}
