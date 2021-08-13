extern crate r_i18n;
use r_i18n::I18nConfig;
use r_i18n::I18n;

fn main() {
    let config: I18nConfig = I18nConfig{locales: &["en", "es"], directory: "translations"};
    let r_i18n: I18n = I18n::configure(&config);

    unsafe {
        let text = r_i18n.t("msg");

        println!("{}", text);
    }
}
