use std::error::Error;

// Return the correct answer for the given turn index as a String
//
// This returns String rather than &str because converting a usize requires a heap allocation.
// If it only returned literals, it could use &str instead and point directly at the literals inside the binary.
fn answer(turn: usize) -> String {
    if turn % 15 == 0 {
        "fizz buzz".into()
    } else if turn % 5 == 0 {
        "buzz".into()
    } else if turn % 3 == 0 {
        "fizz".into()
    } else {
        turn.to_string()
    }
}

// You could also encode the if-else chain as a match,
// but it seems less idiomatic here since it needs don't-care _ binds sprinkled throughout
#[allow(dead_code)]
fn answer_alternate(turn: usize) -> String {
    match turn {
        _ if turn % 15 == 0 => "fizz buzz".into(),
        _ if turn % 5 == 0 => "buzz".into(),
        _ if turn % 3 == 0 => "fizz".into(),
        _ => turn.to_string(),
    }
}

// main() can return a Result<V, E> to allow use of the ? syntax - more on that below.
//
// In this case, V (the success value) is the empty tuple type (),
// because we don't have anything of note to return.
//
// E (the error value) uses Box<dyn Error>, which essentially means "cast whatever error type comes out to a generic error",
// and is a common pattern if you need to return errors of different types (such as a mixture of IO, network, and library error types)
// It could also be replaced by std::io::Result to match the output of read_line, since that's the only error type being used in this program.
fn main() -> Result<(), Box<dyn Error>> {
    // Standard input handle
    let stdin = std::io::stdin();

    // Buffer to hold input
    // Mutable, as Stdin::read_line needs to write into it
    let mut buf = String::new();

    // 1.. is sugar from FromRange<i32>::new(1), which is a type that represents an unbounded range from a given integer to infinity.
    // It also implements the Iterator trait, meaning we can map the answer function over it to create an infinite iterator of correct answers!
    // Immutable, because using it in the for loop below invokes into_iter(), which moves it by-value instead of mutating it in-place.
    let answers = (1..).map(answer);

    // Iterate through our list of correct answers
    for answer in answers {
        // Prompt the user for input
        println!("Enter your answer:");

        // Clear the buffer so we don't accrue previous lines
        buf.clear();

        // Read a line of input into the buffer
        // Note the ? suffix - that's syntactic sugar for "try unwrapping the returned Result type into its success value, return the error value on failure"
        stdin.read_line(&mut buf)?;

        // Strip off the terminating newline characters if they exist, and convert to lowercase to avoid capitalization trouble
        let cmp = buf
            .strip_suffix("\n")
            .unwrap_or(&buf)
            .strip_suffix("\r")
            .unwrap_or(&buf)
            .to_lowercase();

        // Compare with the answer from our iterator, returning an informative error if incorrect.
        if cmp == answer {
            println!("Correct.\n");
        } else {
            return Err(format!("Incorrect. The right answer was {}.", answer).into());
        }
    }

    // Since the game will iterate through correct answers forever until an error is returned, we'll never actually get here.
    // However, the function demands a return value - usually Ok(()) in this case - for correct syntax,
    // but there's a more idiomatic way to encode a non-returning function.
    //
    // unreachable!, or other related macros such as panic! that cause the program to cease,
    // can lift this restriction and allow you to be more explicit that the function won't return via this path.
    unreachable!()
}

// Use 'cargo test' to run unit tests.
// #[cfg(test)] makes the tests module only compile when running via cargo test.
#[cfg(test)]
mod tests {
    use super::*;

    // Array of expected answers lifted shamelessly from the wikipedia page.
    const FIZZ_BUZZ_ANSWERS: [&str; 36] = [
        "1",
        "2",
        "fizz",
        "4",
        "buzz",
        "fizz",
        "7",
        "8",
        "fizz",
        "buzz",
        "11",
        "fizz",
        "13",
        "14",
        "fizz buzz",
        "16",
        "17",
        "fizz",
        "19",
        "buzz",
        "fizz",
        "22",
        "23",
        "fizz",
        "buzz",
        "26",
        "fizz",
        "28",
        "29",
        "fizz buzz",
        "31",
        "32",
        "fizz",
        "34",
        "buzz",
        "fizz",
    ];

    // The #[test] attribute marks this function as a unit test, hooking it into the 'cargo test' machinery
    #[test]
    fn test_fizz_buzz() {
        // Iterate from 1 to 36 (n..=m makes the RHS bound include instead of exclusive),
        // convert the resulting indices into strings, and collect them into a Vec<_> (which gets type-inferred to Vec<String>.)
        let answers = (1..=36).map(answer).collect::<Vec<_>>();

        // Compare the generated answers against the vector of expected answer literals.
        //
        // Rather pleasantly, despite the answers being Vec<String> and the literals being [&str],
        // assert_eq! is able to coerce them both down to string slice collections and do == comparisons without any extra fuss
        assert_eq!(answers, FIZZ_BUZZ_ANSWERS);
    }
}
