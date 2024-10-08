use crate::*;

#[test]
fn multidigit_test(){
    let source = "src/tests/generaltests/stage_1/valid/multi_digit.c";
    let mut source = read_to_string(source).expect("Failed to read string");
    
    let tokens = lexer::lexer(source);
   
    let abstract_syntax_tree = create_abstract_syntax_tree(tokens).expect("REASON");
    let finished_asm = generate_assembly(abstract_syntax_tree).unwrap();
    assert_eq!(finished_asm, ".globl _main \n_main:\n    movl $100, %eax\n    ret")
}

#[test]
fn newlines_test(){
    let source = "src/tests/generaltests/stage_1/valid/newlines.c";
    let mut source = read_to_string(source).expect("Failed to read string");
    
    let tokens = lexer::lexer(source);
   
    let abstract_syntax_tree = create_abstract_syntax_tree(tokens).expect("REASON");
    let finished_asm = generate_assembly(abstract_syntax_tree).unwrap();
    assert_eq!(finished_asm, ".globl _main \n_main:\n    movl $0, %eax\n    ret")
}

#[test]
fn no_newlines_test(){
    let source = "src/tests/generaltests/stage_1/valid/no_newlines.c";
    let mut source = read_to_string(source).expect("Failed to read string");
    
    let tokens = lexer::lexer(source);
   
    let abstract_syntax_tree = create_abstract_syntax_tree(tokens).expect("REASON");
    let finished_asm = generate_assembly(abstract_syntax_tree).unwrap();
    assert_eq!(finished_asm, ".globl _main \n_main:\n    movl $0, %eax\n    ret")
}

#[test]
fn return_0_test(){
    let source = "src/tests/generaltests/stage_1/valid/return_0.c";
    let mut source = read_to_string(source).expect("Failed to read string");
    
    let tokens = lexer::lexer(source);
   
    let abstract_syntax_tree = create_abstract_syntax_tree(tokens).expect("REASON");
    let finished_asm = generate_assembly(abstract_syntax_tree).unwrap();
    assert_eq!(finished_asm, ".globl _main \n_main:\n    movl $0, %eax\n    ret")
}

#[test]
fn return_2_test(){
    let source = "src/tests/generaltests/stage_1/valid/return_2.c";
    let mut source = read_to_string(source).expect("Failed to read string");
    
    let tokens = lexer::lexer(source);
   
    let abstract_syntax_tree = create_abstract_syntax_tree(tokens).expect("REASON");
    let finished_asm = generate_assembly(abstract_syntax_tree).unwrap();
    assert_eq!(finished_asm, ".globl _main \n_main:\n    movl $2, %eax\n    ret")
}

#[test]
fn spaces_test(){
    let source = "src/tests/generaltests/stage_1/valid/spaces.c";
    let mut source = read_to_string(source).expect("Failed to read string");
    
    let tokens = lexer::lexer(source);
   
    let abstract_syntax_tree = create_abstract_syntax_tree(tokens).expect("REASON");
    let finished_asm = generate_assembly(abstract_syntax_tree).unwrap();
    assert_eq!(finished_asm, ".globl _main \n_main:\n    movl $0, %eax\n    ret")
}