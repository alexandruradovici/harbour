// auto-generated: "lalrpop 0.17.2"
// sha256: d9edd0e3df6679deb5b7693add590593c772d76ee557246b73886166bd9762
use std::str::FromStr;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Command {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(String),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, -18, 0, -18, -18, -18,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, -5, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, -12, 0, 0, 0, 14,
        // State 6
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, -19, 0, 10, 16, -19,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, -14, 0, 0, 0, -14,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0,
        // State 9
        0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, -17, 0, -17, -17, -17,
        // State 10
        0, 0, 20, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, -16, 0, -16, -16, -16,
        // State 11
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 0,
        // State 12
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 23,
        // State 13
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, 0, 10, 16, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, -20, 0, 0, 0, -20,
        // State 15
        0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, -16, 0, -16, -16, -16,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0,
        // State 17
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 0,
        // State 18
        0, 0, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, -6, 0, -6, -6, -6,
        // State 19
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 16, -2,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, -15, 0, 0, 0, -15,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 11, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, -13, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, -10, 0, 0, 0, 30,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, -9, 0, 10, 16, 0,
        // State 30
        0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, -7, 0, -7, -7, -7,
        // State 31
        0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, -8, 0, -8, -8, -8,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, -11, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -21,
        // State 3
        -18,
        // State 4
        -5,
        // State 5
        -12,
        // State 6
        -19,
        // State 7
        -14,
        // State 8
        0,
        // State 9
        -17,
        // State 10
        -16,
        // State 11
        0,
        // State 12
        0,
        // State 13
        -9,
        // State 14
        -20,
        // State 15
        -16,
        // State 16
        0,
        // State 17
        0,
        // State 18
        -6,
        // State 19
        0,
        // State 20
        -15,
        // State 21
        -3,
        // State 22
        0,
        // State 23
        -13,
        // State 24
        -10,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -4,
        // State 29
        -9,
        // State 30
        -7,
        // State 31
        -8,
        // State 32
        -11,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        2, 3, 4, 0, 5, 6, 7, 8, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 4, 0, 0, 0, 7, 15, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        2, 0, 4, 0, 0, 21, 7, 8, 0,
        // State 12
        2, 0, 4, 0, 22, 6, 7, 8, 0,
        // State 13
        0, 0, 4, 24, 0, 0, 7, 25, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        2, 27, 4, 0, 5, 6, 7, 8, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 4, 0, 0, 0, 7, 28, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        2, 0, 4, 0, 29, 6, 7, 8, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 4, 33, 0, 0, 7, 25, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"AND"###,
            r###"BACKGROUND"###,
            r###"EQUAL"###,
            r###"ERROR_REDIRECT"###,
            r###"ERROR_REDIRECT_APPEND"###,
            r###"EXPAND"###,
            r###"INPUT_REDIRECT"###,
            r###"LB"###,
            r###"LP"###,
            r###"NUMBER"###,
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
        type Success = String;
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
            __GOTO[(state as usize) * 9 + nt] - 1
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
            Token(15, _) if true => Some(2),
            Token(11, _) if true => Some(3),
            Token(12, _) if true => Some(4),
            Token(4, _) if true => Some(5),
            Token(14, _) if true => Some(6),
            Token(18, _) if true => Some(7),
            Token(9, _) if true => Some(8),
            Token(0, _) if true => Some(9),
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
            Token(1, _) if true => Some(21),
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
                Token(15, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            3 => match __token {
                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            4 => match __token {
                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            5 => match __token {
                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            6 => match __token {
                Token(14, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            7 => match __token {
                Token(18, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            8 => match __token {
                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                _ => unreachable!(),
            },
            9 => match __token {
                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
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
                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
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
                    states_to_pop: 2,
                    nonterminal_produced: 0,
                }
            }
            2 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 1,
                }
            }
            3 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 1,
                }
            }
            4 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 1,
                }
            }
            5 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 2,
                }
            }
            6 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 2,
                }
            }
            7 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 4,
                    nonterminal_produced: 2,
                }
            }
            8 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 0,
                    nonterminal_produced: 3,
                }
            }
            9 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 3,
                }
            }
            10 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 3,
                }
            }
            11 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 4,
                }
            }
            12 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 4,
                }
            }
            13 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 5,
                }
            }
            14 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 3,
                    nonterminal_produced: 5,
                }
            }
            15 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            16 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            17 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 6,
                }
            }
            18 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 1,
                    nonterminal_produced: 7,
                }
            }
            19 => {
                __state_machine::SimulatedReduce::Reduce {
                    states_to_pop: 2,
                    nonterminal_produced: 7,
                }
            }
            20 => __state_machine::SimulatedReduce::Accept,
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
        ) -> Result<String, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
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
    ) -> Option<Result<String,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
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
        let __next_state = __GOTO[__state * 9 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
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
        // Assignment = WORD, EQUAL, Words => ActionFn(11);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
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
        // Assignment = WORD, EQUAL => ActionFn(12);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
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
        // Command = SimpleCommand, PIPE, SimpleCommand => ActionFn(1);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
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
        // Command = SimpleCommand, PIPE, WS, SimpleCommand => ActionFn(2);
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
        // Command = SimpleCommand => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 1)
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
        // Expand = EXPAND, WORD => ActionFn(18);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action18::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 2)
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
        // Expand = EXPAND, LB, WORD, RB => ActionFn(19);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 2)
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
        // Expand = EXPAND, LP, Command, RP => ActionFn(20);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 2)
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
        // Params =  => ActionFn(8);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action8::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (0, 3)
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
        // Params = Words => ActionFn(9);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
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
        // Params = Words, WS, Params => ActionFn(10);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 3)
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
        // SimpleCommand = SimpleCommandElement => ActionFn(4);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 4)
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
        // SimpleCommand = SimpleCommandElement, WS, Params => ActionFn(5);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
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
        // SimpleCommandElement = Words => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 5)
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
        // SimpleCommandElement = Assignment, WS, SimpleCommandElement => ActionFn(7);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 5)
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
        // Word = WORD => ActionFn(15);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
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
        // Word = SINGLE_QUOTES_WORD => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
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
        // Word = Expand => ActionFn(17);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
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
        // Words = Word => ActionFn(13);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
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
        // Words = Word, Words => ActionFn(14);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action14::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 7)
    }
}
pub use self::__parse__Command::CommandParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
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
                "^([0-9]+)",
                "^([\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}\u{205f}\u{3000}]+)",
                "^(\'(?:[\u{0}-\\&\\(-\\[\\]-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])*\')",
                "^((?:[\u{0}-\u{8}\u{e}-\u{1f}!\\#%\\*-13-;=\\?-\\[\\]-z\\~-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}]|2[\u{0}-=\\?-\u{10ffff}])*)",
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
                __regex::Regex::new("^([0-9]+)").unwrap(),
                __regex::Regex::new("^([\t-\r \u{85}\u{a0}\u{1680}\u{2000}-\u{200a}\u{2028}-\u{2029}\u{202f}\u{205f}\u{3000}]+)").unwrap(),
                __regex::Regex::new("^(\'(?:[\u{0}-\\&\\(-\\[\\]-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}])*\')").unwrap(),
                __regex::Regex::new("^((?:[\u{0}-\u{8}\u{e}-\u{1f}!\\#%\\*-13-;=\\?-\\[\\]-z\\~-\u{84}\u{86}-\u{9f}¡-ᙿᚁ-\u{1fff}\u{200b}-‧\u{202a}-\u{202e}‰-⁞\u{2060}-\u{2fff}、-\u{10ffff}]|\\\\[\u{0}-\t\u{b}-\u{10ffff}]|2[\u{0}-=\\?-\u{10ffff}])*)").unwrap(),
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
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, String, usize),
) -> String
{
    format! ("(command1 {}) | (command2 {})", s1, s2)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, s1, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s2, _): (usize, String, usize),
) -> String
{
    format! ("(command1 {}) | (command2 {})", s1, s2)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, String, usize),
) -> String
{
    format! ("{}", s)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, e, _): (usize, String, usize),
) -> String
{
    format! ("(command {}) (parameters)", e)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, e, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, String, usize),
) -> String
{
    format! ("(command {}) (parameters {})", e, p)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, String, usize),
) -> String
{
    w
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, a, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, String, usize),
) -> String
{
    format! ("(assign {}) {}", a, s)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> String
{
    String::from("")
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, String, usize),
) -> String
{
    {
        println! ("(parameter {})", w);
        format! ("(parameter {})", w)
    }
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, String, usize),
) -> String
{
    {
        println! ("(parameter {}) {}", w, p);
        format! ("(parameter {}) {}", w, p)
    }
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, w1, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w2, _): (usize, String, usize),
) -> String
{
    format! ("{} := {}", w1, w2)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format! ("{} := ()", w)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, String, usize),
) -> String
{
    w
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, String, usize),
    (_, e, _): (usize, String, usize),
) -> String
{
    {
        println! ("(words (word {}) {})", w, e);
        format! ("(word {}) {}", w, e)
    }
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, w, _): (usize, &'input str, usize),
) -> String
{
    {
        println! ("{}", w);
        w.to_string()
    }
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, sq, _): (usize, &'input str, usize),
) -> String
{
    {
        println! ("(single quotes {})", sq);
        sq.to_string()
    }
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, e, _): (usize, String, usize),
) -> String
{
    e
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, &'input str, usize),
) -> String
{
    {
        println! ("(expand {})", w);
        format! ("(expand {})", w)
    }
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    {
        println! ("(expand {})", w);
        format! ("(expand {})", w)
    }
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, s, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    {
        println! ("(exe ({}))", s);
        format! ("(exe ({}))", s)
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
