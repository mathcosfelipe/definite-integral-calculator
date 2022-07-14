// const axes: [String, 2] = ["a"; "b"];

fn integrate(operations: Vec<String>) -> Vec<String> {

    for operation in 0..operations.len() {
        println!("{}", operations[operation]);
    }

    operations
}

pub(crate) fn calculate(operations: Vec<String>, _limits_operations: Vec<Vec<f32>>) {

    let _operations_integrated: Vec<String> = integrate(operations);

}