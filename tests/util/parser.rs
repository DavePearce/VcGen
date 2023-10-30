// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::fmt;
use super::lexer::{Lexer,Token};
use super::bytecode::Bytecode;

// ===================================================================
// Parse Error
// ===================================================================


/// Errors which can arise when parsing assembly language and/or
/// assembling it.
#[derive(Debug)]
pub enum ParseError {
    /// When parsing some assembly language, mnemonic was encountered
    /// that requires an operand (e.g. `push`) but none was found.
    ExpectedOperand,
    /// When parsing some assembly language, an invalid comment was
    /// encountered.
    InvalidComment(usize),
    /// When parsing some assembly language, an invalid hex literal
    /// was encountered.
    InvalidHexString(usize),
    /// When parsing some assembly language, an unexpected mnemonic
    /// was encountered.
    InvalidInstruction,
    /// When parsing some assembly language, an unexpected character
    /// was encountered.
    UnexpectedCharacter(usize),
    /// When parsing some assembly language, an unexpected token was
    /// encountered.
    UnexpectedToken,
    /// When assembling a given assembly, a labelled instruction was
    /// encountered that targets a non-existent label.
    UnknownLabel(String),
    /// When assembling a given assembly, a duplicate label was
    /// encountered.
    DuplicateLabel(String),    
    /// When assembling a given assembly, the distance of a calculated
    /// relative offset was found to exceed 16bits.
    InvalidRelativeOffset,
    /// When assembling a given assembly, the distance of a calculated
    /// offset exceeds the maximum permitted code size.
    OffsetTooLarge
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {

}

// ===================================================================
// Parser
// ===================================================================

/// A simple assembly language parser.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    bytecodes: Vec<Bytecode>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Parser<'a> {
        let lexer = Lexer::new(input);
        let bytecodes = Vec::new();
        //
        Self{lexer,bytecodes}
    }

    /// Parse assembly language to form an assembly    
    pub fn parse(mut self) -> Result<Vec<Bytecode>,ParseError> {
        // Keep going until we reach the end.
        while self.lexer.lookahead()? != Token::EOF {
            self.parse_section()?;
        }
        Ok(self.bytecodes)
    }

    fn parse_section(&mut self) -> Result<(),ParseError> {
        loop {
            match self.lexer.lookahead()? {
                Token::Identifier(opcode) => {
                    _ = self.lexer.next();                    
                    self.bytecodes.push(self.parse_insn(opcode)?);
                }
                Token::EOF => {
                    return Ok(());
                }
                _ => {
                    // Something went wrong
                    return Err(ParseError::UnexpectedToken);
                }
            };
        }
    }

    fn parse_insn(&self, insn: &str) -> Result<Bytecode,ParseError> {
        let insn = match insn {
            //
            "lt"|"LT" => Bytecode::Lt,
            "gt"|"GT" => Bytecode::Gt,
            "lteq"|"LTEQ" => Bytecode::LtEq,
            "gteq"|"GTEQ" => Bytecode::GtEq,
            "eq"|"EQ" => Bytecode::Eq,
            //        
            "add"|"ADD" => Bytecode::Add,
            "mul"|"MUL" => Bytecode::Mul,
            "sub"|"SUB" => Bytecode::Sub,
            "div"|"DIV" => Bytecode::Div,
            "rem"|"REM" => Bytecode::Rem,
            //
            _ => {
                return Err(ParseError::InvalidInstruction);
            }
        };
        //
        Ok(insn)
    }    
}
