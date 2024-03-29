// auto-generated: "lalrpop 0.17.2"
// sha256: 9e318d1e3998f57383c147cb2f0611681495084787c9658ad84a469b3f6726
use crate::sh::sh_ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Command {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::sh::sh_ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(CommandPtr),
        Variant2(WordPtr),
        Variant3(ParametersPtr),
        Variant4(RedirectPtr),
        Variant5(WordsPtr),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, 0, -12, 21, 22, 0,
        // State 1
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, -17, 0, -17, -17, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0,
        // State 3
        -36, 0, -36, -36, -36, -36, -36, -36, 0, 0, -36, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36,
        // State 4
        -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, -31, 0, -31, -31, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, -8, -8, 0, 0, 0,
        // State 6
        -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0, -39, 0, -39, -39, 0, 0, -39,
        // State 7
        -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, -5, 0, -5, -5, 0, 0, 0,
        // State 8
        -37, 0, 11, 12, 13, 14, 15, 16, 0, 0, -37, 17, 18, 19, 20, -37, 0, -37, -37, 21, 22, -37,
        // State 9
        -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, -13, 0, -13, -13, 0, 0, 28,
        // State 10
        -35, 0, -35, -35, -35, -35, -35, -35, 0, 0, -35, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35,
        // State 11
        -33, 0, -33, -33, -33, -33, -33, -33, 0, 0, -33, -33, -33, -33, -33, -33, 0, -33, -33, -33, -33, -33,
        // State 12
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 30,
        // State 13
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 32, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0,
        // State 15
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 36,
        // State 16
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 38,
        // State 17
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 40,
        // State 18
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 42,
        // State 19
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 44,
        // State 20
        -34, 0, -34, -34, -34, -34, -34, -34, 0, 0, -34, -34, -34, -34, -34, -34, 0, -34, -34, -34, -34, -34,
        // State 21
        -32, 0, -32, -32, -32, -32, -32, -32, 0, 0, -32, -32, -32, -32, -32, -32, 0, -32, -32, -32, -32, -32,
        // State 22
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 46,
        // State 23
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 48,
        // State 24
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 50,
        // State 25
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 52,
        // State 26
        -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0, -38, 0, -38, -38, 0, 0, -38,
        // State 27
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 0,
        // State 28
        -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0, -22, 0, -22, -22, 0, 0, -22,
        // State 29
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 0,
        // State 30
        -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, -26, 0, -26, -26, 0, 0, -26,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0,
        // State 32
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 0,
        // State 33
        -9, 0, -9, -9, -9, -9, -9, -9, 0, 0, -9, -9, -9, -9, -9, -9, 0, -9, -9, -9, -9, -9,
        // State 34
        -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0, -18, 0, -18, -18, 0, 0, -18,
        // State 35
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 0,
        // State 36
        -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, -29, 0, -29, -29, 0, 0, -29,
        // State 37
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 0,
        // State 38
        -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, -27, 0, -27, -27, 0, 0, -27,
        // State 39
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 0,
        // State 40
        -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, -20, 0, -20, -20, 0, 0, -20,
        // State 41
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 0,
        // State 42
        -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, 0, -24, 0, -24, -24, 0, 0, -24,
        // State 43
        0, 0, 11, 12, 13, 14, 15, 16, 0, 0, 0, 17, 18, 19, 20, 0, 0, 0, 0, 21, 22, 0,
        // State 44
        -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, -1, 0, -1, -1, 0, 0, 0,
        // State 45
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 0,
        // State 46
        -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0, -3, 0, -3, -3, 0, 0, 0,
        // State 47
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, -6, -6, 0, 0, 0,
        // State 49
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 0,
        // State 50
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, -15, 0, -15, -15, 0, 0, 0,
        // State 51
        -12, 0, 11, 12, 13, 14, 15, 16, 0, 0, -12, 17, 18, 19, 20, -12, 0, -12, -12, 21, 22, 0,
        // State 52
        -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, -14, 0, -14, -14, 0, 0, 0,
        // State 53
        -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0, -23, 0, -23, -23, 0, 0, -23,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 67, 25, 0, 0, 0,
        // State 56
        -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, -19, 0, -19, -19, 0, 0, -19,
        // State 57
        -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, -30, 0, -30, -30, 0, 0, -30,
        // State 58
        -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, -28, 0, -28, -28, 0, 0, -28,
        // State 59
        -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0, -21, 0, -21, -21, 0, 0, -21,
        // State 60
        -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, -25, 0, -25, -25, 0, 0, -25,
        // State 61
        -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0, 0, -2, 0, -2, -2, 0, 0, 0,
        // State 62
        -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, -4, 0, -4, -4, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, -7, -7, 0, 0, 0,
        // State 64
        23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, -16, 0, -16, -16, 0, 0, 0,
        // State 65
        -10, 0, -10, -10, -10, -10, -10, -10, 0, 0, -10, -10, -10, -10, -10, -10, 0, -10, -10, -10, -10, -10,
        // State 66
        -11, 0, -11, -11, -11, -11, -11, -11, 0, 0, -11, -11, -11, -11, -11, -11, 0, -11, -11, -11, -11, -11,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -12,
        // State 1
        -17,
        // State 2
        -40,
        // State 3
        -36,
        // State 4
        -31,
        // State 5
        -8,
        // State 6
        -39,
        // State 7
        -5,
        // State 8
        -37,
        // State 9
        -13,
        // State 10
        -35,
        // State 11
        -33,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -34,
        // State 21
        -32,
        // State 22
        -12,
        // State 23
        -12,
        // State 24
        -12,
        // State 25
        -12,
        // State 26
        -38,
        // State 27
        -12,
        // State 28
        -22,
        // State 29
        0,
        // State 30
        -26,
        // State 31
        0,
        // State 32
        0,
        // State 33
        -9,
        // State 34
        -18,
        // State 35
        0,
        // State 36
        -29,
        // State 37
        0,
        // State 38
        -27,
        // State 39
        0,
        // State 40
        -20,
        // State 41
        0,
        // State 42
        -24,
        // State 43
        0,
        // State 44
        -1,
        // State 45
        -12,
        // State 46
        -3,
        // State 47
        -12,
        // State 48
        -6,
        // State 49
        -12,
        // State 50
        -15,
        // State 51
        -12,
        // State 52
        -14,
        // State 53
        -23,
        // State 54
        0,
        // State 55
        0,
        // State 56
        -19,
        // State 57
        -30,
        // State 58
        -28,
        // State 59
        -21,
        // State 60
        -25,
        // State 61
        -2,
        // State 62
        -4,
        // State 63
        -7,
        // State 64
        -16,
        // State 65
        -10,
        // State 66
        -11,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        2, 3, 4, 5, 6, 7, 8, 9, 10, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 4, 0, 0, 7, 0, 9, 27, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 4, 0, 0, 7, 0, 9, 29, 0,
        // State 13
        0, 0, 4, 0, 0, 7, 0, 9, 31, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 4, 0, 0, 7, 0, 9, 35, 0,
        // State 16
        0, 0, 4, 0, 0, 7, 0, 9, 37, 0,
        // State 17
        0, 0, 4, 0, 0, 7, 0, 9, 39, 0,
        // State 18
        0, 0, 4, 0, 0, 7, 0, 9, 41, 0,
        // State 19
        0, 0, 4, 0, 0, 7, 0, 9, 43, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 4, 5, 0, 7, 45, 9, 10, 0,
        // State 23
        0, 0, 4, 5, 0, 7, 47, 9, 10, 0,
        // State 24
        2, 0, 4, 5, 49, 7, 8, 9, 10, 0,
        // State 25
        51, 0, 4, 5, 0, 7, 8, 9, 10, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 4, 53, 0, 7, 0, 9, 10, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 4, 0, 0, 7, 0, 9, 54, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        2, 56, 4, 5, 6, 7, 8, 9, 10, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 4, 0, 0, 7, 0, 9, 57, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 4, 0, 0, 7, 0, 9, 58, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 4, 0, 0, 7, 0, 9, 59, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 4, 0, 0, 7, 0, 9, 60, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 4, 0, 0, 7, 0, 9, 61, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 4, 5, 0, 7, 62, 9, 10, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 4, 5, 0, 7, 63, 9, 10, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        2, 0, 4, 5, 64, 7, 8, 9, 10, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        65, 0, 4, 5, 0, 7, 8, 9, 10, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"AND"###,
            r###"BACKGROUND"###,
            r###"DOUBLE_QUOTES_WORD"###,
            r###"EQUAL"###,
            r###"ERROR_REDIRECT"###,
            r###"ERROR_REDIRECT_APPEND"###,
            r###"EXPAND"###,
            r###"INPUT_REDIRECT"###,
            r###"LB"###,
            r###"LP"###,
            r###"OR"###,
            r###"OUTPUT_ERROR_REDIRECT"###,
            r###"OUTPUT_ERROR_REDIRECT_APPEND"###,
            r###"OUTPUT_REDIRECT"###,
            r###"OUTPUT_REDIRECT_APPEND"###,
            r###"PIPE"###,
            r###"RB"###,
            r###"RP"###,
            r###"SEQUENCE"###,
            r###"SINGLE_QUOTES_WORD"###,
            r###"WORD"###,
            r###"WS"###,
        ];
        __ACTION[(__state * 22)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = CommandPtr;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 22 + integer]
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __ACTION[(state as usize) * 22 + (22 - 1)]
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __GOTO[(state as usize) * 10 + nt] - 1
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state as usize)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            __simulate_reduce(action, ::std::marker::PhantomData::<(&())>)
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(6, _) if true => Some(0),
            Token(5, _) if true => Some(1),
            Token(1, _) if true => Some(2),
            Token(15, _) if true => Some(3),
            Token(11, _) if true => Some(4),
            Token(12, _) if true => Some(5),
            Token(4, _) if true => Some(6),
            Token(14, _) if true => Some(7),
            Token(18, _) if true => Some(8),
            Token(9, _) if true => Some(9),
            Token(20, _) if true => Some(10),
            Token(7, _) if true => Some(11),
            Token(8, _) if true => Some(12),
            Token(16, _) if true => Some(13),
            Token(17, _) if true => Some(14),
            Token(19, _) if true => Some(15),
            Token(21, _) if true => Some(16),
            Token(10, _) if true => Some(17),
            Token(13, _) if true => Some(18),
            Token(2, _) if true => Some(19),
            Token(3, _) if true => Some(20),
            Token(0, _) if true => Some(21),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 => match __token {
                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            1 => match __token {
                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            2 => match __token {
                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            3 => match __token {
                Token(15, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            4 => match __token {
                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            5 => match __token {
                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            6 => match __token {
                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            7 => match __token {
                Token(14, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            8 => match __token {
                Token(18, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            9 => match __token {
                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            10 => match __token {
                Token(20, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            11 => match __token {
                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            12 => match __token {
                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            13 => match __token {
                Token(16, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            14 => match __token {
                Token(17, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            15 => match __token {
                Token(19, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            16 => match __token {
                Token(21, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            17 => match __token {
                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            18 => match __token {
                Token(13, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            19 => match __token {
                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            20 => match __token {
                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            21 => match __token {
                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    fn __simulate_reduce<
        'input,
    >(
        __reduce_index: i8,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> __state_machine::SimulatedReduce<__StateMachine<'input>>
    {
        match __reduce_index {
            0 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 0,
                }
            }
            1 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 0,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 0,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 0,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 1,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 1,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 2,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 2,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 3,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 4,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 4,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            20 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            21 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            22 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            23 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            24 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            25 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            26 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            27 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            28 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 5,
                }
            }
            29 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            30 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            31 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            32 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            33 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            34 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            35 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            36 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            37 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 8,
                }
            }
            38 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 8,
                }
            }
            39 => __state_machine::SimulatedReduce::Accept,
            _ => panic!("invalid reduction index {}", __reduce_index)
        }
    }
    pub struct CommandParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl CommandParser {
        pub fn new() -> CommandParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            CommandParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<CommandPtr, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            let __r = __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: ::std::marker::PhantomData::<(&())>,
                },
                __tokens,
            );
            __r
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<CommandPtr,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            39 => {
                // __Command = Command => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 10 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, CommandPtr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ParametersPtr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, RedirectPtr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WordPtr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WordsPtr, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AndOrCommand = AndOrCommand, AND, SimpleCommand => ActionFn(7);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AndOrCommand = AndOrCommand, AND, WS, SimpleCommand => ActionFn(8);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AndOrCommand = AndOrCommand, OR, SimpleCommand => ActionFn(9);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AndOrCommand = AndOrCommand, OR, WS, SimpleCommand => ActionFn(10);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 0)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AndOrCommand = SimpleCommand => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Command = Command, SEQUENCE, PipeCommand => ActionFn(1);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Command = Command, SEQUENCE, WS, PipeCommand => ActionFn(2);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 1)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Command = PipeCommand => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expand = EXPAND, WORD => ActionFn(37);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action37::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expand = EXPAND, LB, WORD, RB => ActionFn(38);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action38::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (4, 2)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expand = EXPAND, LP, Command, RP => ActionFn(39);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (4, 2)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params =  => ActionFn(13);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action13::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params = Words => ActionFn(14);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Params = Words, WS, Params => ActionFn(15);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PipeCommand = PipeCommand, PIPE, AndOrCommand => ActionFn(4);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PipeCommand = PipeCommand, PIPE, WS, AndOrCommand => ActionFn(5);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 4)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // PipeCommand = AndOrCommand => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = INPUT_REDIRECT, Words => ActionFn(19);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = INPUT_REDIRECT, WS, Words => ActionFn(20);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = OUTPUT_REDIRECT, Words => ActionFn(21);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = OUTPUT_REDIRECT, WS, Words => ActionFn(22);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action22::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = ERROR_REDIRECT, Words => ActionFn(23);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action23::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = ERROR_REDIRECT, WS, Words => ActionFn(24);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = OUTPUT_REDIRECT_APPEND, Words => ActionFn(25);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action25::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = OUTPUT_REDIRECT_APPEND, WS, Words => ActionFn(26);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = ERROR_REDIRECT_APPEND, Words => ActionFn(27);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action27::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = OUTPUT_ERROR_REDIRECT_APPEND, Words => ActionFn(28);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action28::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = OUTPUT_ERROR_REDIRECT_APPEND, WS, Words => ActionFn(29);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = OUTPUT_ERROR_REDIRECT, Words => ActionFn(30);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (2, 5)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Redirect = OUTPUT_ERROR_REDIRECT, WS, Words => ActionFn(31);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SimpleCommand = Params => ActionFn(12);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Word = WORD => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Word = EQUAL => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Word = SINGLE_QUOTES_WORD => ActionFn(34);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Word = DOUBLE_QUOTES_WORD => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Word = Expand => ActionFn(36);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Words = Word => ActionFn(16);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Words = Word, Words => ActionFn(17);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action17::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 8)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Words = Redirect => ActionFn(18);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
}
pub use self::__parse__Command::CommandParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::sh::sh_ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt<'f>(&self, formatter: &mut __fmt::Formatter<'f>) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^([\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}\u{205f}\u{3000}]+)",
                "^(\"(?:[\u{0}-!\\#-\\[\\]-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])*\")",
                "^(\'(?:[\u{0}-\\&\\(-\\[\\]-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])*\')",
                "^((?:[\u{0}-\u{8}\u{e}-\u{1f}!\\#%\\*-;\\?-\\[\\]-z\\~-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])+)",
                "^(\\$)",
                "^(\\&)",
                "^(\\&\\&)",
                "^(\\&>)",
                "^(\\&>>)",
                "^(\\()",
                "^(\\))",
                "^(2>)",
                "^(2>>)",
                "^(;)",
                "^(<)",
                "^(=)",
                "^(>)",
                "^(>>)",
                "^(\\{)",
                "^(\\|)",
                "^(\\|\\|)",
                "^(\\})",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^([\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}\u{205f}\u{3000}]+)").unwrap(),
                __regex::Regex::new("^(\"(?:[\u{0}-!\\#-\\[\\]-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])*\")").unwrap(),
                __regex::Regex::new("^(\'(?:[\u{0}-\\&\\(-\\[\\]-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])*\')").unwrap(),
                __regex::Regex::new("^((?:[\u{0}-\u{8}\u{e}-\u{1f}!\\#%\\*-;\\?-\\[\\]-z\\~-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])+)").unwrap(),
                __regex::Regex::new("^(\\$)").unwrap(),
                __regex::Regex::new("^(\\&)").unwrap(),
                __regex::Regex::new("^(\\&\\&)").unwrap(),
                __regex::Regex::new("^(\\&>)").unwrap(),
                __regex::Regex::new("^(\\&>>)").unwrap(),
                __regex::Regex::new("^(\\()").unwrap(),
                __regex::Regex::new("^(\\))").unwrap(),
                __regex::Regex::new("^(2>)").unwrap(),
                __regex::Regex::new("^(2>>)").unwrap(),
                __regex::Regex::new("^(;)").unwrap(),
                __regex::Regex::new("^(<)").unwrap(),
                __regex::Regex::new("^(=)").unwrap(),
                __regex::Regex::new("^(>)").unwrap(),
                __regex::Regex::new("^(>>)").unwrap(),
                __regex::Regex::new("^(\\{)").unwrap(),
                __regex::Regex::new("^(\\|)").unwrap(),
                __regex::Regex::new("^(\\|\\|)").unwrap(),
                __regex::Regex::new("^(\\})").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = &self.text[..];
            let __start_offset = self.consumed;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 22 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, CommandPtr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    CommandPtr::new (Command::SequentialCommand (s1, s2))
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, CommandPtr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    CommandPtr::new (Command::SequentialCommand (s1, s2))
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    s
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, CommandPtr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    CommandPtr::new (Command::PipeCommand (s1, s2))
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, CommandPtr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    CommandPtr::new (Command::PipeCommand (s1, s2))
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    s
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, CommandPtr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    CommandPtr::new (Command::AndCommand(s1, s2))
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, CommandPtr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    CommandPtr::new (Command::AndCommand(s1, s2))
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, CommandPtr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    CommandPtr::new (Command::OrCommand(s1, s2))
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, CommandPtr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    CommandPtr::new (Command::OrCommand(s1, s2))
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, CommandPtr, usize),
) -> CommandPtr
{
    s
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, mut p, _): (usize, ParametersPtr, usize),
) -> CommandPtr
{
    {
        // for parameter in p.iter() {
        //     get_redirects (redirects, parameter);
        // }
        let mut assignments = Assignments::new ();
        let mut legal_assignment = true;
        let mut redirects = Redirects::new ();
        for parameter in &mut *p {
            let mut param = Words::new ();
            for word in (*parameter).drain(..) {
                // let _:() = word;
                if let Word::Redirect (r) = *word {
                    redirects.push (r);
                }
                else
                {
                    param.push (word);
                }
            }
            *parameter = WordsPtr::new (param);
            if legal_assignment {
                legal_assignment = false;
                if parameter.len () > 1 {
                    if let Word::Word (ref w) = *parameter[0] {
                        if let Word::Word (ref eq) = *parameter[1] {
                            if eq == "=" {
                                assignments.push (AssignmentPtr::new (Assignment {
                                    variable: (&*w).to_string(),
                                    values: WordsPtr::new ((*parameter).drain (2..).collect ())
                                }));
                                (*parameter).clear ();
                                legal_assignment = true;
                            }
                        }
                    }
                }
            }
        }
        p = ParametersPtr::new (p.into_iter().filter (|parameter| parameter.len() > 0).collect());
        let s = CommandPtr::new (Command::SimpleCommand {
            assignments: AssignmentsPtr::new (assignments),
            parameters: p,
            redirects: RedirectsPtr::new (redirects.drain(..).collect())
        });
        // redirects.clear ();
        s
    }
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ParametersPtr
{
    ParametersPtr::new (Parameters::new ())
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, WordsPtr, usize),
) -> ParametersPtr
{
    {
        let mut p = ParametersPtr::new(Parameters::new ());
        p.push (w);
        p
    }
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, WordsPtr, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, mut p, _): (usize, ParametersPtr, usize),
) -> ParametersPtr
{
    {
        p.insert (0, w);
        p
    }
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, WordPtr, usize),
) -> WordsPtr
{
    {
        let mut words = WordsPtr::new (Words::new ());
        words.push (w);
        words
    }
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, WordPtr, usize),
    (_, mut e, _): (usize, WordsPtr, usize),
) -> WordsPtr
{
    {
        e.insert (0, w);
        e
    }
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, r, _): (usize, RedirectPtr, usize),
) -> WordsPtr
{
    {
        // redirects.push (r);
        let mut words = WordsPtr::new (Words::new ());
        words.push (WordPtr::new (Word::Redirect (r)));
        words
    }
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::Input {
            words: w
        });
        r
    }
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::Input {
            words: w
        });
        r
    }
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::Output {
            words: w,
            append: false
        });
        r
    }
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::Output {
            words: w,
            append: false
        });
        r
    }
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::Error {
            words: w,
            append: false
        });
        r
    }
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::Error {
            words: w,
            append: false
        });
        r
    }
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::Output {
            words: w,
            append: true
        });
        r
    }
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::Output {
            words: w,
            append: true
        });
        r
    }
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::Error {
            words: w,
            append: true
        });
        r
    }
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::OutputAndError {
            words: w,
            append: true
        });
        r
    }
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::OutputAndError {
            words: w,
            append: true
        });
        r
    }
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::OutputAndError {
            words: w,
            append: false
        });
        r
    }
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, WordsPtr, usize),
) -> RedirectPtr
{
    {
        let r = RedirectPtr::new (Redirect::OutputAndError {
            words: w,
            append: false
        });
        r
    }
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, &'input str, usize),
) -> WordPtr
{
    {
        WordPtr::new (Word::Word (w.to_string()))
    }
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> WordPtr
{
    {
        WordPtr::new (Word::Word ("=".to_string()))
    }
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, sq, _): (usize, &'input str, usize),
) -> WordPtr
{
    {
        WordPtr::new (Word::Word(sq[1..sq.len()-1].to_string()))
    }
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, sq, _): (usize, &'input str, usize),
) -> WordPtr
{
    {
        WordPtr::new (Word::Quotes(sq[1..sq.len()-1].to_string()))
    }
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, e, _): (usize, WordPtr, usize),
) -> WordPtr
{
    e
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, &'input str, usize),
) -> WordPtr
{
    {
        WordPtr::new (Word::Expand (w.to_string()))
    }
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> WordPtr
{
    {
        WordPtr::new (Word::Expand (w.to_string()))
    }
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, CommandPtr, usize),
    (_, _, _): (usize, &'input str, usize),
) -> WordPtr
{
    {
        WordPtr::new (Word::Execute (s))
    }
}

pub trait __ToTriple<'input, > {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str> {
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
