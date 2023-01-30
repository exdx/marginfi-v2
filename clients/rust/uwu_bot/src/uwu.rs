use rand::seq::SliceRandom;

const UWU: [&str; 86] = [
    r#"(* ^ ω ^)"#,
    r#"(´ ∀ ` *)"#,
    r#"٩(◕‿◕｡)۶	"#,
    r#"☆*:.｡.o(≧▽≦)o.｡.:*☆"#,
    r#"(o^▽^o)"#,
    r#"(⌒▽⌒)☆"#,
    r#"<(￣︶￣)>"#,
    r#"。.:☆*:･'(*⌒―⌒*)))"#,
    r#"ヽ(・∀・)ﾉ"#,
    r#"(´｡• ω •｡`)"#,
    r#"(￣ω￣)"#,
    r#"(o･ω･o)"#,
    r#"(＠＾◡＾)"#,
    r#"ヽ(*・ω・)ﾉ"#,
    r#"(o_ _)ﾉ彡☆"#,
    r#"(^人^)"#,
    r#"(o´▽`o)"#,
    r#"(*´▽`*)"#,
    r#"｡ﾟ( ﾟ^∀^ﾟ)ﾟ｡"#,
    r#"( ´ ω ` )"#,
    r#"(((o(*°▽°*)o)))"#,
    r#"(≧◡≦)"#,
    r#"(o´∀`o)"#,
    r#"(´• ω •`)"#,
    r#"(＾▽＾)"#,
    r#"(⌒ω⌒)"#,
    r#"∑d(°∀°d)"#,
    r#"╰(▔∀▔)╯"#,
    r#"(─‿‿─)"#,
    r#"(*^‿^*)"#,
    r#"ヽ(o^ ^o)ﾉ"#,
    r#"(✯◡✯)"#,
    r#"(◕‿◕)"#,
    r#"(*≧ω≦*)"#,
    r#"(☆▽☆)"#,
    r#"(⌒‿⌒)"#,
    r#"＼(≧▽≦)／"#,
    r#"ヽ(o＾▽＾o)ノ"#,
    r#"☆ ～('▽^人)"#,
    r#"(*°▽°*)	"#,
    r#"٩(｡•́‿•̀｡)۶"#,
    r#"(✧ω✧)"#,
    r#"ヽ(*⌒▽⌒*)ﾉ"#,
    r#"(´｡• ᵕ •｡`)"#,
    r#"( ´ ▽ ` )"#,
    r#"(￣▽￣)"#,
    r#"╰(*´︶`*)╯"#,
    r#"ヽ(>∀<☆)ノ"#,
    r#"o(≧▽≦)o"#,
    r#"(☆ω☆)"#,
    r#"(っ˘ω˘ς )"#,
    r#"＼(￣▽￣)／"#,
    r#"(*¯︶¯*)"#,
    r#"＼(＾▽＾)／"#,
    r#"٩(◕‿◕)۶"#,
    r#"(o˘◡˘o)"#,
    r#"\(★ω★)/"#,
    r#"\(^ヮ^)/"#,
    r#"(〃＾▽＾〃)"#,
    r#"(╯✧▽✧)╯"#,
    r#"o(>ω<)o	"#,
    r#"o( ❛ᴗ❛ )o"#,
    r#"｡ﾟ(TヮT)ﾟ｡"#,
    r#"( ‾́ ◡ ‾́ )"#,
    r#"(ﾉ´ヮ`)ﾉ*: ･ﾟ"#,
    r#"(b ᵔ▽ᵔ)b"#,
    r#"(๑˃ᴗ˂)ﻭ"#,
    r#"(๑˘︶˘๑)"#,
    r#"( ˙꒳\u{200B}˙ )"#,
    r#"(*꒦ິ꒳꒦ີ)"#,
    r#"°˖✧◝(⁰▿⁰)◜✧˖°"#,
    r#"(´･ᴗ･ ` )"#,
    r#"(ﾉ◕ヮ◕)ﾉ*:･ﾟ✧"#,
    r#"(„• ֊ •„)"#,
    r#"(.❛ ᴗ ❛.)"#,
    r#"(⁀ᗢ⁀)"#,
    r#"(￢‿￢ )"#,
    r#"(¬‿¬ )"#,
    r#"(*￣▽￣)b"#,
    r#"( ˙▿˙ )"#,
    r#"(¯▿¯)"#,
    r#"( ◕▿◕ )"#,
    r#"＼(٥⁀▽⁀ )／"#,
    r#"(„• ᴗ •„)"#,
    r#"(ᵔ◡ᵔ)"#,
    r#"( ´ ▿ ` )"#,
];

pub fn get_random_emoji() -> String {
    let mut rng = rand::thread_rng();
    let emoji = UWU.choose(&mut rng).unwrap();
    emoji.to_string()
}