import sys

def parse_gas_snapshot_file(file_path):
    with open(file_path, "r") as f:
        lines = f.readlines()

    gas_usage = {}
    for line in lines:
        test_name, gas = line.strip().split(" (gas: ")
        gas_usage[test_name] = int(gas[:-1])

    return gas_usage

def compare_gas_snapshots(base_file, pr_file):
    base_gas_usage = parse_gas_snapshot_file(base_file)
    pr_gas_usage = parse_gas_snapshot_file(pr_file)

    comparison = "Gas usage comparison:\n```\n"
    for test_name, base_gas in base_gas_usage.items():
        pr_gas = pr_gas_usage.get(test_name, None)
        if pr_gas is not None:
            diff = pr_gas - base_gas
            sign = "+" if diff > 0 else "-"
            comparison += f"{test_name}: {sign}{diff}\n"
    comparison += "```\n"

    return comparison

if __name__ == "__main__":
    base_file = sys.argv[1]
    pr_file = sys.argv[2]
    print(compare_gas_snapshots(base_file, pr_file))