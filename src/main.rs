use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use std::env;

/// Implementation of a binary search!
///
/// Returns the index of `value` in the specified sorted `array`, or None if absent.
fn binary_search(array: &[u32], value: u32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let index = array.len() / 2;
    if array[index] == value {
        Some(index)
    } else if array.len() == 1 {
        None
    } else if array[index] > value {
        binary_search(&array[0..index], value)
    } else {
        binary_search(&array[index..], value).map(|i| i + index)
    }
}

fn generate_random_input(seed: u64) -> (Vec<u32>, u32) {
    // RNG (random number generator) from the seed!
    let mut rng = StdRng::seed_from_u64(seed);

    // Generate input parameters.
    const MAX_TEST_LENGTH: usize = 8;
    const MAX_TEST_VALUE: u32 = 8;

    // Choose a random input length USING THE RNG!
    let length: usize = rng.gen_range(0, MAX_TEST_LENGTH);

    // Generate a random, sorted input array.
    //
    // ... USING THE RNG AGAIN!
    let mut input = Vec::with_capacity(length);
    for _ in 0..length {
        input.push(rng.gen_range(0, MAX_TEST_VALUE));
    }
    input.sort();

    // Generate a random value to search for.
    //
    // ... WITH THE RNG AGAIN!
    let value = rng.gen_range(0, MAX_TEST_VALUE);

    (input, value)
}

/// Returns whether or not this test passed.
fn test(seed: u64) -> bool {
    println!("Testing with random seed: {:?}", seed);
    let (random_array, random_value) = generate_random_input(seed);

    // Actually run the code we're trying to test!
    let actual = binary_search(&random_array, random_value);

    // Validate correctness. In this case, it's easy to express exhaustive
    // invariants for our function:
    //   * If the value is present in the array, its index should be returned.
    //   * Otherwise, None should be returned.
    let expected = random_array.iter().position(|&v| v == random_value);
    if expected != actual
        && actual
            .map(|i| random_array[i] != random_value)
            .unwrap_or(true)
    {
        println!(
            "  FAIL: Searched for {:?} in input array {:?}. Expected: {:?}. Actual: {:?}",
            random_value, random_array, expected, actual
        );
        false
    } else {
        // println!(
        //     "  PASS: Searched for {:?} in input array {:?}. Got: {:?}",
        //     random_value, random_array, actual
        // );
        true
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // When we're coming up with random test cases!
        const NUM_RANDOM_TESTS: usize = 1000000;
        let mut passes = 0;
        for _ in 0..NUM_RANDOM_TESTS {
            let seed = thread_rng().gen();
            let result = test(seed);
            if result {
                passes += 1;
            }
        }
        println!("Passed {:?} out of {:?} tests!", passes, NUM_RANDOM_TESTS);
    } else {
        // Debugging a particular test case...
        let seed = args[1].parse().expect("Failed to parse input");
        test(seed);
    };
}
