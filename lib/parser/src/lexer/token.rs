use crate::lexer::span::Span;
use lazy_static::lazy_static;

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! constant_collection {
    ($collection:ident : $($ident:ident = $value:literal),*,) => {
        $(
        const $ident: &'static str = $value;
        )*


        lazy_static! {
            pub static ref $collection: [&'static str; count!($($ident)*)] = [
                $($ident),*
            ];
        }

    };
}

constant_collection! {
    // These are sorted, so that e.g. 'double' comes before 'do', so that if we
    // check for all keywords using this array, we don't run into prefix-related
    // problems. Other than that, this array is sorted alphabetically.
    KEYWORDS:
    KEYWORD_ABSTRACT = "abstract",
    KEYWORD_ASSERT = "assert",
    KEYWORD_BOOLEAN = "boolean",
    KEYWORD_BREAK = "break",
    KEYWORD_BYTE = "byte",
    KEYWORD_CASE = "case",
    KEYWORD_CATCH = "catch",
    KEYWORD_CHAR = "char",
    KEYWORD_CLASS = "class",
    KEYWORD_CONST = "const",
    KEYWORD_CONTINUE = "continue",
    KEYWORD_DEFAULT = "default",
    KEYWORD_DOUBLE = "double",
    KEYWORD_DO = "do",
    KEYWORD_ELSE = "else",
    KEYWORD_ENUM = "enum",
    KEYWORD_EXTENDS = "extends",
    KEYWORD_FINALLY = "finally",
    KEYWORD_FINAL = "final",
    KEYWORD_FLOAT = "float",
    KEYWORD_FOR = "for",
    KEYWORD_GOTO = "goto",
    KEYWORD_IF = "if",
    KEYWORD_IMPLEMENTS = "implements",
    KEYWORD_IMPORT = "import",
    KEYWORD_INSTANCEOF = "instanceof",
    KEYWORD_INTERFACE = "interface",
    KEYWORD_INT = "int",
    KEYWORD_LONG = "long",
    KEYWORD_NATIVE = "native",
    KEYWORD_NEW = "new",
    KEYWORD_PACKAGE = "package",
    KEYWORD_PRIVATE = "private",
    KEYWORD_PROTECTED = "protected",
    KEYWORD_PUBLIC = "public",
    KEYWORD_RETURN = "return",
    KEYWORD_SHORT = "short",
    KEYWORD_STATIC = "static",
    KEYWORD_STRICTFP = "strictfp",
    KEYWORD_SUPER = "super",
    KEYWORD_SWITCH = "switch",
    KEYWORD_SYNCHRONIZED = "synchronized",
    KEYWORD_THIS = "this",
    KEYWORD_THROWS = "throws",
    KEYWORD_THROW = "throw",
    KEYWORD_TRANSIENT = "transient",
    KEYWORD_TRY = "try",
    KEYWORD_VOID = "void",
    KEYWORD_VOLATILE = "volatile",
    KEYWORD_WHILE = "while",
}

constant_collection! {
    SEPARATORS:
    SEPARATOR_SEMICOLON = ";",
    SEPARATOR_COMMA = ",",
    SEPARATOR_PERIOD = ".",
    SEPARATOR_LEFT_PAR = "(",
    SEPARATOR_RIGHT_PAR = ")",
    SEPARATOR_LEFT_CURLY = "{",
    SEPARATOR_RIGHT_CURLY = "}",
    SEPARATOR_LEFT_BRACKET = "[",
    SEPARATOR_RIGHT_BRACKET = "]",
}

constant_collection! {
    BOOLEAN_VALUES:
    BOOLEAN_TRUE = "true",
    BOOLEAN_FALSE = "false",
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Keyword(Keyword),
    Ident(Ident),
    Literal(Literal),
    Operator(Operator),
    Separator(Separator),
    Comment(Comment),
}

impl Token {
    pub fn span(&self) -> &Span {
        match self {
            Token::Ident(ident) => &ident.span,
            Token::Literal(literal) => literal.span(),
            Token::Keyword(keyword) => keyword.span(),
            Token::Operator(operator) => operator.span(),
            Token::Separator(separator) => separator.span(),
            Token::Comment(comment) => comment.span(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Ident {
    span: Span,
}

impl Ident {
    pub fn new(span: Span) -> Self {
        Ident { span }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

macro_rules! token_type {
    ($token_type:ident: $($name:ident: $constructor_name:ident),*,) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum $token_type {
            $($name($crate::lexer::token::Span)),*
        }

        impl $token_type {
            $(
                pub fn $constructor_name(span: $crate::lexer::token::Span) -> Self {
                    Self::$name(span)
                }
            )*

            pub fn span(&self) -> &$crate::lexer::token::Span {
                match self {
                    $(Self::$name(span) => span),*
                }
            }
        }
    };
}

macro_rules! try_from_str {
    ($ty:ident: $($name:ident: $value:expr),*,) => {
        impl<'a> $ty {
            pub fn try_from_str(s: &'a str, span:Span) -> Option<Self> {
                match s {
                    $(x if x == $value => Some(Self::$name(span))),*,
                    _ => None,
                }
            }
        }
    };
}

token_type! {
    Keyword:
    Abstract: new_abstract,
    Boolean: new_boolean,
    Byte: new_byte,
    Break: new_break,
    Class: new_class,
    Case: new_case,
    Catch: new_catch,
    Char: new_char,
    Continue: new_continue,
    Default: new_default,
    Do: new_do,
    Double: new_double,
    Else: new_else,
    Extends: new_extends,
    Final: new_final,
    Finally: new_finally,
    Float: new_float,
    For: new_for,
    If: new_if,
    Implements: new_implements,
    Import: new_import,
    InstanceOf: new_instance_of,
    Int: new_int,
    Interface: new_interface,
    Long: new_long,
    Native: new_native,
    New: new_new,
    Package: new_package,
    Private: new_private,
    Protected: new_protected,
    Public: new_public,
    Return: new_return,
    Short: new_short,
    Static: new_static,
    Super: new_super,
    Switch: new_switch,
    Synchronized: new_synchronized,
    This: new_this,
    Throw: new_throw,
    Throws: new_throws,
    Transient: new_transient,
    Try: new_try,
    Void: new_void,
    Volatile: new_volatile,
    While: new_while,
    Assert: new_assert,
    Const: new_const,
    Enum: new_enum,
    Goto: new_goto,
    Strictfp: new_strictfp,
}

try_from_str! {
    Keyword:
    Abstract: KEYWORD_ABSTRACT,
    Boolean: KEYWORD_BOOLEAN,
    Byte: KEYWORD_BYTE,
    Break: KEYWORD_BREAK,
    Class: KEYWORD_CLASS,
    Case: KEYWORD_CASE,
    Catch: KEYWORD_CATCH,
    Char: KEYWORD_CHAR,
    Continue: KEYWORD_CONTINUE,
    Default: KEYWORD_DEFAULT,
    Do: KEYWORD_DO,
    Double: KEYWORD_DOUBLE,
    Else: KEYWORD_ELSE,
    Extends: KEYWORD_EXTENDS,
    Final: KEYWORD_FINAL,
    Finally: KEYWORD_FINALLY,
    Float: KEYWORD_FLOAT,
    For: KEYWORD_FOR,
    If: KEYWORD_IF,
    Implements: KEYWORD_IMPLEMENTS,
    Import: KEYWORD_IMPORT,
    InstanceOf: KEYWORD_INSTANCEOF,
    Int: KEYWORD_INT,
    Interface: KEYWORD_INTERFACE,
    Long: KEYWORD_LONG,
    Native: KEYWORD_NATIVE,
    New: KEYWORD_NEW,
    Package: KEYWORD_PACKAGE,
    Private: KEYWORD_PRIVATE,
    Protected: KEYWORD_PROTECTED,
    Public: KEYWORD_PUBLIC,
    Return: KEYWORD_RETURN,
    Short: KEYWORD_SHORT,
    Static: KEYWORD_STATIC,
    Super: KEYWORD_SUPER,
    Switch: KEYWORD_SWITCH,
    Synchronized: KEYWORD_SYNCHRONIZED,
    This: KEYWORD_THIS,
    Throw: KEYWORD_THROW,
    Throws: KEYWORD_THROWS,
    Transient: KEYWORD_TRANSIENT,
    Try: KEYWORD_TRY,
    Void: KEYWORD_VOID,
    Volatile: KEYWORD_VOLATILE,
    While: KEYWORD_WHILE,
    Assert: KEYWORD_ASSERT,
    Const: KEYWORD_CONST,
    Enum: KEYWORD_ENUM,
    Goto: KEYWORD_GOTO,
    Strictfp: KEYWORD_STRICTFP,
}

token_type! {
    Literal:
    Integer: new_integer,
    FloatingPoint: new_floating_point,
    Character: new_character,
    String: new_string,
    Boolean: new_boolean,
}

token_type! {
    Operator:
    Arithmetic: new_arithmetic,
    Assignment: new_assignment,
    Relational: new_relational,
    Unary: new_unary,
    Logical: new_logical,
    Bitwise: new_bitwise,
    Shift: new_shift,
    QuestionMark: new_question_mark,
    Colon: new_colon,
}

token_type! {
    Separator:
    Semicolon: new_semicolon,
    Comma: new_comma,
    Dot: new_period,
    LeftPar: new_left_par,
    RightPar: new_right_par,
    LeftCurly: new_left_curly,
    RightCurly: new_right_curly,
    LeftBracket: new_left_bracket,
    RightBracket: new_right_bracket,
}

try_from_str! {
    Separator:
    Semicolon: SEPARATOR_SEMICOLON,
    Comma: SEPARATOR_COMMA,
    Dot: SEPARATOR_PERIOD,
    LeftPar: SEPARATOR_LEFT_PAR,
    RightPar: SEPARATOR_RIGHT_PAR,
    LeftCurly: SEPARATOR_LEFT_CURLY,
    RightCurly: SEPARATOR_RIGHT_CURLY,
    LeftBracket: SEPARATOR_LEFT_BRACKET,
    RightBracket: SEPARATOR_RIGHT_BRACKET,
}

token_type! {
    Comment:
    Line: new_line,
    Block: new_block,
}
