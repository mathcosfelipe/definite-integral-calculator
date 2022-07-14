extern crate beginner_tools;
use beginner_tools::input;

mod integral;

fn main() {
    
    let amount_operations: u32 = loop {
        if let Ok(amount_operations_validity) = input("Inform the amount of operations you want calculate.\nExample (2x + 5), is equal 2 operations: ") {
            break amount_operations_validity
        };
        println!("Invalid value! Input a integer number. Try again.");
    };

    let mut operations: Vec<String> = Vec::new();

    for operation_input in 1..amount_operations + 1 {
        let operation: String = loop {
            let operation_message: String = format!("Input the operation nº {}: ", operation_input);
            if let Ok(operation_validity) = input(&operation_message) {
                break operation_validity
            };
            println!("Syntax error! Try again.");
        };
        operations.push(operation.trim().to_string());
    }

    let mut limits_operations: Vec<Vec<f32>> = Vec::new();

    for limit_input in 1..amount_operations + 1 {
    
        let inferior_limit: f32 = loop {
            let inferior_limit_message: String = format!("Input the inferior limit of operation nº {}: ", limit_input);
            if let Ok(inferior_limit_message_validity) = input(&inferior_limit_message) {
                break inferior_limit_message_validity
            };
            println!("Invalid value! Input a float number. Try again.");
        };
    
        let superior_limit: f32 = loop {
            let superior_limit_message: String = format!("Input the superior limit of operation nº {}: ", limit_input);
            if let Ok(superior_limit_message_validity) = input(&superior_limit_message) {
                break superior_limit_message_validity
            };
            println!("Invalid value! Input a float number. Try again.");
        };

        let mut limits_operation: Vec<f32> = Vec::new();
        limits_operation.push(inferior_limit);
        limits_operation.push(superior_limit);
        limits_operations.push(limits_operation);
    
    }

    integral::calculate(operations, limits_operations);

}