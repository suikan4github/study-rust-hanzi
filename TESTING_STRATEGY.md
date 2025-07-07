# Testing Strategy for process_by_pinyin() and process_by_tone()

This document outlines a comprehensive approach for testing the `process_by_pinyin()` and `process_by_tone()` functions.

## 1. Unit Testing through Refactoring

### process_by_pinyin() Related Functions

#### a) `group_by_pinyin()` - Data Grouping Logic
```rust
pub fn group_by_pinyin(records: &[HanziRecord], use_traditional: bool) -> Vec<(String, Vec<String>)>
```

**Testable aspects:**
- Correct frequency-based sorting (descending frequency, then ascending pinyin)
- Simplified/Traditional character selection logic
- Data grouping accuracy

#### b) `format_pinyin_output()` - Output Formatting Logic
```rust
pub fn format_pinyin_output(grouped_data: &[(String, Vec<String>)], fold_size: Option<usize>) -> Vec<String>
```

**Testable aspects:**
- Output format accuracy (alignment, colon separation)
- Fold functionality behavior
- Character count accuracy

### process_by_tone() Related Functions

#### c) `group_by_tone()` - Tone-based Data Grouping Logic
```rust
pub fn group_by_tone(records: &[HanziRecord], target_pinyin: &str, use_traditional: bool) -> Option<Vec<(u32, String, Vec<String>)>>
```

**Testable aspects:**
- Filtering for specified pinyin
- Tone-based grouping accuracy
- Tone order sorting (1, 2, 3, 4, 5)
- Simplified/Traditional character selection logic
- Non-existent pinyin handling (returning None)

#### d) `format_tone_output()` - Tone-based Output Formatting
```rust
pub fn format_tone_output(tone_groups: &[(u32, String, Vec<String>)]) -> Vec<String>
```

**Testable aspects:**
- Output format accuracy (pinyin: characters)
- Display of pinyin with tone marks
- String concatenation accuracy

### Unit Test Examples
```rust
#[test]
fn test_group_by_pinyin_simplified() {
    let records = create_test_records();
    let grouped = group_by_pinyin(&records, false);
    
    assert_eq!(grouped[0].0, "ji");
    assert_eq!(grouped[0].1, vec!["机", "计"]);
}

#[test]
fn test_group_by_tone_existing_pinyin() {
    let records = create_test_records();
    let result = group_by_tone(&records, "ji", false);
    
    assert!(result.is_some());
    let groups = result.unwrap();
    assert!(groups.len() > 0);
    // Verify tone ordering
    for i in 1..groups.len() {
        assert!(groups[i-1].0 <= groups[i].0);
    }
}

#[test]
fn test_format_pinyin_output_with_fold() {
    let test_data = vec![
        ("test".to_string(), vec!["一".to_string(), "二".to_string(), "三".to_string()]),
    ];
    
    let output = format_pinyin_output(&test_data, Some(2));
    assert!(output.len() >= 2, "Should fold long lines");
}
```
```

## 2. Integration Testing

### Command-line Output Testing
```rust
#[test]
fn test_by_pinyin_output_format() {
    let output = Command::new("cargo")
        .args(&["run", "--", "by-pinyin"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(!stdout.is_empty());
    assert!(stdout.lines().next().unwrap().contains(":"));
}

#[test]
fn test_by_tone_output_format() {
    let output = Command::new("cargo")
        .args(&["run", "--", "by-tone", "ma"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(!stdout.is_empty());
    
    // Check tone mark format
    let lines: Vec<&str> = stdout.lines().collect();
    for line in lines {
        assert!(line.contains(":"), "Each line should contain ':'");
        // Should start with pinyin with tone marks
        assert!(line.chars().next().unwrap().is_alphabetic());
    }
}
```

### Feature Comparison Testing
```rust
#[test]
fn test_by_pinyin_traditional_vs_simplified() {
    // Compare simplified and traditional character output
    let simplified_output = run_command(&["by-pinyin"]);
    let traditional_output = run_command(&["by-pinyin", "--traditional"]);
    
    assert_ne!(simplified_output, traditional_output);
}

#[test]
fn test_by_tone_traditional_vs_simplified() {
    let simplified_output = run_command(&["by-tone", "ma"]);
    let traditional_output = run_command(&["by-tone", "ma", "--traditional"]);
    
    assert_ne!(simplified_output, traditional_output);
}

#[test]
fn test_by_tone_nonexistent_pinyin() {
    let output = Command::new("cargo")
        .args(&["run", "--", "by-tone", "nonexistent"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert!(stdout.contains("No characters found"));
}
```

## 3. Property-based Testing (Recommended Additional Implementation)

### QuickCheck-style Approach
```rust
#[test]
fn test_pinyin_grouping_properties() {
    // For random HanziRecord sets:
    // - Total character count after grouping matches original count
    // - Frequency order is correctly maintained
    // - Same pinyin doesn't appear in multiple groups
}

#[test]
fn test_tone_grouping_properties() {
    // For valid pinyin inputs:
    // - All returned characters have the specified pinyin_without_tone
    // - Tone groups are properly ordered (1, 2, 3, 4, 5)
    // - No duplicate characters within the same tone group
}
```

## 4. Performance Testing

### Large Dataset Testing
```rust
#[test]
fn test_performance_with_large_dataset() {
    let start = std::time::Instant::now();
    let grouped = group_by_pinyin(&large_records, false);
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_secs(1));
    assert!(!grouped.is_empty());
}

#[test]
fn test_tone_performance() {
    let start = std::time::Instant::now();
    let result = group_by_tone(&large_records, "ji", false);
    let duration = start.elapsed();
    
    assert!(duration < std::time::Duration::from_millis(100));
    assert!(result.is_some());
}
```

## 5. Error Handling Testing

### File Reading Error Testing
```rust
#[test]
fn test_process_by_pinyin_file_not_found() {
    // Test behavior with non-existent files
    // Note: process_by_pinyin() currently calls std::process::exit(1),
    // this part could be refactored to be more testable
}

#[test]
fn test_invalid_hanzi_data() {
    // Test with malformed TSV data
    // Test with missing columns
    // Test with invalid character encodings
}
```

### Edge Case Testing
```rust
#[test]
fn test_empty_dataset() {
    let empty_records = vec![];
    let grouped = group_by_pinyin(&empty_records, false);
    assert!(grouped.is_empty());
    
    let tone_result = group_by_tone(&empty_records, "ji", false);
    assert!(tone_result.is_none());
}

#[test]
fn test_single_character_dataset() {
    let single_record = vec![create_single_test_record()];
    let grouped = group_by_pinyin(&single_record, false);
    assert_eq!(grouped.len(), 1);
}
```

## 6. Regression Testing

### Golden File Testing
```rust
#[test]
fn test_output_regression() {
    let output = format_pinyin_output(&fixed_test_data, None);
    let expected = include_str!("../test_data/expected_output.txt");
    assert_eq!(output.join("\n"), expected.trim());
}

#[test]
fn test_tone_output_regression() {
    let tone_data = create_fixed_tone_test_data();
    let output = format_tone_output(&tone_data);
    let expected = include_str!("../test_data/expected_tone_output.txt");
    assert_eq!(output.join("\n"), expected.trim());
}
```

### Snapshot Testing
```rust
#[test]
fn test_pinyin_output_snapshot() {
    // Use insta crate for snapshot testing
    let output = format_pinyin_output(&standard_test_data, None);
    insta::assert_snapshot!(output.join("\n"));
}
```

## 7. Test Execution

### Run All Tests
```bash
cargo test
```

### Run Specific Tests
```bash
cargo test test_group_by_pinyin
cargo test test_group_by_tone
cargo test integration_test
```

### Run Tests by Category
```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_test
```

### Coverage Analysis
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Continuous Integration
```yaml
# .github/workflows/test.yml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --all-features
```
## Summary

### Benefits
1. **Unit Testing**: Component isolation allows independent testing of each part
2. **Integration Testing**: Validates actual command-line behavior
3. **Maintainability**: Ensures existing functionality works after refactoring
4. **Debugging**: Easy identification of problem sources when issues occur
5. **Documentation**: Tests serve as living documentation of expected behavior
6. **Confidence**: Comprehensive test coverage provides confidence in changes

### Current Test Implementation Status
```
Unit Tests (src/main.rs):
✓ test_format_pinyin_output_alignment ... ok
✓ test_format_pinyin_output_no_fold ... ok  
✓ test_format_pinyin_output_with_fold ... ok
✓ test_group_by_pinyin_simplified ... ok
✓ test_group_by_pinyin_traditional ... ok
✓ test_group_by_tone_existing_pinyin ... ok
✓ test_group_by_tone_nonexistent_pinyin ... ok
✓ test_format_tone_output_basic ... ok
✓ test_tone_sorting_order ... ok

Integration Tests (tests/integration_test.rs):
✓ test_by_pinyin_output_format ... ok
✓ test_by_pinyin_traditional_vs_simplified ... ok
✓ test_by_pinyin_fold_option ... ok
✓ test_by_tone_output_format ... ok
✓ test_by_tone_traditional_vs_simplified ... ok
✓ test_by_tone_nonexistent_pinyin ... ok
✓ test_by_tone_tone_order ... ok

Library Tests (src/lib.rs):
✓ test_read_hanzi_file_length ... ok
✓ test_read_hanzi_file_tenth_element ... ok
✓ test_read_hanzi_file_last_element ... ok
✓ test_set_hanzi_onsets ... ok
✓ test_set_hanzi_rime ... ok
```

### Test Coverage Areas
- ✅ **Data Processing Logic**: Grouping, sorting, filtering
- ✅ **Output Formatting**: Alignment, folding, tone marks
- ✅ **Character Selection**: Simplified vs Traditional
- ✅ **Command-line Interface**: All subcommands and options
- ✅ **Edge Cases**: Empty data, non-existent pinyin
- ✅ **Integration**: End-to-end functionality testing

### Recommended Improvements
1. **Property-based Testing**: Add randomized input testing
2. **Performance Benchmarks**: Measure and track performance over time
3. **Error Injection**: Test network failures, disk full scenarios
4. **Fuzz Testing**: Test with malformed input data
5. **Golden File Testing**: Maintain expected output snapshots
6. **Mutation Testing**: Verify test quality by introducing bugs

These comprehensive tests ensure reliable functionality across all features of both `process_by_pinyin()` and `process_by_tone()` functions, providing confidence in the codebase for both current functionality and future changes.
