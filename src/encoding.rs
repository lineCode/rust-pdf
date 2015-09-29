use std::collections::BTreeMap;


#[derive(Debug)]
pub struct Encoding {
    name_to_code: BTreeMap<&'static str, u8>
}

impl Encoding {
    /// Get the encoded code point from a type1 name
    pub fn get_code(&self, name: &str) -> Option<u8> {
        match self.name_to_code.get(name) {
            Some(&code) => Some(code),
            None => None
        }
    }
    fn init_block(&mut self, start: u8, data: Vec<&'static str>) {
        let mut i = start - 1;
        for name in data {
            i += 1;
            self.name_to_code.insert(name, i);
        }
    }
}

lazy_static! {
    pub static ref WIN_ANSI_ENCODING: Encoding = {
        let mut result = Encoding { name_to_code: BTreeMap::new() };
        result.init_block(0o40, vec!(
            "space", "exclam", "quotedbl", "numbersign",
            "dollar", "percent", "ampersand", "quotesingle"));
        result.init_block(0o50, vec!(
            "parenleft", "parenright", "asterisk", "plus",
            "comma", "hyphen", "period", "slash"));
        result.init_block(0o60, vec!(
            "zero", "one", "two", "three", "four", "five", "six", "seven"));
        result.init_block(0o70, vec!(
            "eight", "nine", "colon", "semicolon",
            "less", "equal", "greater", "question"));
        result.init_block(0o100, vec!(
            "at", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
            "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V",
            "W", "X", "Y", "Z"));
        result.init_block(0o133, vec!(
            "bracketleft",
            "backslash", "bracketright", "asciicircum", "underscore"));
        result.init_block(0o140, vec!(
            "grave", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
            "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v",
            "w", "x", "y", "z"));
        result.init_block(0o173, vec!(
            "braceleft", "bar", "braceright", "asciitilde"));
        result.init_block(0o200, vec!(
            "Euro", "..1", "quotesinglbase", "florin",
            "quotedblbase", "ellipsis", "dagger", "daggerdbl"));
        result.init_block(0o210, vec!(
            "circumflex", "perthousand", "Scaron", "guilsinglleft",
            "OE", "..5", "Zcaron", "..7"));
        result.init_block(0o220, vec!(
            "..0", "quoteleft", "quoteright", "quotedblleft",
            "quotedblright", "bullet", "endash", "emdash"));
        result.init_block(0o230, vec!(
            "tilde", "trademark", "scaron", "guilsinglright",
            "oe", "..5", "zcaron", "Ydieresis"));
        result.init_block(0o240, vec!(
            "..0", "exclamdown", "cent", "sterling",
            "currency", "yen", "brokenbar", "section"));
        result.init_block(0o250, vec!(
            "dieresis", "copyright", "ordfeminine", "guillemotleft",
            "logicalnot", "..5", "registered", "macron"));
        result.init_block(0o260, vec!(
            "degree", "plusminus", "twosuperior", "threesuperior",
            "acute", "mu", "paragraph", "periodcentered"));
        result.init_block(0o270, vec!(
            "cedilla", "onesuperior", "ordmasculine", "guillemotright",
            "onequarter", "onehalf", "threequarters", "questiondown"));
        result.init_block(0o300, vec!(
            "Agrave", "Aacute", "Acircumflex", "Atilde",
            "Adieresis", "Aring", "AE", "Ccedilla"));
        result.init_block(0o310, vec!(
            "Egrave", "Eacute", "Ecircumflex", "Edieresis",
            "Igrave", "Iacute", "Icircumflex", "Idieresis"));
        result.init_block(0o320, vec!(
            "Eth", "Ntilde", "Ograve", "Oacute",
            "Ocircumflex", "Otilde", "Odieresis", "multiply"));
        result.init_block(0o330, vec!(
            "Oslash", "Ugrave", "Uacute", "Ucircumflex",
            "Udieresis", "Yacute", "Thorn", "germandbls"));
        result.init_block(0o340, vec!(
            "agrave", "aacute", "acircumflex", "atilde",
            "adieresis", "aring", "ae", "ccedilla"));
        result.init_block(0o350, vec!(
            "egrave", "eacute", "ecircumflex", "edieresis",
            "igrave", "iacute", "icircumflex", "idieresis"));
        result.init_block(0o360, vec!(
            "eth", "ntilde", "ograve", "oacute",
            "ocircumflex", "otilde", "odieresis", "divide"));
        result.init_block(0o370, vec!(
            "oslash", "ugrave", "uacute", "ucircumflex",
            "udieresis", "yacute", "thorn", "ydieresis"));
        result
    };
}

#[test]
fn test_get_winansi_points() {
    let ref enc = WIN_ANSI_ENCODING;
    assert_eq!(Some('A' as u8), enc.get_code("A"));
    assert_eq!(Some('Z' as u8), enc.get_code("Z"));
    assert_eq!(Some('a' as u8), enc.get_code("a"));
    assert_eq!(Some('z' as u8), enc.get_code("z"));
    assert_eq!(Some(' ' as u8), enc.get_code("space"));
    assert_eq!(Some('&' as u8), enc.get_code("ampersand"));
}
