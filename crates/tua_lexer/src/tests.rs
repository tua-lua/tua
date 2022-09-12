use super::*;

use expect_test::{expect, Expect};

#[test]
fn valid_hashbang() {
    let input = "#!/usr/bin/env tua\nlocal x = 0;";
    assert_eq!(strip_hashbang(input), Some(18));
}

#[test]
fn hashbang_second_line() {
    let input = "\n#!/usr/bin/env tua";
    assert_eq!(strip_hashbang(input), None);
}

fn check_lexing(src: &str, expect: Expect) {
    let actual: String = tokenize(src)
        .map(|token| format!("{:?}\n", token))
        .collect();
    expect.assert_eq(&actual)
}

#[test]
fn smoke_test() {
    check_lexing(
        r#"
--[[ comment ]]
function fn() {
  print("");
}
#a[b]['c']:d(...)
::label::
local x, y = 0 + 1 - 2 * 3 / 4 ^ 5 % 6, 0
if x ~= y or y <= z or z > x then
  goto ::label::
end
"#,
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: LongComment { terminated: true }, len: 15 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 8 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 2 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 3 }
            Token { kind: Ident, len: 5 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Literal { kind: ShortString { quote: '"', terminated: true } }, len: 2 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Hash, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: OpenBracket, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: CloseBracket, len: 1 }
            Token { kind: OpenBracket, len: 1 }
            Token { kind: Literal { kind: ShortString { quote: '\'', terminated: true } }, len: 3 }
            Token { kind: CloseBracket, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Dot, len: 1 }
            Token { kind: Dot, len: 1 }
            Token { kind: Dot, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Ident, len: 5 }
            Token { kind: Colon, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 5 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: true } }, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Plus, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Minus, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Star, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Slash, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Caret, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Percent, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 1 }
            Token { kind: Comma, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: true } }, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Tilde, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Lt, len: 1 }
            Token { kind: Eq, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Gt, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: Whitespace, len: 3 }
            Token { kind: Ident, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Ident, len: 5 }
            Token { kind: Colon, len: 1 }
            Token { kind: Colon, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}

#[test]
fn short_comment() {
    check_lexing(
        r"
--
-- short
---comment
---[[ still short ]]
",
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: ShortComment, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: ShortComment, len: 8 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: ShortComment, len: 10 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: ShortComment, len: 20 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}

#[test]
fn long_comment() {
    check_lexing(
        r"
--[[]]
--[=[
  long
  [[ comment ]]
--]=]
",
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: LongComment { terminated: true }, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: LongComment { terminated: true }, len: 34 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}

#[test]
fn unterminated_long_comment() {
    check_lexing(
        r"
--[=[ ]]
",
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: LongComment { terminated: false }, len: 9 }
        "#]],
    )
}

#[test]
fn short_string() {
    check_lexing(
        r#"
'short'
"string"
"#,
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: ShortString { quote: '\'', terminated: true } }, len: 7 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: ShortString { quote: '"', terminated: true } }, len: 8 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}

#[test]
fn unterminated_short_string() {
    check_lexing(
        r#"
'short"
"string'
"#,
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: ShortString { quote: '\'', terminated: false } }, len: 7 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: ShortString { quote: '"', terminated: false } }, len: 8 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}

#[test]
fn long_string() {
    check_lexing(
        r#"
[==[
  long
  [[ string ]]
]==]
"#,
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: LongString { level: 2, terminated: true } }, len: 31 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}

#[test]
fn unterminated_long_string() {
    check_lexing(
        r#"
[==[
  long
  [[ string ]]
]=]
"#,
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: LongString { level: 2, terminated: false } }, len: 31 }
        "#]],
    )
}

#[test]
fn decimal_number() {
    check_lexing(
        r#"
3
3.0
3.1416
314.16e-2
314.16e
0.31416E1
"#,
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 9 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: true } }, len: 7 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Decimal, empty_number: false, empty_exponent: false } }, len: 9 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}

#[test]
fn hexadecimal_number() {
    check_lexing(
        r#"
0x
0xff
0x0.1E
0xA23p-4
0xA23p-
0X1.921FB54442D18P+1
"#,
        expect![[r#"
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Hexadecimal, empty_number: true, empty_exponent: false } }, len: 2 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Hexadecimal, empty_number: false, empty_exponent: false } }, len: 4 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Hexadecimal, empty_number: false, empty_exponent: false } }, len: 6 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Hexadecimal, empty_number: false, empty_exponent: false } }, len: 8 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Hexadecimal, empty_number: false, empty_exponent: true } }, len: 7 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Number { base: Hexadecimal, empty_number: false, empty_exponent: false } }, len: 20 }
            Token { kind: Whitespace, len: 1 }
        "#]],
    )
}
