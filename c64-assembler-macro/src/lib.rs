use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn application(input: TokenStream) -> TokenStream {
    //dbg!(input.clone());
    let mut lines = Vec::<String>::default();
    lines.push("{".to_string());
    lines.push("  use c64_assembler::builder::{*};".to_string());
    lines.push("  ApplicationBuilder::default()".to_string());
    let mut iter = input.into_iter();
    while let Some(tree) = iter.next() {
        if let TokenTree::Ident(identifier) = tree {
            let name = identifier.to_string();
            if name == "name" {
                let mut line = Vec::<String>::default();
                line.push("    .name(".to_string());
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Literal(application_name)) = iter.next() {
                    line.push(format!("{application_name})"));
                    lines.push(line.join(""));
                }
            }
            if name == "entry_point" {
                let mut line = Vec::<String>::default();
                line.push("    .entry_point(".to_string());
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Literal(entry_point)) = iter.next() {
                    line.push(format!("{entry_point})"));
                    lines.push(line.join(""));
                }
            }
            if name == "include_vic2_defines" {
                lines.push("    .include_vic2_defines()".to_string());
            }
            if name == "include_sid_defines" {
                lines.push("    .include_sid_defines()".to_string());
            }
            if name == "module" {
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Group(sub_tree)) = iter.next() {
                    lines.push("    .module(".to_string());
                    lines.push(build_module(sub_tree.stream()));
                    lines.push("    )".to_string());
                }
            }
        }
    }
    lines.push("    .build()".to_string());
    lines.push("}".to_string());

    //println!("{:#?}", lines.join("\n"));
    lines.join("\n").parse().unwrap()
}

fn build_module(input: TokenStream) -> String {
    let mut lines = Vec::<String>::default();
    lines.push("  ModuleBuilder::default()".to_string());
    let mut iter = input.into_iter();
    while let Some(tree) = iter.next() {
        if let TokenTree::Ident(identifier) = tree {
            let name = identifier.to_string();
            if name == "name" {
                let mut line = Vec::<String>::default();
                line.push("    .name(".to_string());
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Literal(module_name)) = iter.next() {
                    line.push(format!("{module_name})"));
                    lines.push(line.join(""));
                }
            }
            if name == "instructions" {
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Group(sub_tree)) = iter.next() {
                    lines.push("    .instructions(".to_string());
                    lines.push(build_instructions(sub_tree.stream()));
                    lines.push("    )".to_string());
                }
            }
            if name == "function" {
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Group(sub_tree)) = iter.next() {
                    lines.push("    .function(".to_string());
                    lines.push(build_function(sub_tree.stream()));
                    lines.push("    )".to_string());
                }
            }
        }
    }
    lines.push("    .build()".to_string());
    lines.join("\n")
}

fn build_function(input: TokenStream) -> String {
    let mut lines = Vec::<String>::default();
    lines.push("  FunctionBuilder::default()".to_string());
    let mut iter = input.into_iter();
    while let Some(tree) = iter.next() {
        if let TokenTree::Ident(identifier) = tree {
            let name = identifier.to_string();
            if name == "name" {
                let mut line = Vec::<String>::default();
                line.push("    .name(".to_string());
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Literal(function_name)) = iter.next() {
                    line.push(format!("{function_name})"));
                    lines.push(line.join(""));
                }
            }
            if name == "instructions" {
                let _eq = iter.next().unwrap();
                if let Some(TokenTree::Group(sub_tree)) = iter.next() {
                    lines.push("    .instructions(".to_string());
                    lines.push(build_instructions(sub_tree.stream()));
                    lines.push("    )".to_string());
                }
            }
        }
    }
    lines.push("    .build()".to_string());
    lines.join("\n")
}
/*
Ident {
    ident: "jmp",
    span: #0 bytes(20193..20196),
},
Group {
    delimiter: Parenthesis,
    stream: TokenStream [
        Ident {
            ident: "test",
            span: #0 bytes(20198..20202),
        },
        ],
        span: #0 bytes(20197..20203),
    },
`*/
fn build_address_mode(
    line: &mut Vec<String>,
    tokens: &[TokenTree],
    allow_immediate: bool,
    allow_accumulator: bool,
    allow_absolute: bool,
    allow_indirect: bool,
) -> usize {
    match tokens.first().unwrap() {
        TokenTree::Punct(punct) => {
            if punct.to_string() == "#" {
                assert!(allow_immediate);
                return build_address_mode_imm(line, &tokens[1..]) + 1;
            }
        }
        TokenTree::Ident(ident) => {
            if ident.to_string() == *"a" {
                assert!(allow_accumulator);
                return build_address_mode_accumulator(line, tokens);
            }
            assert!(allow_absolute);
            return build_address_mode_absolute(line, tokens);
        }
        TokenTree::Group(_group) => {
            assert!(allow_indirect);
            return build_address_mode_indirect(line, &tokens[0..]);
        }
        _ => todo!("HUH!"),
    }
    0
}

fn build_address_mode_accumulator(line: &mut Vec<String>, _tokens: &[TokenTree]) -> usize {
    line.push("_acc()".to_string());
    1
}

fn build_address_mode_absolute(line: &mut Vec<String>, tokens: &[TokenTree]) -> usize {
    let mut num_tokens = 1;
    let address = if let Some(TokenTree::Ident(ident)) = tokens.first() {
        ident.to_string()
    } else {
        "".to_string()
    };
    line.push("_addr".to_string());
    let mut offset = 0;
    let mut index = "".to_string();
    if let Some(TokenTree::Punct(p)) = tokens.get(1) {
        if p.as_char() == '+' {
            if let Some(TokenTree::Literal(l)) = tokens.get(2) {
                offset = l.to_string().parse::<u16>().unwrap();
            }
            num_tokens = 3;
        }
        if p.as_char() == ',' {
            if let Some(TokenTree::Ident(identifier)) = tokens.get(2) {
                index = identifier.to_string();
            }
            num_tokens = 3;
        }
    }
    if (offset) > 0 {
        line.push("_offs".to_string());
    }
    if !index.is_empty() {
        line.push(format!("_{}", index));
    }

    line.push("(".to_string());
    line.push(format!("\"{}\"", address));
    if offset > 0 {
        line.push(format!(", {}", offset));
    }
    line.push(")".to_string());
    num_tokens
}

fn build_address_mode_indirect(line: &mut Vec<String>, tokens: &[TokenTree]) -> usize {
    let mut is_indirect_indexed = false;
    let mut is_indexed_indirect = false;
    let mut address = String::new();
    if let TokenTree::Group(group) = &tokens[0] {
        for token in group.stream() {
            if let TokenTree::Ident(identifier) = &token {
                address = identifier.to_string();
            }
            if let TokenTree::Punct(punct) = &token {
                if punct.as_char() == ',' {
                    is_indexed_indirect = true;
                    break;
                }
            }
        }
    }
    if let Some(TokenTree::Punct(punct)) = &tokens.get(1) {
        if punct.as_char() == ',' {
            is_indirect_indexed = true;
        }
    }

    if is_indexed_indirect {
        line.push(format!("_ind_x(\"{address}\")"));
        1
    } else if is_indirect_indexed {
        line.push(format!("_ind_y(\"{address}\")"));
        3
    } else {
        line.push(format!("_ind(\"{address}\")"));
        1
    }
}

fn build_address_mode_imm(line: &mut Vec<String>, tokens: &[TokenTree]) -> usize {
    let mut num_tokens = 0;
    line.push("_imm".to_string());
    let mut is_hex = false;
    let mut is_low = false;
    let mut is_high = false;
    for token in tokens {
        num_tokens += 1;
        match token {
            TokenTree::Punct(punct) => {
                if punct.to_string() == "$" {
                    is_hex = true;
                }
                if punct.to_string() == "<" {
                    is_low = true;
                }
                if punct.to_string() == ">" {
                    is_high = true;
                }
            }
            TokenTree::Literal(value) => {
                if is_hex {
                    line.push(format!("(0x{})", value));
                } else {
                    line.push(format!("({})", value));
                }
                break;
            }
            TokenTree::Ident(value) => {
                if is_low {
                    line.push(format!("_low(\"{}\")", value));
                } else if is_high {
                    line.push(format!("_high(\"{}\")", value));
                } else if is_hex {
                    line.push(format!("(0x{})", value));
                } else {
                    line.push(format!("(\"{}\")", value));
                }
                break;
            }
            _ => todo!(),
        }
    }
    num_tokens
}

fn build_instructions(input: TokenStream) -> String {
    let mut lines = Vec::<String>::default();
    lines.push("  InstructionBuilder::default()".to_string());
    let tokens = input.into_iter().collect::<Vec<TokenTree>>();
    let mut sub_start = 0;
    for i in 0..tokens.len() {
        if i < sub_start {
            continue;
        }
        let token = &tokens[i];
        if let TokenTree::Ident(identifier) = token {
            let name = identifier.to_string();
            match name.as_str() {
                "include_basic_header" => {
                    lines.push("    .add_basic_header()".to_string());
                    sub_start = i + 1;
                }

                "asl" | "lsr" | "ror" | "rol" => {
                    let mut line = Vec::default();
                    line.push(format!("    .{name}"));
                    let add_tokens_parsed = build_address_mode(&mut line, &tokens[i + 1..], false, true, true, false);
                    lines.push(line.join(""));
                    sub_start = i + 1 + add_tokens_parsed;
                }
                "adc" | "and" | "cmp" | "cpx" | "cpy" | "eor" | "lda" | "ldx" | "ldy" | "ora" | "sbc" => {
                    let mut line = Vec::default();
                    line.push(format!("    .{name}"));
                    let add_tokens_parsed = build_address_mode(&mut line, &tokens[i + 1..], true, false, true, true);
                    lines.push(line.join(""));
                    sub_start = i + 1 + add_tokens_parsed;
                }

                "bcc" | "bcs" | "beq" | "bmi" | "bne" | "bpl" | "bvc" | "bvs" | "jsr" | "bit" | "dec" | "inc" => {
                    let mut line = Vec::default();
                    line.push(format!("    .{name}"));
                    let add_tokens_parsed = build_address_mode(&mut line, &tokens[i + 1..], false, false, true, false);
                    lines.push(line.join(""));
                    sub_start = i + 1 + add_tokens_parsed;
                }

                "sta" | "stx" | "sty" | "jmp" => {
                    let mut line = Vec::default();
                    line.push(format!("    .{name}"));
                    let add_tokens_parsed = build_address_mode(&mut line, &tokens[i + 1..], false, false, true, true);
                    lines.push(line.join(""));
                    sub_start = i + 1 + add_tokens_parsed;
                }

                "brk" | "cld" | "cli" | "clv" | "dex" | "dey" | "inx" | "iny" | "nop" | "pha" | "psr" | "pla"
                | "php" | "plp" | "rti" | "sec" | "sed" | "sei" | "tax" | "tay" | "tsx" | "txa" | "txs" | "tya"
                | "clc" | "rts" => {
                    lines.push(format!("    .{name}()"));
                    sub_start = i + 1;
                }

                &_ => {
                    lines.push(format!("    .label(\"{name}\")"));
                    sub_start = i + 2;
                }
            }
        }
    }

    lines.push("    .build()".to_string());
    lines.join("\n")
}

#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    //dbg!(input.clone());
    let mut lines = Vec::<String>::default();
    lines.push("{".to_string());
    lines.push("  use c64_assembler::builder::{*};".to_string());
    lines.push(build_module(input));
    lines.push("}".to_string());
    //println!("{:#?}", lines.join("\n"));
    lines.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn function(input: TokenStream) -> TokenStream {
    //dbg!(input.clone());
    let mut lines = Vec::<String>::default();
    lines.push("{".to_string());
    lines.push("  use c64_assembler::builder::{*};".to_string());
    lines.push(build_function(input));
    lines.push("}".to_string());
    //println!("{:#?}", lines.join("\n"));
    lines.join("\n").parse().unwrap()
}

#[proc_macro]
pub fn instructions(input: TokenStream) -> TokenStream {
    //dbg!(input.clone());
    let mut lines = Vec::<String>::default();
    lines.push("{".to_string());
    lines.push("  use c64_assembler::builder::{*};".to_string());
    lines.push(build_instructions(input));
    lines.push("}".to_string());
    //println!("{:#?}", lines.join("\n"));
    lines.join("\n").parse().unwrap()
}
