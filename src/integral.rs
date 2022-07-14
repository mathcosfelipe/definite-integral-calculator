fn integrate(operations: Vec<String>) -> Vec<String> {

    let mut operations_integrateds: Vec<String> = Vec::new();

    for operation in 0..operations.len() {
        
        if operations[operation].contains("x") {
            
            if operations[operation] == "-x" {
                operations_integrateds.push(("-x^2/2").to_string());
            }

            if operations[operation] == "x" {
                operations_integrateds.push(("x^2/2").to_string());
            } 

        }
    
    }

    operations
}

pub(crate) fn calculate(operations: Vec<String>, _limits_operations: Vec<Vec<f32>>) {

    let _operations_integrated: Vec<String> = integrate(operations);

}