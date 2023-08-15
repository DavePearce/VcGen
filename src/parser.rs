use std::collections::HashMap;
use crate::lexer::{EOF,Lexer,Token,TokenType};
use crate::Bytecode;

/// Defines the set of tokens which are considered to identify logical
/// connectives (e.g. `&&`, `||`, etc).
pub const LOGICAL_CONNECTIVES : &[TokenType] = &[
    TokenType::AmpersandAmpersand,
    TokenType::BarBar,
    TokenType::LongRightArrow
];

/// Defines the set of tokens which are considered to identify
/// arithmetic comparators (e.g. `<`, `<=`, `==`, etc).
pub const ARITHMETIC_COMPARATORS : &[TokenType] = &[
    TokenType::EqualsEquals,
    TokenType::ShreakEquals,
    TokenType::LeftAngle,
    TokenType::LeftAngleEquals,
    TokenType::RightAngle,
    TokenType::RightAngleEquals
];

/// Defines the set of tokens which are considered to identify
/// arithmetic operators (e.g. `+`, `-`, `*`, etc).
pub const ARITHMETIC_OPERATORS : &[TokenType] = &[
    TokenType::Minus,
    TokenType::Percent,
    TokenType::Plus,
    TokenType::RightSlash,
    TokenType::Star
];

pub const BINARY_CONNECTIVES : &[ &[TokenType] ] = &[
    ARITHMETIC_OPERATORS,
    ARITHMETIC_COMPARATORS,
    LOGICAL_CONNECTIVES
];

pub const TYPES : &[TokenType] = &[
    TokenType::Uint
];

// ===================================================================
// Parser
// ===================================================================

/// Simplest possible parser.  Its a combination lexer and parser!
pub struct Parser {
    /// Character sequence being parsed
    lexer: Lexer,
    /// Environment used for determining indices
    env: Environment,
    /// Bytecode program being constructed
    bytecodes: Vec<Bytecode>
}

impl Parser {
    /// Construct a parser from a string slice.
    pub fn new(content: &str) -> Self {
        // Convert string slice into Vec<char>
        let lexer = Lexer::new(content);
        //
        let env = Environment::new();
        // Done
        Self{lexer, env, bytecodes: Vec::new()}
    }

    /// Parse a line of text into a term.
    pub fn parse(mut self) -> Result<Vec<Bytecode>,()> {
        while self.lexer.lookahead(0) != EOF {
	    self.parse_declaration()?;
        }
        //
        Ok(self.bytecodes)
    }

    fn parse_declaration(&mut self) -> Result<(),()> {
        let lookahead = self.lexer.lookahead(0);
        //
        match lookahead.kind {
            TokenType::ForAll => self.parse_decl_forall(),
            TokenType::Fun => self.parse_decl_fun(),
	    TokenType::RecFun => self.parse_decl_recfun(),
            _ => {
                self.parse_decl_assert()
            }
        }
    }

    // ===============================================================
    // Declarations
    // ===============================================================

    fn parse_decl_assert(&mut self) -> Result<(),()> {
	// Record assertion
	self.bytecodes.push(Bytecode::Assert);
	// Parse asserted expression
	self.parse_expr();
	// Done
	Ok(())
    }

    fn parse_decl_forall(&mut self) -> Result<(),()> {
        self.lexer.expect(TokenType::ForAll);
        // Parse quantified variables
        let params = self.parse_decl_params()?;
        // Allocate params within environment
        self.env.alloc_vars(&params);
	// Record assertion
	self.bytecodes.push(Bytecode::Assert);
	// Parse asserted expression
	self.parse_expr();
	// Done
	Ok(())
    }

    fn parse_decl_fun(&mut self) -> Result<(),()> {
        self.lexer.expect(TokenType::Fun);
        // Parse function name
        let id = self.lexer.expect(TokenType::Identifier);
        // Parse declared parameters
        let params = self.parse_decl_params()?;
        // Allocate params within environment
        self.env.alloc_vars(&params);
	// Construct function bytecode
	self.bytecodes.push(Bytecode::Fun(params.len()));
        // Parse function body
        let body = self.parse_block()?;
        // Done
        let name = self.lexer.to_string(&id);
        // Allocate function
        self.env.alloc_fn(&name, params.len());
	// Done
	Ok(())
    }

    fn parse_decl_recfun(&mut self) -> Result<(),()> {
        self.lexer.expect(TokenType::RecFun);
        // Parse function name
        let id = self.lexer.expect(TokenType::Identifier);
        // Parse declared parameters
        let params = self.parse_decl_params()?;
        // Allocate params within environment
        self.env.alloc_vars(&params);
        // Update environment
        let name = self.lexer.to_string(&id);
        // Allocate function
        self.env.alloc_fn(&name, params.len());
	// Construct function bytecode
	self.bytecodes.push(Bytecode::Fun(params.len()));
        // Parse function body
        let body = self.parse_block()?;
	// Done
	Ok(())
    }

    fn parse_decl_params(&mut self) -> Result<Vec<String>,()> {
        let mut params = Vec::new();
        self.lexer.expect(TokenType::LeftBrace);
        let mut lookahead = self.lexer.lookahead(0);
        //
        while lookahead.kind != TokenType::RightBrace {
            if !params.is_empty() { self.lexer.expect(TokenType::Comma); }
            let var = self.parse_identifier()?;
            params.push(var);
            lookahead = self.lexer.lookahead(0);
        }
        // Done
        self.lexer.expect(TokenType::RightBrace);
        Ok(params)
    }

    /// Parse a _block_, which is a term wrapped in curly braces
    /// (e.g. `{ [] }`).
    fn parse_block(&mut self) -> Result<(),()> {
        // Blocks begin with open curly brace
        self.lexer.expect(TokenType::LeftCurly);
        // Parse contents expression
        self.parse_expr()?;
        // Parse closing brace
        self.lexer.expect(TokenType::RightCurly);
	// Done
	Ok(())
    }

    // ===============================================================
    // Expressions
    // ===============================================================

    pub fn parse_expr(&mut self) -> Result<(),()> {
        self.parse_expr_binary(3)
    }

    /// Parse a binary expression at a given _level_.  Higher levels
    /// indicate expressions which bind _less tightly_.  Furthermore,
    /// level `0` corresponds simply to parsing a unary expression.
    fn parse_expr_binary(&mut self, level: usize) -> Result<(),()> {
        if level == 0 {
            self.parse_expr_postfix()
        } else {
            let tokens = BINARY_CONNECTIVES[level-1];
	    // Save insertion point for binary operator
	    let index = self.bytecodes.len();
            // Parse level below
	    self.parse_expr_binary(level-1)?;
            // Check whether binary connective follows
            match self.lexer.match_any(tokens) {
                Some(t) => {
		    self.bytecodes.insert(index,Self::binop_from_token(t.kind));
                    // FIXME: turn this into a loop!
	            let rhs = self.parse_expr_binary(level-1)?;
                }
                None => {}
            };
	    // Done
	    Ok(())
        }
    }

    fn parse_expr_braced(&mut self) -> Result<(),()> {
        self.lexer.expect(TokenType::LeftBrace);
	self.parse_expr()?;
        self.lexer.expect(TokenType::RightBrace);
	Ok(())
    }

    fn parse_expr_ifelse(&mut self) -> Result<(),()> {
        self.lexer.expect(TokenType::If);
        self.bytecodes.push(Bytecode::If);
        // Parse condition
	self.parse_expr()?;
        // Parse true branch
        self.parse_block()?;
        // Parse false branch (currently required)
        self.lexer.expect(TokenType::Else);
        self.parse_block()?;
        // Done
        Ok(())
    }

    fn parse_expr_invoke(&mut self) -> Result<(),()> {
	// Save bytecode for later
	let index = self.bytecodes.len();
	// Match function name
        let id = self.lexer.expect(TokenType::Identifier);
        let name = self.lexer.to_string(&id);
        let (id,arity) = self.env.lookup_fn(&name).unwrap();
	// Create invocation
	self.bytecodes.push(Bytecode::Invoke(0,0));
        // Parse left brace
        self.lexer.expect(TokenType::LeftBrace);
        // Parse terms within literal
        let n = self.parse_exprs_until(TokenType::RightBrace)?;
        // Match right brace
        self.lexer.expect(TokenType::RightBrace);
        // Should generate an error :)
        assert_eq!(n, arity);
        // Update invoke bytecode
	self.bytecodes[index] = Bytecode::Invoke(id,arity);
	// Done
	Ok(())
    }

    fn parse_expr_not(&mut self) -> Result<(),()> {
        self.lexer.expect(TokenType::Shreak);
        self.bytecodes.push(Bytecode::Not);
        self.parse_expr()?;
        Ok(())
    }

    fn parse_expr_postfix(&mut self) -> Result<(),()> {
        let index = self.bytecodes.len();
        // Parse the source term
        self.parse_expr_unit()?;
        //
        if self.lexer.matches(TokenType::Colon) {
            match self.lexer.match_any(TYPES) {
                Some(t) => {
		    self.bytecodes.insert(index,Self::type_from_token(t.kind));
                }
                _ => { return Err(()); }
            }
        }
        // Done
        Ok(())
    }

    fn parse_expr_unit(&mut self) -> Result<(),()> {
        let lookahead = self.lexer.lookahead(0);
        //
        match lookahead.kind {
            TokenType::BoolLiteral(v) => self.parse_literal_bool(v),
            TokenType::LeftBrace => self.parse_expr_braced(),
            TokenType::Identifier => {
                // Disambiguate static invocation from variable access
                if self.lexer.lookahead(1).kind == TokenType::LeftBrace {
                    self.parse_expr_invoke()
                } else {
                    self.parse_expr_varaccess()
                }
            }
            TokenType::IntLiteral => self.parse_literal_int(),
            TokenType::Shreak => self.parse_expr_not(),
            TokenType::If => self.parse_expr_ifelse(),
            _ => {
                panic!("unexpected token {lookahead:?}");
            }
        }
    }

    /// Parse a sequence of zero or more comma-separated terms until a
    /// given end token is encountered.
    fn parse_exprs_until(&mut self, end: TokenType) -> Result<usize,()> {
        let mut lookahead = self.lexer.lookahead(0);
	let mut n = 0;
        while lookahead.kind != end {
            if n != 0 {
                self.lexer.expect(TokenType::Comma);
            }
	    self.parse_expr()?;
            lookahead = self.lexer.lookahead(0);
	    n += 1
        }
        //
        Ok(n)
    }

    fn parse_expr_varaccess(&mut self) -> Result<(),()> {
        // Match variable name
        let id = self.lexer.expect(TokenType::Identifier);
        let name = self.lexer.to_string(&id);
        // Lookup variable
        let var = self.env.lookup_var(&name).unwrap();
        self.bytecodes.push(Bytecode::Var(var));
        Ok(())
    }

    // ===============================================================
    // Literals
    // ===============================================================

    fn parse_literal_bool(&mut self, val: bool) -> Result<(),()> {
        self.lexer.expect(TokenType::BoolLiteral(val));
	self.bytecodes.push(Bytecode::Bool(val));
        Ok(())
    }

    fn parse_literal_int(&mut self) -> Result<(),()> {
        let tok = self.lexer.expect(TokenType::IntLiteral);
        let s = self.lexer.to_string(&tok);
        let i = s.parse::<usize>().unwrap();
	self.bytecodes.push(Bytecode::Int(i));
        Ok(())
    }


    // ===============================================================
    // Misc
    // ===============================================================

    fn parse_identifier(&mut self) -> Result<String,()> {
        let ith = self.lexer.expect(TokenType::Identifier);
        Ok(self.lexer.to_string(&ith))
    }

    /// Construct a `BinOp` from a `TokenType`.
    fn binop_from_token(token: TokenType) -> Bytecode {
	let bop = match token {
            // Equality
            TokenType::EqualsEquals => Bytecode::Eq,
            TokenType::ShreakEquals => Bytecode::Neq,
            // Comparison
	    TokenType::LeftAngle => Bytecode::Lt,
            TokenType::LeftAngleEquals => Bytecode::LtEq,
            TokenType::RightAngle => Bytecode::Gt,
            TokenType::RightAngleEquals => Bytecode::GtEq,
            // Arithmetic
            TokenType::Minus => Bytecode::Sub,
	    TokenType::Percent => Bytecode::Rem,
	    TokenType::Plus => Bytecode::Add,
            TokenType::RightSlash => Bytecode::Div,
            TokenType::Star => Bytecode::Mul,
            // Logical
            TokenType::AmpersandAmpersand => Bytecode::And,
            TokenType::BarBar => Bytecode::Or,
            TokenType::LongRightArrow => Bytecode::Implies,
            // No match
	    _ => { unreachable!(); }
	};
	bop
    }

    /// Construct type test a `TokenType`.
    fn type_from_token(token: TokenType) -> Bytecode {
        match token {
            // Equality
            TokenType::Uint => Bytecode::IsUint,
            _ => { unreachable!(); }
        }
    }
}

// ===================================================================
// Typing Environment
// ===================================================================

struct Environment {
    /// Maps functions to their indices
    fn_bindings: HashMap<String,(usize,usize)>,
    /// Maps variables to their indices
    var_bindings: HashMap<String,usize>
}

impl Environment {
    pub fn new() -> Self {
        let fn_bindings = HashMap::new();
        let var_bindings = HashMap::new();
        Self{fn_bindings,var_bindings}
    }

    pub fn alloc_fn(&mut self, name: &str, arity: usize) {
        // Determine index of this function
        let index = self.fn_bindings.len();
        //
        self.fn_bindings.insert(name.to_string(),(index,arity));
    }

    pub fn alloc_vars(&mut self, vars: &[String]) {
        self.var_bindings.clear();
        //
        for (i,n) in vars.iter().enumerate() {
            self.var_bindings.insert(n.to_string(),i);
        }
    }

    pub fn lookup_fn(&self, name: &str) -> Option<(usize,usize)> {
	self.fn_bindings.get(name).copied()
    }

    pub fn lookup_var(&self, name: &str) -> Option<usize> {
	self.var_bindings.get(name).copied()
    }
}
