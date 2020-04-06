// auto-generated: "lalrpop 0.18.1"
// sha256: 4cafc9a6e73e695717e47da955adbb6e2aa75e17efe5d11214ee72b8d69796
use crate::TokenTree;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__TokenTree {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::TokenTree;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1((TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)),
        Variant2(::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>),
        Variant3(Vec<TokenTree>),
        Variant4(TokenTree),
        Variant5(::std::vec::Vec<TokenTree>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        7, 8, 0, 0, 0, 9, 0, 0, 10,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        -36, -36, 0, 0, 0, -36, 0, 0, -36,
        // State 3
        7, 8, 0, 0, 0, 9, 0, 0, 10,
        // State 4
        0, 0, -15, 17, 18, 19, 20, 21, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        -33, -33, 0, 0, 0, -33, 0, 0, -33,
        // State 7
        7, 8, 0, 0, 0, 9, 0, 0, 10,
        // State 8
        -32, -32, 0, 0, 0, -32, 0, 0, -32,
        // State 9
        0, 0, -39, -39, -39, -39, -39, -39, 0,
        // State 10
        -37, -37, 0, 0, 0, -37, 0, 0, -37,
        // State 11
        0, 0, -17, 17, 18, 19, 20, 21, 0,
        // State 12
        0, 0, -16, 17, 18, 19, 20, 0, 0,
        // State 13
        7, 8, 0, 0, 0, 9, 0, 0, 10,
        // State 14
        0, 0, -30, -30, -30, -30, -30, -30, 0,
        // State 15
        0, 0, -19, 17, 18, 19, 20, 21, 0,
        // State 16
        -25, -25, 0, 0, 0, -25, 0, 0, -25,
        // State 17
        -23, -23, 0, 0, 0, -23, 0, 0, -23,
        // State 18
        -24, -24, 0, 0, 0, -24, 0, 0, -24,
        // State 19
        -26, -26, 0, 0, 0, -26, 0, 0, -26,
        // State 20
        0, 0, -27, -27, -27, -27, -27, -27, 0,
        // State 21
        0, 0, 30, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, -18, 17, 18, 19, 20, 0, 0,
        // State 23
        0, 0, -21, 17, 18, 19, 20, 21, 0,
        // State 24
        7, 8, 0, 0, 0, 9, 0, 0, 10,
        // State 25
        7, 8, 0, 0, 0, 9, 0, 0, 10,
        // State 26
        0, 0, -7, -7, -7, -7, -7, 21, 0,
        // State 27
        0, 0, -20, 17, 18, 19, 20, 0, 0,
        // State 28
        0, 0, -31, -31, -31, -31, -31, -31, 0,
        // State 29
        0, 0, -38, -38, -38, -38, -38, -38, 0,
        // State 30
        0, 0, -22, 17, 18, 19, 20, 0, 0,
        // State 31
        7, 8, 0, 0, 0, 9, 0, 0, 10,
        // State 32
        0, 0, -11, -11, -11, -11, -11, 21, 0,
        // State 33
        0, 0, -8, -8, -8, -8, -8, 21, 0,
        // State 34
        0, 0, -9, -9, -9, -9, -9, 21, 0,
        // State 35
        0, 0, -12, -12, -12, -12, -12, 21, 0,
        // State 36
        0, 0, -13, -13, -13, -13, -13, 21, 0,
        // State 37
        0, 0, -10, -10, -10, -10, -10, 21, 0,
        // State 38
        0, 0, -14, -14, -14, -14, -14, 21, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -40,
        // State 2
        0,
        // State 3
        0,
        // State 4
        -15,
        // State 5
        -41,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        -39,
        // State 10
        0,
        // State 11
        -17,
        // State 12
        -16,
        // State 13
        0,
        // State 14
        -30,
        // State 15
        -19,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -27,
        // State 21
        0,
        // State 22
        -18,
        // State 23
        -21,
        // State 24
        0,
        // State 25
        0,
        // State 26
        -7,
        // State 27
        -20,
        // State 28
        -31,
        // State 29
        -38,
        // State 30
        -22,
        // State 31
        0,
        // State 32
        -11,
        // State 33
        -8,
        // State 34
        -9,
        // State 35
        -12,
        // State 36
        -13,
        // State 37
        -10,
        // State 38
        -14,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 4, 5, 6, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 12, 0, 0,
        // State 4
        0, 0, 13, 0, 14, 15, 0, 16, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 22, 0, 0, 0, 0, 3, 0, 4, 5, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 23, 0, 14, 15, 0, 24, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 26, 27, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 28, 0, 14, 29, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 31, 0, 14, 29, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 32, 33, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 34, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 15, 0, 35, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 36, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 15, 0, 37, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 15, 0, 38, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 15, 0, 39, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###""?""###,
            r###"r#"[0-9]+"#"###,
        ];
        __ACTION[(__state * 9)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
        type Success = Vec<TokenTree>;
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
            __ACTION[(state as usize) * 9 + integer]
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __ACTION[(state as usize) * 9 + (9 - 1)]
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __GOTO[(state as usize) * 14 + nt] - 1
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
            panic!("error recovery not enabled for this grammar")
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
            Token(1, _) if true => Some(0),
            Token(2, _) if true => Some(1),
            Token(3, _) if true => Some(2),
            Token(4, _) if true => Some(3),
            Token(5, _) if true => Some(4),
            Token(6, _) if true => Some(5),
            Token(7, _) if true => Some(6),
            Token(8, _) if true => Some(7),
            Token(0, _) if true => Some(8),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => match __token {
                Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(0, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct TokenTreeParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl TokenTreeParser {
        pub fn new() -> TokenTreeParser {
            let __builder = super::__intern_token::new_builder();
            TokenTreeParser {
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
        ) -> Result<Vec<TokenTree>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
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
    ) -> Option<Result<Vec<TokenTree>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            35 => {
                __reduce35(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            40 => {
                // __TokenTree = TokenTree => ActionFn(0);
                let __sym0 = __pop_Variant3(__symbols);
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
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TokenTree, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<TokenTree>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize)
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
    ) -> (usize, ::std::vec::Vec<TokenTree>, usize)
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
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*) = Infix, Primary => ActionFn(29);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action29::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*) = Infix, Prefix+, Primary => ActionFn(30);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action30::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*) = Infix, Primary, Postfix+ => ActionFn(31);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*) = Infix, Prefix+, Primary, Postfix+ => ActionFn(32);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action32::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 0)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)* =  => ActionFn(12);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action12::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)* = (Infix Prefix* Primary Postfix*)+ => ActionFn(13);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)+ = Infix, Primary => ActionFn(37);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action37::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)+ = Infix, Prefix+, Primary => ActionFn(38);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action38::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)+ = Infix, Primary, Postfix+ => ActionFn(39);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)+ = Infix, Prefix+, Primary, Postfix+ => ActionFn(40);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action40::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (4, 2)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)+ = (Infix Prefix* Primary Postfix*)+, Infix, Primary => ActionFn(41);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action41::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)+ = (Infix Prefix* Primary Postfix*)+, Infix, Prefix+, Primary => ActionFn(42);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action42::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (4, 2)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)+ = (Infix Prefix* Primary Postfix*)+, Infix, Primary, Postfix+ => ActionFn(43);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant5(__symbols);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action43::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (4, 2)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (Infix Prefix* Primary Postfix*)+ = (Infix Prefix* Primary Postfix*)+, Infix, Prefix+, Primary, Postfix+ => ActionFn(44);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant5(__symbols);
        let __sym3 = __pop_Variant4(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action44::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (5, 2)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Group = Primary => ActionFn(45);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action45::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Group = Primary, (Infix Prefix* Primary Postfix*)+ => ActionFn(46);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action46::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Group = Prefix+, Primary => ActionFn(47);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action47::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Group = Prefix+, Primary, (Infix Prefix* Primary Postfix*)+ => ActionFn(48);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action48::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Group = Primary, Postfix+ => ActionFn(49);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action49::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Group = Primary, Postfix+, (Infix Prefix* Primary Postfix*)+ => ActionFn(50);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action50::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Group = Prefix+, Primary, Postfix+ => ActionFn(51);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action51::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Group = Prefix+, Primary, Postfix+, (Infix Prefix* Primary Postfix*)+ => ActionFn(52);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant2(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action52::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (4, 3)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Infix = "+" => ActionFn(5);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Infix = "-" => ActionFn(6);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Infix = "*" => ActionFn(7);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Infix = "/" => ActionFn(8);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Postfix = "?" => ActionFn(11);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Postfix* =  => ActionFn(15);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action15::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 6)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Postfix* = Postfix+ => ActionFn(16);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Postfix+ = Postfix => ActionFn(21);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Postfix+ = Postfix+, Postfix => ActionFn(22);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action22::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prefix = "-" => ActionFn(9);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prefix = "!" => ActionFn(10);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prefix* =  => ActionFn(17);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action17::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (0, 9)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prefix* = Prefix+ => ActionFn(18);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce35<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prefix+ = Prefix => ActionFn(19);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Prefix+ = Prefix+, Prefix => ActionFn(20);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action20::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 10)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Primary = "(", Group, ")" => ActionFn(3);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Primary = r#"[0-9]+"# => ActionFn(4);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // TokenTree = Group => ActionFn(1);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 12)
    }
}
pub use self::__parse__TokenTree::TokenTreeParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::TokenTree;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^([0-9]+)", false),
            ("^(!)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\+)", false),
            ("^(\\-)", false),
            ("^(/)", false),
            ("^(\\?)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<TokenTree>, usize),
) -> Vec<TokenTree>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<TokenTree>, usize),
) -> Vec<TokenTree>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, prefix, _): (usize, ::std::vec::Vec<TokenTree>, usize),
    (_, primary, _): (usize, TokenTree, usize),
    (_, mut postfix, _): (usize, ::std::vec::Vec<TokenTree>, usize),
    (_, rest, _): (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    {
    let mut group = prefix;
    group.push(primary);
    group.append(&mut postfix);
    for (infix, mut prefix, primary, mut postfix) in rest {
        group.push(infix);
        group.append(&mut prefix);
        group.push(primary);
        group.append(&mut postfix);
    }
    group
}
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<TokenTree>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> TokenTree
{
    TokenTree::Group(__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> TokenTree
{
    TokenTree::Primary(__0.parse::<i32>().unwrap())
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> TokenTree
{
    TokenTree::Infix('+')
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> TokenTree
{
    TokenTree::Infix('-')
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> TokenTree
{
    TokenTree::Infix('*')
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> TokenTree
{
    TokenTree::Infix('/')
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> TokenTree
{
    TokenTree::Prefix('-')
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> TokenTree
{
    TokenTree::Prefix('!')
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> TokenTree
{
    TokenTree::Postfix('?')
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    vec![]
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    v
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TokenTree, usize),
    (_, __1, _): (usize, ::std::vec::Vec<TokenTree>, usize),
    (_, __2, _): (usize, TokenTree, usize),
    (_, __3, _): (usize, ::std::vec::Vec<TokenTree>, usize),
) -> (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)
{
    (__0, __1, __2, __3)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<TokenTree>
{
    vec![]
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<TokenTree>, usize),
) -> ::std::vec::Vec<TokenTree>
{
    v
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<TokenTree>
{
    vec![]
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<TokenTree>, usize),
) -> ::std::vec::Vec<TokenTree>
{
    v
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TokenTree, usize),
) -> ::std::vec::Vec<TokenTree>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<TokenTree>, usize),
    (_, e, _): (usize, TokenTree, usize),
) -> ::std::vec::Vec<TokenTree>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TokenTree, usize),
) -> ::std::vec::Vec<TokenTree>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<TokenTree>, usize),
    (_, e, _): (usize, TokenTree, usize),
) -> ::std::vec::Vec<TokenTree>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>), usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
    (_, e, _): (usize, (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>), usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<TokenTree>, usize),
    __2: (usize, TokenTree, usize),
) -> (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action15(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<TokenTree>, usize),
    __2: (usize, TokenTree, usize),
    __3: (usize, ::std::vec::Vec<TokenTree>, usize),
) -> (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action16(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<TokenTree>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action15(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<TokenTree>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<TokenTree>, usize),
    __3: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action16(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, TokenTree, usize),
) -> (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<TokenTree>, usize),
    __2: (usize, TokenTree, usize),
) -> (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<TokenTree>, usize),
) -> (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        input,
        __0,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<TokenTree>, usize),
    __2: (usize, TokenTree, usize),
    __3: (usize, ::std::vec::Vec<TokenTree>, usize),
) -> (TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action18(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        input,
        __0,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __temp0,
        __0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<TokenTree>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        input,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<TokenTree>, usize),
    __2: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action17(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        __temp0,
        __0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<TokenTree>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<TokenTree>, usize),
    __3: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action18(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        __temp0,
        __1,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, TokenTree, usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action29(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<TokenTree>, usize),
    __2: (usize, TokenTree, usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action30(
        input,
        __0,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<TokenTree>, usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action31(
        input,
        __0,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<TokenTree>, usize),
    __2: (usize, TokenTree, usize),
    __3: (usize, ::std::vec::Vec<TokenTree>, usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    let __start0 = __0.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action32(
        input,
        __0,
        __1,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, TokenTree, usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action29(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<TokenTree>, usize),
    __3: (usize, TokenTree, usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action30(
        input,
        __1,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, TokenTree, usize),
    __3: (usize, ::std::vec::Vec<TokenTree>, usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action31(
        input,
        __1,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<TokenTree>, usize),
    __3: (usize, TokenTree, usize),
    __4: (usize, ::std::vec::Vec<TokenTree>, usize),
) -> ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>
{
    let __start0 = __1.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action32(
        input,
        __1,
        __2,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
) -> Vec<TokenTree>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action12(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action13(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<TokenTree>, usize),
    __1: (usize, TokenTree, usize),
) -> Vec<TokenTree>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action12(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<TokenTree>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action13(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<TokenTree>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action12(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    input: &'input str,
    __0: (usize, TokenTree, usize),
    __1: (usize, ::std::vec::Vec<TokenTree>, usize),
    __2: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action13(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action35(
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<TokenTree>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<TokenTree>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action12(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<TokenTree>, usize),
    __1: (usize, TokenTree, usize),
    __2: (usize, ::std::vec::Vec<TokenTree>, usize),
    __3: (usize, ::std::vec::Vec<(TokenTree, ::std::vec::Vec<TokenTree>, TokenTree, ::std::vec::Vec<TokenTree>)>, usize),
) -> Vec<TokenTree>
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action13(
        input,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action36(
        input,
        __0,
        __1,
        __2,
        __temp0,
    )
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
