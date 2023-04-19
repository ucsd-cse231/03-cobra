mod infra;

// Your tests go here!
success_tests! {
    {
        name: false_val,
        expected: "false",
    },

    {
        name: input_compare,
        input: "2",
        expected: "false",
    },
}

runtime_error_tests! {
    {
        name: invalid_argument,
        expected: "invalid argument",
    },
}

static_error_tests! {
    {
        name: number_bounds_fail,
        expected: "invalid",
    }
}
