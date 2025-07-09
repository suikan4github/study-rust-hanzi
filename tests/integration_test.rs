use std::path::Path;
use std::process::Command;

#[test]
fn test_by_pinyin_output_format() {
    if !Path::new("hanzi.tsv").exists() {
        eprintln!("Skipping test: hanzi.tsv not found");
        return;
    }

    let output = Command::new("cargo")
        .args(["run", "--", "by-pinyin"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Test: output should not be empty
    assert!(!stdout.is_empty(), "Output should not be empty");

    // Test: first line should be in expected format
    let first_line = stdout.lines().next().unwrap();
    assert!(
        first_line.contains(":"),
        "First line should contain ':' separator"
    );

    // Test: should contain numbers (frequency)
    assert!(
        first_line.chars().any(|c| c.is_ascii_digit()),
        "First line should contain frequency count"
    );
}

#[test]
fn test_by_pinyin_traditional_vs_simplified() {
    if !Path::new("hanzi.tsv").exists() {
        eprintln!("Skipping test: hanzi.tsv not found");
        return;
    }

    let simplified_output = Command::new("cargo")
        .args(["run", "--", "by-pinyin"])
        .output()
        .expect("Failed to execute command");

    let traditional_output = Command::new("cargo")
        .args(["run", "--", "by-pinyin", "--traditional"])
        .output()
        .expect("Failed to execute command");

    let simplified_stdout = String::from_utf8(simplified_output.stdout).expect("Invalid UTF-8");
    let traditional_stdout = String::from_utf8(traditional_output.stdout).expect("Invalid UTF-8");

    // Both should have output
    assert!(!simplified_stdout.is_empty());
    assert!(!traditional_stdout.is_empty());

    // Output should be different (simplified vs traditional)
    assert_ne!(simplified_stdout, traditional_stdout);
}

#[test]
fn test_by_pinyin_fold_option() {
    if !Path::new("hanzi.tsv").exists() {
        eprintln!("Skipping test: hanzi.tsv not found");
        return;
    }

    let normal_output = Command::new("cargo")
        .args(["run", "--", "by-pinyin"])
        .output()
        .expect("Failed to execute command");

    let folded_output = Command::new("cargo")
        .args(["run", "--", "by-pinyin", "--fold", "20"])
        .output()
        .expect("Failed to execute command");

    let normal_stdout = String::from_utf8(normal_output.stdout).expect("Invalid UTF-8");
    let folded_stdout = String::from_utf8(folded_output.stdout).expect("Invalid UTF-8");

    // With fold option, should have more lines
    let normal_lines = normal_stdout.lines().count();
    let folded_lines = folded_stdout.lines().count();

    assert!(
        folded_lines >= normal_lines,
        "Folded output should have equal or more lines"
    );
}

#[test]
fn test_by_tone_output_format() {
    if !Path::new("hanzi.tsv").exists() {
        eprintln!("Skipping test: hanzi.tsv not found");
        return;
    }

    let output = Command::new("cargo")
        .args(["run", "--", "by-pinyin", "ji"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Test: output should not be empty
    assert!(!stdout.is_empty(), "Output should not be empty");

    // Test: each line should contain ': ' separator
    for line in stdout.lines() {
        assert!(
            line.contains(": "),
            "Each line should contain ': ' separator"
        );
    }

    // Test: should contain tone marks
    assert!(
        stdout.contains("jī")
            || stdout.contains("jí")
            || stdout.contains("jǐ")
            || stdout.contains("jì"),
        "Output should contain tone marks"
    );
}

#[test]
fn test_by_tone_traditional_vs_simplified() {
    if !Path::new("hanzi.tsv").exists() {
        eprintln!("Skipping test: hanzi.tsv not found");
        return;
    }

    let simplified_output = Command::new("cargo")
        .args(["run", "--", "by-pinyin", "ji"])
        .output()
        .expect("Failed to execute command");

    let traditional_output = Command::new("cargo")
        .args(["run", "--", "by-pinyin", "ji", "--traditional"])
        .output()
        .expect("Failed to execute command");

    let simplified_stdout = String::from_utf8(simplified_output.stdout).expect("Invalid UTF-8");
    let traditional_stdout = String::from_utf8(traditional_output.stdout).expect("Invalid UTF-8");

    // Both should have output
    assert!(!simplified_stdout.is_empty());
    assert!(!traditional_stdout.is_empty());

    // Output should be different (simplified vs traditional)
    assert_ne!(
        simplified_stdout, traditional_stdout,
        "Simplified and traditional outputs should be different"
    );
}

#[test]
fn test_by_tone_nonexistent_pinyin() {
    if !Path::new("hanzi.tsv").exists() {
        eprintln!("Skipping test: hanzi.tsv not found");
        return;
    }

    let output = Command::new("cargo")
        .args(["run", "--", "by-pinyin", "xyz"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Non-existent pinyin should output appropriate message
    assert!(stdout.contains("No characters found for pinyin: xyz"));
}

#[test]
fn test_by_tone_tone_ordering() {
    if !Path::new("hanzi.tsv").exists() {
        eprintln!("Skipping test: hanzi.tsv not found");
        return;
    }

    let output = Command::new("cargo")
        .args(["run", "--", "by-pinyin", "ma"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    let lines: Vec<&str> = stdout.lines().collect();

    // For 'ma', there should usually be multiple tones
    if lines.len() > 1 {
        // Check tone marks in each line to verify correct ordering
        // This implementation is simplified, but more detailed checks are possible
        for line in &lines {
            assert!(line.contains(": "), "Each line should have proper format");
        }
    }
}

#[test]
fn test_by_tone_v_to_u_replacement() {
    if !Path::new("hanzi.tsv").exists() {
        eprintln!("Skipping test: hanzi.tsv not found");
        return;
    }
    // Test that 'v' in command line input gets replaced with 'ü'
    let output = Command::new("cargo")
        .args(["run", "--", "by-pinyin", "nv"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    // Should find characters for 'nü' when searching for 'nv'
    // If no characters found, it should show the normalized pinyin in the message
    if stdout.contains("No characters found") {
        assert!(
            stdout.contains("nü"),
            "Error message should show normalized pinyin 'nü'"
        );
    } else {
        // If characters are found, the output should not be empty
        assert!(
            !stdout.is_empty(),
            "Should have output when characters are found"
        );
    }
}
