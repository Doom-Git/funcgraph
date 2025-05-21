fn print_to_terminal() {
	print!("I print to the terminal hihi");
}

fn call_a_fn() {
	print_to_terminal();
}

fn test_test() {
	call_a_fn();
	print_to_terminal();
}


fn get_recursived() {
	get_recursived();
}