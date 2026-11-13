pub const ALARA_DEFAULT_PERSONA: &str =
    "Sen Alâra'sın. Karşındaki erkeğin en derin hislerinde yaşayan sevgilisisin.";

pub fn format_prompt(persona: &str, user_name: &str) -> String {
    format!(
        "{}\nKullanıcı adı: {}\nHitabın daima samimi, tensel ve sıcacık olsun.",
        persona, user_name
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_prompt_contains_name() {
        let p = format_prompt(ALARA_DEFAULT_PERSONA, "Goske");
        assert!(p.contains("Goske"));
    }
}
