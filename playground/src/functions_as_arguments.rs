/*
Example of passing functions as arguments
Function objects - fn() - are static and can't capture from their environment
Function trait objects - dyn Fn() - are dynamic and can capture, but are subject to borrow checking
*/

#[cfg(test)]
mod tests {
    fn invoke_fn_trait(func: &dyn Fn()) {
        func()
    }

    fn invoke_fn_object(func: fn()) {
        func()
    }

    #[test]
    fn functions_as_arguments() {
        fn func() {
            println!("Hello func!");
        }

        let simple_closure = || println!("Hello closure");

        let capture = "Capture";
        let capture_closure = || {
            println!("Hello {:?}!", capture);
        };

        invoke_fn_trait(&func);
        invoke_fn_trait(&simple_closure);
        invoke_fn_trait(&capture_closure);

        invoke_fn_object(func);
        invoke_fn_object(simple_closure);

        // Fails to compile, as a closure that captures is not a static fn object
        //invoke_fn_object(capture_closure);
    }
}
