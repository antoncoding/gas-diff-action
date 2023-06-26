use std::collections::BTreeMap;

/// Parses the gas snapshot file and returns a map of test name to gas usage.
/// Use BTreeMap to preserve the order of the tests.
/// ```
/// TestSuite1:test_case1() (gas: 123)
/// TestSuite1:test_case2() (gas: 456)
/// TestSuite2:test_case1() (gas: 789)
/// TestSuite3:test_fuzz_blah(uint8,uint8) (runs: 256, μ: 5266, ~: 5271)
/// ```
fn parse_gas_snapshot_file(file_path: &str) -> BTreeMap<String, i32> {
    let content = std::fs::read_to_string(file_path).expect("Could not read file");
    let lines: Vec<&str> = content.lines().collect();

    let mut gas_usage = BTreeMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" (").collect();
        if parts.len() != 2 {
            continue;
        }

        let test_name = parts[0].to_string();
        let gas_info_parts: Vec<&str> = parts[1].split(": ").collect();

        // if it's a fuzz test, skip it
        if gas_info_parts.len() != 2 {
            continue;
        }

        let gas_cost = gas_info_parts[1].trim_end_matches(')').parse::<i32>().expect("wrong format: gas is not a number");
        gas_usage.insert(test_name, gas_cost);
    }

    gas_usage
}

fn compare_gas_snapshots(base_file: &str, pr_file: &str) -> String {
    let base_gas_usage = parse_gas_snapshot_file(base_file);
    let pr_gas_usage = parse_gas_snapshot_file(pr_file);

    let mut comparison = String::from("Gas usage comparison:\n\n| Suite | Case | Gas Diff | Percentage Change |\n| --- | --- | --- | --- |\n");
    let mut has_diff = false;
    let mut prev_suite = String::new();

    for (test_name, base_gas) in base_gas_usage {
        let pr_gas = match pr_gas_usage.get(&test_name) {
            Some(gas) => *gas,
            None => continue,
        };

        let diff = pr_gas - base_gas;
        if diff == 0 {
            continue;
        }

        has_diff = true;
        let percentage_change = (diff as f64 / base_gas as f64) * 100.0;
        let emoji = if diff < 0 { ":recycle:" } else { ":fuelpump:" };
        let diff_sign = if diff > 0 { "+" } else { "" };

        let parts: Vec<&str> = test_name.split(":").collect();
        let suite = parts[0];
        let case = parts[1][parts[1].to_lowercase().find("test").unwrap()..].replace("()", "");

        let suite_final = if suite == prev_suite {
            String::new()
        } else {
            prev_suite = suite.to_string();
            format!("`{}`", suite)
        };

        comparison += &format!(
            "| {} | `{}` | {} {}{} | {:.2}% |\n",
            suite_final, case, emoji, diff_sign, diff, percentage_change
        );
    }

    if has_diff {
        comparison
    } else {
        String::from("Gas usage not changed!")
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let base_file = &args[1];
    let pr_file = &args[2];
    println!("{}", compare_gas_snapshots(base_file, pr_file));
}
