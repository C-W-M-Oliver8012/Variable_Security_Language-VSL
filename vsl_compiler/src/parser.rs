use std::fs;
use std::io::Write;
use std::collections::HashMap;

#[path = "lexer.rs"]
pub mod lexer;

#[path = "standard_library.rs"]
pub mod standard_library;

const POP: i64 = 1;
const LOCAL_LOAD: i64 = 2;
const LOCAL_STORE: i64 = 3;

const I_CONSTANT: i64 = 4;
const I_ADD: i64 = 5;
const I_SUB: i64 = 6;
const I_MUL: i64 = 7;
const I_DIV: i64 = 8;
const I_EQUAL: i64 = 9;
const I_LESS: i64 = 10;
const I_GREATER: i64 = 11;
const I_NOT_EQUAL: i64 = 12;
const I_LESS_EQUAL: i64 = 13;
const I_GREATER_EQUAL: i64 = 14;

const F_CONSTANT: i64 = 15;
const F_ADD: i64 = 16;
const F_SUB: i64 = 17;
const F_MUL: i64 = 18;
const F_DIV: i64 = 19;
const F_EQUAL: i64 = 20;
const F_LESS: i64 = 21;
const F_GREATER: i64 = 22;
const F_NOT_EQUAL: i64 = 23;
const F_LESS_EQUAL: i64 = 24;
const F_GREATER_EQUAL: i64 = 25;

const S_CONSTANT: i64 = 26;
const S_ADD: i64 = 27;
const S_EQUAL: i64 = 28;
const S_NOT_EQUAL: i64 = 29;

const OP_AND: i64 = 30;
const OP_OR: i64 = 31;

const JUMP_IF_FALSE: i64 = 32;
const JUMP: i64 = 33;

const CALL: i64 = 34;
const RETURN_VAL: i64 = 35;
const RETURN_NON_VAL: i64 = 36;
const ARG_LOAD: i64 = 37;
const ARG_STORE: i64 = 38;

const USE: i64 = 39;

const HALT: i64 = 40;

//----------------------------------------------------------------------------------

const SEMI_COLON: u8 = 0;
const LEFT_PARENTHESIS: u8 = 1;
const RIGHT_PARENTHESIS: u8 = 2;
const LEFT_CURLEY: u8 = 3;
const RIGHT_CURLEY: u8 = 4;
const EQUAL: u8 = 5;
const COLON: u8 = 6;
const COMMA: u8 = 7;

const ADD: u8 = 10;
const MUL: u8 = 11;
const DIV: u8 = 12;
const SUB: u8 = 13;

//const MOD: u8 = 14;
const LESS: u8 = 15;
const GREATER: u8 = 16;
const EQUAL_EQUAL: u8 = 17;
const NOT_EQUAL: u8 = 18;
const LESS_EQUAL: u8 = 19;
const GREATER_EQUAL: u8 = 20;

const INT_TYPE: u8 = 21;
const FLOAT_TYPE: u8 = 22;
const STRING_TYPE: u8 = 23;
const FN: u8 = 24;
const LET: u8 = 25;
const AND: u8 = 26;
const OR: u8 = 27;
const WHILE: u8 = 28;
const RETURN: u8 = 29;
const IF: u8 = 30;
const ELSE: u8 = 31;

const VOID: u8 = 32;
const BREAK: u8 = 33;
const VEC_INT: u8 = 34;
const VEC_FLOAT: u8 = 35;
const VEC_STRING: u8 = 36;

const IDENTIFIER: u8 = 37;
const INT: u8 = 38;
const FLOAT: u8 = 39;
const STRING: u8 = 40;

//----------------------------------------------------------------------------------

struct Variable {
    pub mem_location: i64,
    pub var_type: u8,
    pub security: i64,
    pub is_arg: bool,
    pub arg_location: i64,
    pub scope: i64,
    pub function_name: String,
}

struct Function {
    pub mem_location: i64,
    pub fn_type: u8,
    pub security: i64,
    pub num_args: i64,
    pub arg_types: Vec<u8>,
    pub arg_securities: Vec<i64>,
    pub times_parsed: i64,
}

pub struct Parser {
    error: bool,
    num_tokens: usize,
    current_token_num: usize,
    code: Vec<i64>,
    var_data: HashMap<String, Variable>,
    stack_size: i64,
    fn_data: HashMap<String, Function>,
    current_fn_name: String,
    return_num: i64,
    current_scope: i64,
    sl_data: HashMap<String, standard_library::SlData>,
    loop_nest_num: i64,
}

impl Parser {
    pub fn new(tokens: &Vec<lexer::Token>) -> Parser {
        let parser = Parser {
            error: false,
            num_tokens: tokens.len(),
            current_token_num: 0,
            code: Vec::new(),
            var_data: HashMap::new(),
            stack_size: 0,
            fn_data: HashMap::new(),
            current_fn_name: String::new(),
            return_num: 0,
            current_scope: -1,
            sl_data: standard_library::get_sl_data(),
            loop_nest_num: 0,
        };
        parser
    }

    fn is_last_token(&mut self) -> bool {
        if self.current_token_num == self.num_tokens - 1 {
            return true;
        }
        false
    }

    fn consume_token(&mut self) {
        if self.is_last_token() == false {
            self.current_token_num += 1;
        }
    }

    pub fn output_code(&mut self) {
        let mut binary_data: Vec<u8> = Vec::new();
    
        for chunk in self.code.to_vec() {
            let bytes = chunk.to_be_bytes();
            binary_data.push(bytes[0]);
            binary_data.push(bytes[1]);
            binary_data.push(bytes[2]);
            binary_data.push(bytes[3]);
            binary_data.push(bytes[4]);
            binary_data.push(bytes[5]);
            binary_data.push(bytes[6]);
            binary_data.push(bytes[7]);
        }
        
        let mut file = fs::File::create("program").expect("Failed to create file test.");
        file.write_all(&binary_data).expect("Failed to write to binary file");
    }

    fn index_functions(&mut self, tokens: &Vec<lexer::Token>) {
        let mut instruction_counter: i64 = 3;
        
        while self.is_last_token() == false {
            match tokens[self.current_token_num].token_num {
                FN => {
                    let mut fn_type: u8 = INT;
                    let identifier: String;
                    let mut security: i64 = 0;
                    let mem_location: i64 = instruction_counter;
                    let mut num_args: i64 = 0;
                    let mut arg_types = Vec::new();
                    let mut arg_securities = Vec::new();

                    // fn
                    self.consume_token();

                    if tokens[self.current_token_num].token_num != VOID {
                        if tokens[self.current_token_num].token_num == INT_TYPE {
                            fn_type = INT;
                        } else if tokens[self.current_token_num].token_num == FLOAT_TYPE {
                            fn_type = FLOAT;
                        } else if tokens[self.current_token_num].token_num == STRING_TYPE {
                            fn_type = STRING;
                        }
                        // type
                        self.consume_token();
                        // colon
                        self.consume_token();
                        if tokens[self.current_token_num].token_num == INT {
                            security = tokens[self.current_token_num].token_string.parse::<i64>().expect("Failed to parse integer.");
                        }
                        // security
                        self.consume_token();
                    } else {
                        fn_type = VOID;
                        // type
                        self.consume_token();
                    }
                    identifier = tokens[self.current_token_num].token_string.clone();
                    if self.sl_data.contains_key(&identifier) {
                        println!("Function name '{}' cannot be the same as a standard library function on line {}.", identifier, tokens[self.current_token_num].line_num);
                        self.error = true;
                    }
                    // identifier
                    self.consume_token();
                    // left parenthesis
                    self.consume_token();

                    if tokens[self.current_token_num].token_num != RIGHT_PARENTHESIS {
                        loop {
                            let mut var_type = INT;
                            let mut var_security: i64 = 0;
                            num_args += 1;
    
                            // identifier
                            self.consume_token();
                            if tokens[self.current_token_num].token_num == INT_TYPE {
                                var_type = INT;
                            } else if tokens[self.current_token_num].token_num == FLOAT_TYPE {
                                var_type = FLOAT;
                            } else if tokens[self.current_token_num].token_num == STRING_TYPE {
                                var_type = STRING;
                            } else if tokens[self.current_token_num].token_num == VEC_INT {
                                var_type = VEC_INT;
                            } else if tokens[self.current_token_num].token_num == VEC_FLOAT {
                                var_type = VEC_FLOAT;
                            } else if tokens[self.current_token_num].token_num == VEC_STRING {
                                var_type = VEC_STRING;
                            }
                            arg_types.push(var_type);
                            // var type
                            self.consume_token();
                            // colon
                            self.consume_token();
                            if tokens[self.current_token_num].token_num == INT {
                                var_security = tokens[self.current_token_num].token_string.parse::<i64>().expect("Failed to parse integer.");
                            }
                            arg_securities.push(var_security);
                            // security
                            self.consume_token();
    
                            if tokens[self.current_token_num].token_num != COMMA {
                                break;
                            } else {
                                self.consume_token();
                            }
                        }
                    }

                    // right parenthesis
                    self.consume_token();

                    let new_fn = Function {
                        mem_location: mem_location,
                        fn_type: fn_type,
                        security: security,
                        num_args: num_args,
                        arg_types: arg_types,
                        arg_securities: arg_securities,
                        times_parsed: 0,
                    };

                    self.fn_data.insert(identifier, new_fn);
                },
                INT => {
                    // check to make sure not variable security
                    if self.current_token_num != 0 {
                        if tokens[self.current_token_num - 1].token_num != COLON {
                            instruction_counter += 2;
                            self.consume_token();
                        } else {
                            self.consume_token();
                        }
                    } else {
                        self.consume_token();
                    }
                },
                FLOAT => {
                    instruction_counter += 2;
                    self.consume_token();
                },
                STRING => {
                    // S_CONSTANT
                    instruction_counter += 1;
                    for c in tokens[self.current_token_num].token_string.chars() {
                        // Every character
                        if c != '"' && c != '\\' {
                            instruction_counter += 1;
                        }
                    }
                    // null terminator
                    instruction_counter += 1;
                    self.consume_token();
                },
                LET => {
                    self.consume_token();
                    if self.current_token_num + 1 < self.num_tokens {
                        if tokens[self.current_token_num + 1].token_num == INT_TYPE
                            || tokens[self.current_token_num + 1].token_num == FLOAT_TYPE 
                            || tokens[self.current_token_num + 1].token_num == STRING_TYPE 
                            || tokens[self.current_token_num + 1].token_num == VEC_INT
                            || tokens[self.current_token_num + 1].token_num == VEC_FLOAT
                            || tokens[self.current_token_num + 1].token_num == VEC_STRING {
                                instruction_counter += 2;
                            }
                    }
                },
                IDENTIFIER => {
                    self.consume_token();
                    if tokens[self.current_token_num].token_num != LEFT_PARENTHESIS {
                        instruction_counter += 2;
                    } else if tokens[self.current_token_num].token_num == LEFT_PARENTHESIS {
                        if self.sl_data.contains_key(&tokens[self.current_token_num - 1].token_string) {
                            if tokens[self.current_token_num - 1].token_string == "print".to_string() {
                                instruction_counter += 3;
                                let mut tmp_current_token_num = self.current_token_num;
                                while tmp_current_token_num < self.num_tokens && tokens[tmp_current_token_num].token_num != SEMI_COLON {
                                    if tokens[tmp_current_token_num].token_num == IDENTIFIER {
                                        tmp_current_token_num += 1;
                                        if tmp_current_token_num < self.num_tokens && tokens[tmp_current_token_num].token_num == LEFT_PARENTHESIS {
                                            let mut num_parenthesis: i64 = 1;
                                            tmp_current_token_num += 1;
                                            while num_parenthesis != 0 && tmp_current_token_num < self.num_tokens {
                                                if tokens[tmp_current_token_num].token_num == LEFT_PARENTHESIS {
                                                    num_parenthesis += 1;
                                                    tmp_current_token_num += 1;
                                                } else if tokens[tmp_current_token_num].token_num == RIGHT_PARENTHESIS {
                                                    num_parenthesis -= 1;
                                                    tmp_current_token_num += 1;
                                                } else {
                                                    tmp_current_token_num += 1;
                                                }
                                            }
                                        }
                                    }
                                    if tmp_current_token_num < self.num_tokens && tokens[tmp_current_token_num].token_num == COMMA {
                                        instruction_counter += 3;
                                    }
                                    tmp_current_token_num += 1;
                                }
                            } else if tokens[self.current_token_num - 1].token_string == "read".to_string() {
                                instruction_counter += 3;
                            } else {
                                instruction_counter += 2;
                            }
                        } else {
                            instruction_counter += 3;
                        }
                    }
                },
                ADD => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                SUB => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                DIV => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                MUL => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                EQUAL_EQUAL => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                NOT_EQUAL => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                LESS => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                LESS_EQUAL => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                GREATER => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                GREATER_EQUAL => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                AND => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                OR => {
                    instruction_counter += 1;
                    self.consume_token();
                },
                IF => {
                    instruction_counter += 4;
                    self.consume_token();
                },
                WHILE => {
                    instruction_counter += 4;
                    self.consume_token();
                }
                RETURN => {
                    instruction_counter += 1;
                    self.consume_token();
                }
                _ => {
                    self.consume_token();
                },
            }
        }
    }

    pub fn parse(&mut self, tokens: &Vec<lexer::Token>) -> bool {
        self.index_functions(tokens);
        match self.fn_data.get("main") {
            Some(x) => {
                self.code.push(CALL);
                self.code.push(x.mem_location);
                // main always has 0 arguments
                self.code.push(0);
            },
            None => {
                println!("No main function found in program.");
                self.error = true;
            }
        }
        self.current_token_num = 0;
        while self.is_last_token() == false {
            self.fn_dec(tokens);
        }
        self.error
    }












    // Parsing code
    //--------------------------------------------------------------------------------------------------------------------------

    fn fn_dec(&mut self, tokens: &Vec<lexer::Token>) {
        self.stack_size = 0;
        self.fn_keyword(tokens);
        let fn_type: u8;
        if tokens[self.current_token_num].token_num != VOID {
            fn_type = tokens[self.current_token_num].token_num;
            self.fn_type(tokens);
            self.colon(tokens);
            self.integer(tokens);
        } else {
            fn_type = tokens[self.current_token_num].token_num;
            self.fn_type(tokens);
        }
        self.current_fn_name = tokens[self.current_token_num].token_string.clone();
        if self.current_fn_name == "main" {
            if fn_type != VOID {
                println!("Function 'main' must be of type void on line {}.", tokens[self.current_token_num].line_num);
                self.error = true;
            }
        }
        let identifier = tokens[self.current_token_num].token_string.clone();
        match self.fn_data.get_mut(&identifier) {
            Some(x) => {
                x.times_parsed += 1;
                if x.times_parsed > 1 {
                    println!("Function with name '{}' already exists on line {}.", identifier, tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            },
            None => {},
        }
        self.identifier(tokens);
        if self.current_fn_name == "main" {
            self.left_parenthesis(tokens);
            self.right_parenthesis(tokens);
        } else {
            self.left_parenthesis(tokens);
            if tokens[self.current_token_num].token_num != RIGHT_PARENTHESIS {
                let mut arg_location: i64 = 0;
                if tokens[self.current_token_num].token_num != RIGHT_PARENTHESIS {
                    loop {
                        let var_name = tokens[self.current_token_num].token_string.clone();
                        let mut var_type = INT;
                        let mut var_security: i64 = 0;

                        // identifier
                        self.identifier(tokens);
                        if tokens[self.current_token_num].token_num == INT_TYPE {
                            var_type = INT;
                        } else if tokens[self.current_token_num].token_num == FLOAT_TYPE {
                            var_type = FLOAT;
                        } else if tokens[self.current_token_num].token_num == STRING_TYPE {
                            var_type = STRING;
                        } else if tokens[self.current_token_num].token_num == VEC_INT {
                            var_type = VEC_INT;
                        } else if tokens[self.current_token_num].token_num == VEC_FLOAT {
                            var_type = VEC_FLOAT;
                        } else if tokens[self.current_token_num].token_num == VEC_STRING {
                            var_type = VEC_STRING;
                        }
                        // var type
                        self.var_type(tokens);
                        // colon
                        self.colon(tokens);
                        if tokens[self.current_token_num].token_num == INT {
                            var_security = tokens[self.current_token_num].token_string.parse::<i64>().expect("Failed to parse integer.");
                        }
                        // security
                        self.integer(tokens);

                        let variable = Variable {
                            mem_location: 0,
                            var_type: var_type,
                            security: var_security, 
                            is_arg: true,
                            arg_location: arg_location,
                            scope: 0,
                            function_name: identifier.clone(),
                        };

                        arg_location += 1;
                        self.var_data.insert(var_name, variable);

                        if tokens[self.current_token_num].token_num != COMMA {
                            break;
                        } else {
                            self.comma(tokens);
                        }
                    }
                }
            }
            self.right_parenthesis(tokens);
        }
        self.return_num = 0;
        let mut locals: Vec<i64> = Vec::new();
        let mut breaks: Vec<i64> = Vec::new();
        self.block(tokens, &mut locals, &mut breaks);
        if self.return_num == 0 {
            println!("Function '{}' has no return statement.", self.current_fn_name);
            self.error = true;
        }
    }

    fn block(&mut self, tokens: &Vec<lexer::Token>, locals: &mut Vec<i64>, breaks: &mut Vec<i64>) {
        self.left_curley(tokens);
        self.current_scope += 1;
        while self.is_last_token() == false && tokens[self.current_token_num].token_num != RIGHT_CURLEY {
            match tokens[self.current_token_num].token_num {
                LET => {
                    self.var_dec(tokens, locals);
                    self.semi_colon(tokens);
                },
                IDENTIFIER => {
                    if self.current_token_num + 1 < tokens.len() {
                        if tokens[self.current_token_num + 1].token_num != LEFT_PARENTHESIS {
                            self.var_def(tokens);
                            self.semi_colon(tokens);
                        } else if tokens[self.current_token_num + 1].token_num == LEFT_PARENTHESIS {
                            match self.sl_data.get(&tokens[self.current_token_num].token_string) {
                                Some(x) => {
                                    if x.return_type != standard_library::VOID {
                                        println!("Function not within an expression must be void on line {}.", tokens[self.current_token_num].line_num);
                                        self.error = true;
                                    }
                                    self.sl_use(tokens, standard_library::VOID);
                                    self.semi_colon(tokens);
                                },
                                None => {
                                    self.fn_call(tokens);
                                    self.semi_colon(tokens);
                                },
                            }
                        }
                    } else {
                        println!("Program suddenly ended on line {}.", tokens[self.current_token_num].line_num);
                        self.error = true;
                    }
                },
                IF => {
                    self.if_statement(tokens, locals, breaks);
                },
                WHILE => {
                    self.while_statement(tokens, locals);
                },
                BREAK => {
                    if self.loop_nest_num == 0 {
                        println!("Cannot break out of a non loop on line {}.", tokens[self.current_token_num].line_num);
                        self.error = true;
                    }
                    self.break_statement(tokens, breaks);
                    self.semi_colon(tokens);
                },
                RETURN => {
                    self.return_statement(tokens, locals);
                    self.semi_colon(tokens);
                },
                _ => {
                    println!("Beginning of unkown statement type on line {}.", tokens[self.current_token_num].line_num);
                    self.consume_token();
                    self.error = true;
                },
            }
        }
        self.right_curley(tokens);
        let mut keys: Vec<String> = Vec::new();
        for (key, val) in self.var_data.iter() {
            if val.function_name == self.current_fn_name && val.scope == self.current_scope {
                keys.push(key.clone());
            }
        }
        for key in keys {
            self.var_data.remove(&key);
        }
        self.current_scope -= 1;
    }

    fn left_curley(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != LEFT_CURLEY {
            println!("Expected '{}', got '{}' on line {}.", 123 as char, tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn right_curley(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != RIGHT_CURLEY {
            println!("Expected '{}', got '{}' on line {}.", 125 as char, tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn fn_keyword(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != FN {
            println!("Expected 'fn', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn fn_type(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != INT_TYPE &&
            tokens[self.current_token_num].token_num != FLOAT_TYPE && 
            tokens[self.current_token_num].token_num != STRING_TYPE &&
            tokens[self.current_token_num].token_num != STRING_TYPE &&
            tokens[self.current_token_num].token_num != VEC_INT && 
            tokens[self.current_token_num].token_num != VEC_FLOAT && 
            tokens[self.current_token_num].token_num != VEC_STRING &&
            tokens[self.current_token_num].token_num != VOID {
                println!("Expected type, got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
                self.error = true;
                self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn semi_colon(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != SEMI_COLON {
            println!("Expected ';', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
        } else {
            self.consume_token();
        }
    }

    fn let_keyword(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != LET {
            println!("Expected 'let', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn identifier(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != IDENTIFIER {
            println!("Expected identifier, got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn var_type(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != INT_TYPE &&
            tokens[self.current_token_num].token_num != FLOAT_TYPE && 
            tokens[self.current_token_num].token_num != STRING_TYPE && 
            tokens[self.current_token_num].token_num != VEC_INT && 
            tokens[self.current_token_num].token_num != VEC_FLOAT && 
            tokens[self.current_token_num].token_num != VEC_STRING {
                println!("Expected type, got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
                self.error = true;
                self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn colon(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != COLON {
            println!("Expected ':', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn integer(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != INT {
            println!("Expected integer, got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn equal(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != EQUAL {
            println!("Expected '=', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn left_parenthesis(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != LEFT_PARENTHESIS {
            println!("Expected '(' got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn right_parenthesis(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != RIGHT_PARENTHESIS {
            println!("Expected ')' got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn comma(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != COMMA {
            println!("Expected ',' got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn return_keyword(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != RETURN {
            println!("Expected 'return', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn if_keyword(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != IF {
            println!("Expected 'if', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn else_keyword(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != ELSE {
            println!("Expected 'else', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn while_keyword(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != WHILE {
            println!("Expected 'while', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

    fn break_keyword(&mut self, tokens: &Vec<lexer::Token>) {
        if tokens[self.current_token_num].token_num != BREAK {
            println!("Expected 'break', got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
        } else {
            self.consume_token();
        }
    }

















    // Expression Code
    // ---------------------------------------------------------------------------------------------------------------------------------------------------------------------

    fn drain_expression_stack(&mut self, tokens: &Vec<lexer::Token>, expression_type: u8, expression_stack: &mut Vec<u8>) {
        let or = 1;
            let and = 2;
            let equal_equal = 3;
            let not_equal = 4;
            let less = 5;
            let greater = 6;
            let less_equal = 7;
            let greater_equal = 8;
            let add = 9;
            let sub = 10;
            let mul = 11;
            let div = 12;
        while expression_stack.is_empty() == false {
            if expression_stack[expression_stack.len() - 1] == div {
                if expression_type == INT {
                    self.code.push(I_DIV);
                } else if expression_type == FLOAT {
                    self.code.push(F_DIV);
                } else {
                    println!("Type does not support division on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == mul {
                if expression_type == INT {
                    self.code.push(I_MUL);
                } else if expression_type == FLOAT {
                    self.code.push(F_MUL);
                } else {
                    println!("Type does not support multiplication on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == sub {
                if expression_type == INT {
                    self.code.push(I_SUB);
                } else if expression_type == FLOAT {
                    self.code.push(F_SUB);
                } else {
                    println!("Type does not support subtraction on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == add {
                if expression_type == INT {
                    self.code.push(I_ADD);
                } else if expression_type == FLOAT {
                    self.code.push(F_ADD);
                } else if expression_type == STRING {
                    self.code.push(S_ADD);
                } else {
                    println!("Type does not support addition on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == greater_equal {
                if expression_type == INT {
                    self.code.push(I_GREATER_EQUAL);
                } else if expression_type == FLOAT {
                    self.code.push(F_GREATER_EQUAL);
                } else {
                    println!("Type does not support greater than or equal comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == less_equal {
                if expression_type == INT {
                    self.code.push(I_LESS_EQUAL);
                } else if expression_type == FLOAT {
                    self.code.push(F_LESS_EQUAL);
                } else {
                    println!("Type does not support less than or equal comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == greater {
                if expression_type == INT {
                    self.code.push(I_GREATER);
                } else if expression_type == FLOAT {
                    self.code.push(F_GREATER);
                } else {
                    println!("Type does not support greater than comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == less {
                if expression_type == INT {
                    self.code.push(I_LESS);
                } else if expression_type == FLOAT {
                    self.code.push(F_LESS);
                } else {
                    println!("Type does not support less than comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == equal_equal {
                if expression_type == INT {
                    self.code.push(I_EQUAL);
                } else if expression_type == FLOAT {
                    self.code.push(F_EQUAL);
                } else if expression_type == STRING {
                    self.code.push(S_EQUAL);
                } else {
                    println!("Type does not support equal comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == not_equal {
                if expression_type == INT {
                    self.code.push(I_NOT_EQUAL);
                } else if expression_type == FLOAT {
                    self.code.push(F_NOT_EQUAL);
                } else if expression_type == STRING {
                    self.code.push(S_NOT_EQUAL);
                } else {
                    println!("Type does not support not equal comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            } else if expression_stack[expression_stack.len() - 1] == and {
                self.code.push(OP_AND);
            } else if expression_stack[expression_stack.len() - 1] == or {
                self.code.push(OP_OR);
            }
            expression_stack.pop();
        }
    }

    fn expression(&mut self, tokens: &Vec<lexer::Token>, expression_type: u8, variable_security: i64) {
        let mut expression_stack: Vec<u8> = Vec::new();
        self.term(tokens, &mut expression_stack, expression_type, variable_security);
        self.drain_expression_stack(tokens, expression_type, &mut expression_stack);
    }

    fn get_precedence(&mut self, current_token_num: u8) -> u8 {
        let or = 1;
        let and = 2;
        let equal_equal = 3;
        let not_equal = 4;
        let less = 5;
        let greater = 6;
        let less_equal = 7;
        let greater_equal = 8;
        let add = 9;
        let sub = 10;
        let mul = 11;
        let div = 12;
        let precedence: u8;
        if current_token_num == or {
            precedence = 1;
        } else if current_token_num == and {
            precedence = 2;
        } else if current_token_num == equal_equal {
            precedence = 3;
        } else if current_token_num == not_equal {
            precedence = 3;
        } else if current_token_num == less {
            precedence = 4;
        } else if current_token_num == greater {
            precedence = 4;
        } else if current_token_num == less_equal {
            precedence = 4;
        } else if current_token_num == greater_equal {
            precedence = 4;
        } else if current_token_num == add {
            precedence = 5;
        } else if current_token_num == sub {
            precedence = 5;
        } else if current_token_num == mul {
            precedence = 6;
        } else if current_token_num == div {
            precedence = 6;
        } else {
            precedence = 0;
        }
        precedence
    }

    fn shunting_yard(&mut self, tokens: &Vec<lexer::Token>, expression_stack: &mut Vec<u8>, expression_type: u8, value: u8) {
        let or = 1;
        let and = 2;
        let equal_equal = 3;
        let not_equal = 4;
        let less = 5;
        let greater = 6;
        let less_equal = 7;
        let greater_equal = 8;
        let add = 9;
        let sub = 10;
        let mul = 11;
        let div = 12;
        while expression_stack.is_empty() == false && self.get_precedence(value) <= self.get_precedence(expression_stack[expression_stack.len() - 1]) {
            if expression_stack[expression_stack.len() - 1] == div {
                if expression_type == INT {
                    self.code.push(I_DIV);
                } else if expression_type == FLOAT {
                    self.code.push(F_DIV);
                } else {
                    println!("Type does not support division on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == mul {
                if expression_type == INT {
                    self.code.push(I_MUL);
                } else if expression_type == FLOAT {
                    self.code.push(F_MUL);
                } else {
                    println!("Type does not support multiplication on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == sub {
                if expression_type == INT {
                    self.code.push(I_SUB);
                } else if expression_type == FLOAT {
                    self.code.push(F_SUB);
                } else {
                    println!("Type does not support subtraction on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == add {
                if expression_type == INT {
                    self.code.push(I_ADD);
                } else if expression_type == FLOAT {
                    self.code.push(F_ADD);
                } else if expression_type == STRING {
                    self.code.push(S_ADD);
                } else {
                    println!("Type does not support addition on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == greater_equal {
                if expression_type == INT {
                    self.code.push(I_GREATER_EQUAL);
                } else if expression_type == FLOAT {
                    self.code.push(F_GREATER_EQUAL);
                } else {
                    println!("Type does not support greater than or equal comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == less_equal {
                if expression_type == INT {
                    self.code.push(I_LESS_EQUAL);
                } else if expression_type == FLOAT {
                    self.code.push(F_LESS_EQUAL);
                } else if expression_type == STRING {
                    println!("Type does not support less than or equal comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == greater {
                if expression_type == INT {
                    self.code.push(I_GREATER);
                } else if expression_type == FLOAT {
                    self.code.push(F_GREATER);
                } else if expression_type == STRING {
                    println!("Type does not support greater than comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == less {
                if expression_type == INT {
                    self.code.push(I_LESS);
                } else if expression_type == FLOAT {
                    self.code.push(F_LESS);
                } else if expression_type == STRING {
                    println!("Type does not support less than comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == equal_equal {
                if expression_type == INT {
                    self.code.push(I_EQUAL);
                } else if expression_type == FLOAT {
                    self.code.push(F_EQUAL);
                } else if expression_type == STRING {
                    self.code.push(S_EQUAL);
                } else {
                    println!("Type does not support equal comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == not_equal {
                if expression_type == INT {
                    self.code.push(I_NOT_EQUAL);
                } else if expression_type == FLOAT {
                    self.code.push(F_NOT_EQUAL);
                } else if expression_type == STRING {
                    self.code.push(S_NOT_EQUAL);
                } else {
                    println!("Type does not support not equal comparison on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == and {
                self.code.push(OP_AND);
                expression_stack.pop();
            } else if expression_stack[expression_stack.len() - 1] == or {
                self.code.push(OP_OR);
                expression_stack.pop();
            }
        }
        expression_stack.push(value);
    }

    fn change_expression_type(&mut self, tokens: &Vec<lexer::Token>) -> u8 {
        let mut expression_type: u8 = INT;
        let mut current_token_num = self.current_token_num;
        while tokens[current_token_num].token_num == LEFT_PARENTHESIS && current_token_num < tokens.len() {
            current_token_num += 1;
        }
        if tokens[current_token_num].token_num == INT {
            expression_type = INT;
        } else if tokens[current_token_num].token_num == FLOAT {
            expression_type = FLOAT;
        } else if tokens[current_token_num].token_num == IDENTIFIER {
            if current_token_num + 1 < tokens.len() {
                // identifier is a function
                if tokens[current_token_num + 1].token_num == LEFT_PARENTHESIS {
                    match self.sl_data.get(&tokens[self.current_token_num].token_string) {
                        Some(x) => {
                            if x.return_type == standard_library::INT {
                                expression_type = INT;
                            } else if x.return_type == standard_library::FLOAT {
                                expression_type = FLOAT;
                            } else if x.return_type == standard_library::STRING {
                                expression_type = STRING;
                            }
                        },
                        None => {},
                    }
                    match self.fn_data.get(&tokens[current_token_num].token_string) {
                        Some(x) => {
                            expression_type = x.fn_type;
                        },
                        None => {},
                    }
                    // identifier is a variable
                } else if tokens[current_token_num + 1].token_num != LEFT_PARENTHESIS {
                    match self.var_data.get(&tokens[current_token_num].token_string) {
                        Some(x) => {
                            expression_type = x.var_type;
                        },
                        None => {},
                    }
                }
            }
        }
        expression_type
    }

    fn term(&mut self, tokens: &Vec<lexer::Token>, expression_stack: &mut Vec<u8>, mut expression_type: u8, variable_security: i64) {
        let or = 1;
        let and = 2;
        let equal_equal = 3;
        let not_equal = 4;
        let less = 5;
        let greater = 6;
        let less_equal = 7;
        let greater_equal = 8;
        let add = 9;
        let sub = 10;
        let mul = 11;
        let div = 12;
        self.literal(tokens, expression_type, variable_security);
        loop {
            match tokens[self.current_token_num].token_num {
                DIV => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(div);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, div);
                    }
                },
                MUL => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(mul);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, mul);
                    }
                },
                SUB => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(sub);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, sub);
                    }
                },
                ADD => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(add);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, add);
                    }
                },
                GREATER_EQUAL => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(greater_equal);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, greater_equal);
                    }
                },
                LESS_EQUAL => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(less_equal);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, less_equal);
                    }
                },
                GREATER => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(greater);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, greater);
                    }
                },
                LESS => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(less);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, less);
                    }
                },
                NOT_EQUAL => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(not_equal);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, not_equal);
                    }
                },
                EQUAL_EQUAL => {
                    self.consume_token();
                    if expression_stack.is_empty() == true {
                        expression_stack.push(equal_equal);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, equal_equal);
                    }
                },
                AND => {
                    self.consume_token();
                    self.drain_expression_stack(tokens, expression_type, expression_stack);
                    if expression_stack.is_empty() == true {
                        expression_stack.push(and);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, and);
                    }
                    expression_type = self.change_expression_type(tokens);
                },
                OR => {
                    self.consume_token();
                    self.drain_expression_stack(tokens, expression_type, expression_stack);
                    if expression_stack.is_empty() == true {
                        expression_stack.push(or);
                    } else {
                        self.shunting_yard(tokens, expression_stack, expression_type, or);
                    }
                    expression_type = self.change_expression_type(tokens);
                },
                _ => break,
            }
            self.literal(tokens, expression_type, variable_security);
        }
    }

    fn literal(&mut self, tokens: &Vec<lexer::Token>, expression_type: u8, variable_security: i64) {
        if tokens[self.current_token_num].token_num == INT && expression_type == INT {
            self.code.push(I_CONSTANT);
            self.code.push(tokens[self.current_token_num].token_string.parse::<i64>().expect("Failed to parse integer."));
            self.consume_token();
            return;
        } else if tokens[self.current_token_num].token_num == FLOAT && expression_type == FLOAT {
            self.code.push(F_CONSTANT);
            self.code.push(i64::from_be_bytes(f64::to_be_bytes(tokens[self.current_token_num].token_string.parse::<f64>().expect("Failed to parse float."))));
            self.consume_token();
            return;
        } else if tokens[self.current_token_num].token_num == STRING && expression_type == STRING {
            self.string_constant(tokens);
            return;
        } else if tokens[self.current_token_num].token_num == IDENTIFIER {
            if self.current_token_num + 1 < tokens.len() {
                // identifier is a function
                if tokens[self.current_token_num + 1].token_num == LEFT_PARENTHESIS {
                    match self.sl_data.get(&tokens[self.current_token_num].token_string) {
                        Some(x) => {
                            if x.return_type == standard_library::INT && expression_type != INT {
                                println!("Type mismatch on line {}.", tokens[self.current_token_num].line_num);
                                self.error = true;
                            } else if x.return_type == standard_library::FLOAT && expression_type != FLOAT {
                                println!("Type mismatch on line {}.", tokens[self.current_token_num].line_num);
                                self.error = true;
                            } else if x.return_type == standard_library::STRING && expression_type != STRING {
                                println!("Type mismatch on line {}.", tokens[self.current_token_num].line_num);
                                self.error = true;
                            } else if x.return_type == standard_library::VEC_INT && expression_type != VEC_INT {
                                println!("Type mismatch on line {}.", tokens[self.current_token_num].line_num);
                                self.error = true;
                            } else if x.return_type == standard_library::VEC_FLOAT && expression_type != VEC_FLOAT {
                                println!("Type mismatch on line {}.", tokens[self.current_token_num].line_num);
                                self.error = true;
                            } else if x.return_type == standard_library::VEC_STRING && expression_type != VEC_STRING {
                                println!("Type mismatch on line {}.", tokens[self.current_token_num].line_num);
                                self.error = true;
                            }

                            let return_type: i64;
                            if expression_type == INT {
                                return_type = standard_library::INT;
                            } else if expression_type == FLOAT {
                                return_type = standard_library::FLOAT;
                            } else if expression_type == STRING {
                                return_type = standard_library::STRING;
                            } else if expression_type == VEC_INT {
                                return_type = standard_library::VEC_INT;
                            } else if expression_type == VEC_FLOAT {
                                return_type = standard_library::VEC_FLOAT;
                            } else if expression_type == VEC_INT {
                                return_type = standard_library::VEC_STRING;
                            } else {
                                return_type = standard_library::VOID;
                            }
                            self.sl_use(tokens, return_type);
                        },
                        None => {
                            self.identifier_function(tokens, expression_type, variable_security);
                        }
                    }
                    return;
                    // identifier is a variable
                } else if tokens[self.current_token_num + 1].token_num != LEFT_PARENTHESIS {
                    self.identifier_variable(tokens, expression_type, variable_security);
                    return;
                }
            }
        } else if tokens[self.current_token_num].token_num == LEFT_PARENTHESIS {
            self.consume_token();
            self.expression(tokens, expression_type, variable_security);
            if tokens[self.current_token_num].token_num == RIGHT_PARENTHESIS {
                self.consume_token();
                return;
            } else if tokens[self.current_token_num].token_num != RIGHT_PARENTHESIS {
                println!("No closing parenthesis on line {}", tokens[self.current_token_num].line_num);
                self.error = true;
                self.consume_token();
                return;
            }
        } else if expression_type == INT && tokens[self.current_token_num].token_num != INT {
            println!("Expected integer, got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
            return;
        } else if expression_type == FLOAT && tokens[self.current_token_num].token_num != FLOAT {
            println!("Expected float, got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
            self.error = true;
            self.consume_token();
            return;
        }
        println!("Invalid Token Type: Expected either literal or grouped expression, got '{}' on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
        self.error = true;
        self.consume_token();
    }















    // Code Generation... Kinda (It's mixed with the parsing code for statements)
    // ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

    fn if_statement(&mut self, tokens: &Vec<lexer::Token>, locals: &mut Vec<i64>, breaks: &mut Vec<i64>) {
        let mut end_of_if_jump_locations: Vec<i64> = Vec::new();
        loop {
            self.if_keyword(tokens);

            let expression_type = self.change_expression_type(tokens);
            self.expression(tokens, expression_type, 100);

            self.code.push(JUMP_IF_FALSE);
            let code_location = self.code.len();
            self.code.push(0);
            self.block(tokens, locals, breaks);
            self.code.push(JUMP);
            end_of_if_jump_locations.push(self.code.len() as i64);
            self.code.push(0);
            let jump_location = self.code.len();
            self.code[code_location] = jump_location as i64;

            if tokens[self.current_token_num].token_num != ELSE {
                break;
            } else {
                self.else_keyword(tokens);
                if tokens[self.current_token_num].token_num == LEFT_CURLEY {
                    self.block(tokens, locals, breaks);
                    break;
                }
            }
        }

        for location in end_of_if_jump_locations {
            self.code[location as usize] = self.code.len() as i64;
        }
    }

    fn while_statement(&mut self, tokens: &Vec<lexer::Token>, locals: &mut Vec<i64>) {
        self.while_keyword(tokens);
        let begin_location = self.code.len() as i64;

        let expression_type = self.change_expression_type(tokens);
        self.expression(tokens, expression_type, 100);
        self.code.push(JUMP_IF_FALSE);
        let code_location = self.code.len();
        self.code.push(0);
        let mut breaks: Vec<i64> = Vec::new();
        self.loop_nest_num += 1;
        self.block(tokens, locals, &mut breaks);
        self.loop_nest_num -= 1;
        self.code.push(JUMP);
        self.code.push(begin_location);
        let jump_location = self.code.len() as i64;
        self.code[code_location] = jump_location;
        for break_statement in breaks {
            self.code[break_statement as usize] = jump_location;
        }
    }

    fn break_statement(&mut self, tokens: &Vec<lexer::Token>, breaks: &mut Vec<i64>) {
        self.break_keyword(tokens);
        self.code.push(JUMP);
        breaks.push(self.code.len() as i64);
        self.code.push(0);
    }

    fn var_def(&mut self, tokens: &Vec<lexer::Token>) {
        let identifier: String = tokens[self.current_token_num].token_string.clone();
        self.identifier(tokens);
        self.equal(tokens);
        let mut expression_type = INT;
        let mut security_level = 0;
        let mut mem_location = 0;
        let mut is_arg: bool = false;
        let mut arg_location: i64 = 0;
        let mut scope = 0;
        match self.var_data.get(&identifier) {
            Some(x) => {
                expression_type = x.var_type;
                security_level = x.security;
                mem_location = x.mem_location;
                is_arg = x.is_arg;
                arg_location = x.arg_location;
                scope = x.scope;
            },
            None => {
                println!("Undeclared variable '{}' on line {}.", identifier, tokens[self.current_token_num].line_num);
                self.error = true;
            },
        }
        if scope > self.current_scope {
            println!("Variable '{}' is not found within this scope on line {}.", identifier, tokens[self.current_token_num].line_num);
            self.error = true;
        }
        self.expression(tokens, expression_type, security_level);

        if is_arg == false {
            self.code.push(LOCAL_STORE);
            self.code.push(mem_location);
        } else {
            self.code.push(ARG_STORE);
            self.code.push(arg_location);
        }
    }

    fn var_dec(&mut self, tokens: &Vec<lexer::Token>, locals: &mut Vec<i64>) {
        self.let_keyword(tokens);
        let identifier: String = tokens[self.current_token_num].token_string.clone();
        if self.var_data.contains_key(&identifier) == true {
            println!("Identifier '{}' already declared on line {}.", identifier, tokens[self.current_token_num].line_num);
            self.error = true;
        }
        self.identifier(tokens);
        let mut var_type: u8 = INT_TYPE;
        let mut expression_type = INT;
        self.stack_size += 1;
        if tokens[self.current_token_num].token_num == INT_TYPE {
            var_type = INT;
            expression_type = INT;
        } else if tokens[self.current_token_num].token_num == FLOAT_TYPE {
            var_type = FLOAT;
            expression_type = FLOAT
        } else if tokens[self.current_token_num].token_num == STRING_TYPE {
            var_type = STRING;
            expression_type = STRING;
        } else if tokens[self.current_token_num].token_num == VEC_INT {
            var_type = VEC_INT;
            expression_type = VEC_INT;
        } else if tokens[self.current_token_num].token_num == VEC_FLOAT {
            var_type = VEC_FLOAT;
            expression_type = VEC_FLOAT;
        } else if tokens[self.current_token_num].token_num == VEC_STRING {
            var_type = VEC_STRING;
            expression_type = VEC_STRING;
        }
        self.var_type(tokens);
        self.colon(tokens);
        let mut security_level: i64 = 0;
        if tokens[self.current_token_num].token_num == INT {
            security_level = tokens[self.current_token_num].token_string.parse::<i64>().expect("Failed to parse integer.");
        }
        if security_level > 100 {
            println!("'{}' exceeds the maximum security of 100 on line {}.", security_level, tokens[self.current_token_num].line_num);
            self.error = true;
        } else if security_level < 0 {
            println!("'{}' is below the lowest security of 0 on line {}.", security_level, tokens[self.current_token_num].line_num);
            self.error = true;
        }
        self.integer(tokens);

        self.equal(tokens);

        self.expression(tokens, expression_type, security_level);

        let mem_location: i64 = self.stack_size - 1;
        locals.push(mem_location);

        let variable = Variable {
            mem_location: mem_location,
            var_type: var_type,
            security: security_level,
            is_arg: false,
            arg_location: 0,
            scope: self.current_scope,
            function_name: self.current_fn_name.clone(),
        };

        self.var_data.insert(identifier.clone(), variable);

        self.code.push(LOCAL_STORE);
        self.code.push(mem_location);
    }

    fn fn_call(&mut self, tokens: &Vec<lexer::Token>) {
        let identifier = tokens[self.current_token_num].token_string.clone();
        let mut mem_location: i64 = 0;
        let mut num_args = 0;
        let mut arg_types: Vec<u8> = Vec::new();
        let mut arg_securities: Vec<i64> = Vec::new();
        match self.fn_data.get(&identifier) {
            Some(x) => {
                num_args = x.num_args;
                arg_types = x.arg_types.clone();
                arg_securities = x.arg_securities.clone();
                mem_location = x.mem_location;
                if x.fn_type != VOID {
                    println!("Function not within an expression must be void on line {}.", tokens[self.current_token_num].line_num);
                    self.error = true;
                }
            },
            None => {
                println!("Unkown function '{}' found on line {}.", identifier, tokens[self.current_token_num].line_num);
                self.error = true;
            },
        }
        self.identifier(tokens);
        self.left_parenthesis(tokens);
        if tokens[self.current_token_num].token_num != RIGHT_PARENTHESIS {
            let mut i = 0 as usize;
            while i < num_args as usize {
                self.expression(tokens, arg_types[i], arg_securities[i]);
                if i != num_args as usize - 1 {
                    self.comma(tokens);
                }
                i += 1;
            }
        }
        self.right_parenthesis(tokens);
        self.code.push(CALL);
        self.code.push(mem_location);
        self.code.push(num_args);
    }

    fn return_statement(&mut self, tokens: &Vec<lexer::Token>, locals: &mut Vec<i64>) {
        self.return_keyword(tokens);
        if self.current_scope == 0 {
            self.return_num += 1;
        }
        let mut fn_type = INT;
        let mut security = 0;
        let mut error = false;
        if tokens[self.current_token_num].token_num != SEMI_COLON {
            match self.fn_data.get(&self.current_fn_name) {
                Some(x) => {
                    if x.fn_type != VOID {
                        fn_type = x.fn_type;
                        security = x.security;
                    } else {
                        error = true;
                        println!("Cannot return value from void function on line {}.", tokens[self.current_token_num].line_num);
                        self.error = true;
                    }
                },
                None => {},
            }
            if error == false {
                self.expression(tokens, fn_type, security);
            }
            locals.sort_by(|a, b| b.cmp(a));
            for local in locals {
                self.code.push(POP);
                self.code.push(*local);
            }
            self.code.push(RETURN_VAL);
        } else {
            match self.fn_data.get(&self.current_fn_name) {
                Some(x) => {
                    if x.fn_type != VOID {
                        println!("Must return value from non-void function on line {}.", tokens[self.current_token_num].line_num);
                        self.error = true;
                    }
                },
                None => {},
            }
            if self.current_fn_name == "main" {
                locals.sort_by(|a, b| b.cmp(a));
                for local in locals {
                    self.code.push(POP);
                    self.code.push(*local);
                }
                self.code.push(HALT);
            } else {
                locals.sort_by(|a, b| b.cmp(a));
                for local in locals {
                    self.code.push(POP);
                    self.code.push(*local);
                }
                self.code.push(RETURN_NON_VAL);
            }
        }
    }

    fn sl_use(&mut self, tokens: &Vec<lexer::Token>, expression_type: i64) {
        if tokens[self.current_token_num].token_string == "print" {
            self.consume_token();
            self.left_parenthesis(tokens);

            loop {
                if tokens[self.current_token_num].token_num == INT {
                    self.expression(tokens, INT, 100);
                    self.code.push(USE);
                    self.code.push(standard_library::PRINT);
                    self.code.push(standard_library::INT);
                } else if tokens[self.current_token_num].token_num == FLOAT {
                    self.expression(tokens, FLOAT, 100);
                    self.code.push(USE);
                    self.code.push(standard_library::PRINT);
                    self.code.push(standard_library::FLOAT);
                } else if tokens[self.current_token_num].token_num == STRING {
                    self.expression(tokens, STRING, 100);
                    self.code.push(USE);
                    self.code.push(standard_library::PRINT);
                    self.code.push(standard_library::STRING);
                } else if tokens[self.current_token_num].token_num == IDENTIFIER {
                    let expression_type: u8;
                    expression_type = self.change_expression_type(tokens);
                    self.expression(tokens, expression_type, 100);
                    if expression_type == INT {
                        self.code.push(USE);
                        self.code.push(standard_library::PRINT);
                        self.code.push(standard_library::INT);
                    } else if expression_type == FLOAT {
                        self.code.push(USE);
                        self.code.push(standard_library::PRINT);
                        self.code.push(standard_library::FLOAT);
                    } else if expression_type == STRING {
                        self.code.push(USE);
                        self.code.push(standard_library::PRINT);
                        self.code.push(standard_library::STRING);
                    }
                } else {
                    println!("Unknown token '{}', expected beginning of expression on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
                    self.error = true;
                }
                if tokens[self.current_token_num].token_num != COMMA {
                    break;
                } else {
                    self.comma(tokens);
                }
            }
    
            self.right_parenthesis(tokens);
        } else if tokens[self.current_token_num].token_string == "read" {
            self.consume_token();
            self.left_parenthesis(tokens);
            self.right_parenthesis(tokens);
            self.code.push(USE);
            self.code.push(standard_library::READ);
            self.code.push(expression_type);
        } else {
            let identifier = tokens[self.current_token_num].token_string.clone();
            let types: Vec<i64>;
            let num_types: i64;
            match self.sl_data.get_mut(&identifier) {
                Some(x) => {
                    types = x.types.clone();
                    num_types = x.num_types;
                },
                None => {
                    types = Vec::new();
                    num_types = 0;
                },
            }
            self.consume_token();
            self.left_parenthesis(tokens);
            
            let mut current_arg_type = 0;
            for arg_type in types {
                if arg_type == standard_library::INT {
                    self.expression(tokens, INT, 100);
                } else if arg_type == standard_library::FLOAT {
                    self.expression(tokens, FLOAT, 100);
                } else if arg_type == standard_library::STRING {
                    self.expression(tokens, STRING, 100);
                } else if arg_type == standard_library::VEC_INT {
                    self.expression(tokens, VEC_INT, 100);
                } else if arg_type == standard_library::VEC_FLOAT {
                    self.expression(tokens, VEC_FLOAT, 100);
                } else if arg_type == standard_library::VEC_STRING {
                    self.expression(tokens, VEC_STRING, 100);
                }
                current_arg_type += 1;
                if current_arg_type != num_types {
                    self.comma(tokens);
                }
            }
            self.right_parenthesis(tokens);

            match self.sl_data.get_mut(&identifier) {
                Some(x) => {
                    self.code.push(USE);
                    self.code.push(x.sl_num);
                },
                None => {},
            }
        }
    }

    fn identifier_function(&mut self, tokens: &Vec<lexer::Token>, expression_type: u8, variable_security: i64) {
        let identifier = tokens[self.current_token_num].token_string.clone();
        let mut mem_location: i64 = 0;
        let mut num_args: i64 = 0;
        let mut security: i64 = 0;
        let mut fn_type: u8 = INT;
        let mut arg_types: Vec<u8> = Vec::new();
        let mut arg_securities: Vec<i64> = Vec::new();
        match self.fn_data.get(&identifier) {
            Some(x) => {
                num_args = x.num_args;
                arg_types = x.arg_types.clone();
                arg_securities = x.arg_securities.clone();
                mem_location = x.mem_location;
                security = x.security;  
                fn_type = x.fn_type;
            },
            None => {
                println!("Unkown function '{}' found on line {}.", identifier, tokens[self.current_token_num].line_num);
                self.error = true;
            },
        }
        if fn_type != expression_type {
            println!("Type mismatch: function '{}' on line {}.", identifier, tokens[self.current_token_num].line_num);
            self.error = true;
        }
        if security > variable_security {
            println!("Max security level exceeded with '{}' on line {}.", identifier, tokens[self.current_token_num].line_num);
            self.error = true;
        }
        self.identifier(tokens);
        self.left_parenthesis(tokens);
        if tokens[self.current_token_num].token_num != RIGHT_PARENTHESIS {
            let mut i = 0 as usize;
            while i < num_args as usize {
                self.expression(tokens, arg_types[i], arg_securities[i]);
                if i != num_args as usize - 1 {
                    self.comma(tokens);
                }
                i += 1;
            }
        }
        self.right_parenthesis(tokens);
        self.code.push(CALL);
        self.code.push(mem_location);
        self.code.push(num_args);
    }

    fn string_constant(&mut self, tokens: &Vec<lexer::Token>) {
        self.code.push(S_CONSTANT);
        let mut backslash: bool = false;
        for c in tokens[self.current_token_num].token_string.clone().chars() {
            if backslash == false {
                if c != '"' && c != '\\' {
                    self.code.push(c as u8 as i64);
                } else if c == '\\' {
                    backslash = true;
                }
            } else {
                if c == 'n' {
                    self.code.push(10 as i64);
                    backslash = false;
                } else {
                    println!("Expect 'n' after backslash in token {} on line {}.", tokens[self.current_token_num].token_string, tokens[self.current_token_num].line_num);
                    self.error = true;
                    backslash = false;
                }
            }
        }
        self.code.push(0);
        self.consume_token();
    }

    fn identifier_variable(&mut self, tokens: &Vec<lexer::Token>, expression_type: u8, variable_security: i64) {
        let identifier: String = tokens[self.current_token_num].token_string.clone();
        let mut var_type = INT;
        let mut security_level = 0;
        let mut mem_location = 0;
        let mut is_arg = false;
        let mut arg_location = 0;
        let mut scope = 0;
        match self.var_data.get(&identifier) {
            Some(x) => {
                var_type = x.var_type;
                security_level = x.security;
                mem_location = x.mem_location;
                is_arg = x.is_arg;
                arg_location = x.arg_location;
                scope = x.scope;
            },
            None => println!("Undeclared variable '{}' on line {}.", identifier, tokens[self.current_token_num].line_num),
        }

        if var_type != expression_type {
            println!("Type mismatch: identifier '{}' on line {}.", identifier, tokens[self.current_token_num].line_num);
            self.error = true;
        }
        if security_level > variable_security {
            println!("Max security level exceeded with '{}' on line {}.", identifier, tokens[self.current_token_num].line_num);
            self.error = true;
        }
        if scope > self.current_scope {
            println!("Variable '{}' is not found within this scope on line {}.", identifier, tokens[self.current_token_num].line_num);
            self.error = true;
        }
        if is_arg == false {
            self.code.push(LOCAL_LOAD);
            self.code.push(mem_location);
        } else {
            self.code.push(ARG_LOAD);
            self.code.push(arg_location);
        }
        self.consume_token();
    }
}