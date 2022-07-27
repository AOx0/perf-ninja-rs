#![feature(bench_black_box)]

use std::fs::read_to_string;

use compiler_intrinsics_2::solution;

fn main() {
    let inputs = vec![
        /*"counter-example.txt" // input where sequential solution is faster*/
        "inputs/LoopVectorize.cpp", // a large C++ file from the LLVM compiler.
        "inputs/MarkTwain-TomSawyer.txt", // a typical text file with long lines.
    ];

    let mut input_contents = Vec::with_capacity(inputs.len());
    for input in &inputs {
        let input_content = read_to_string(input).unwrap();
        input_contents.push(input_content);
    }

    for _ in 0..500 {
        for input_content in &input_contents {
            let output = solution(input_content);
            std::hint::black_box(output);
        }
    }
}

#[cfg(test)]
mod tests {
    use compiler_intrinsics_2::solution;
    use std::cmp::max;
    use std::fs::read_to_string;

    #[test]
    fn validate() -> std::io::Result<()> {
        // feel free to comment out tests for debugging
        let inputs = vec![
            "inputs/test1.txt",               // basic test
            "inputs/test2.txt",               // no end-of-line in the end
            "inputs/test3.txt",               // small number of characters
            "inputs/LoopVectorize.cpp",       // a large C++ file from the LLVM compiler.
            "inputs/MarkTwain-TomSawyer.txt", // a typical text file with long lines.
            "inputs/counter-example.txt",     // input where sequential solution is faster
        ];
        for input in &inputs {
            let input_contents = read_to_string(input)?;

            let original_result = original_solution(&input_contents);
            let result = solution(&input_contents);
            assert_eq!(original_result, result);
        }
        Ok(())
    }

    fn original_solution(input_contents: &str) -> u32 {
        let mut longest_line = 0;
        let mut cur_line_length = 0;
        for s in input_contents.chars() {
            longest_line = max(cur_line_length, longest_line);
            cur_line_length = if s == '\n' { 0 } else { cur_line_length + 1 };
        }

        // if no end-of-line in the end
        longest_line = max(cur_line_length, longest_line);
        return longest_line;
    }
}
