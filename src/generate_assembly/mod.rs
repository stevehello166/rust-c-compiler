use crate::abstract_syntax_tree::*;

pub fn generate_assembly(ast: Program) -> Result<String, ()>{
    let mut assembly_out = ".globl _main \n".to_owned();
    let mut function_main = false;
    for function in ast.0 {
        if function.name == "main".to_owned() {
            function_main = true;
        }
        
        let mut function_code = "".to_owned();
        function_code.push_str(&("_".to_owned() + &function.name + ":\n"));

        for statement in function.body.0{
            
            match statement.statement_type{
                StatementType::Return(return_arg) => {
                    let constant = match return_arg{
                        Expression::Constant(constant) => constant,
                        _ => todo!("HOW THE FUCK"),
                    };
                    function_code.push_str(&("    movl $".to_owned() + &constant.to_string() + ", %eax\n"));
                    function_code.push_str("    ret");
                },
                _ => todo!()
            }
        }
        assembly_out.push_str(&function_code);
    }
    if function_main == false {
        return Err(())
    }
    return Ok(assembly_out)
}

//assign to expression is movl
// return is ret