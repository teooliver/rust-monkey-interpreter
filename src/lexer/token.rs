struct TokenType {
    Type: string,
    Literal: string,
}

const ILLEGAL: String = "ILLEGAL";
const EOF: String = "EOF";

// Identifiers + literals
const IDENT: String = "IDENT"; // add, foobar, x, y, ...
const INT: String = "INT";

// Operators
const ASSIGN: String = "=";
const PLUS: String = "+";

// Delimiters
const COMMA: String = ",";
const SEMICOLON: String = ";";
const LPAREN: String = "(";
const RPAREN: String = ")";
const LBRACE: String = "{";
const RBRACE: String = "}";

// Keywords
const FUNCTION: String = "FUNCTION";
const LET: String = "LET";
