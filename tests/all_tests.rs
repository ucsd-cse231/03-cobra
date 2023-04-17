mod infra;

// Your tests go here!
success_tests! {
    {
        name: test_name,
        input: "10",
        expected: "output",
    },
}

runtime_error_tests! {
    {
        name: test_name2,
        input: "10",
        expected: "output",
    }
}

compiler_error_tests! {

    {
        name: test_name3,
        expected: "output",
    }
}
