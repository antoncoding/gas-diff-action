import sys

def parse_gas_snapshot_file(file_path):
    with open(file_path, "r") as f:
        lines = f.readlines()

    gas_usage = {}
    for line in lines:
        try:
            # Split the line to extract the relevant information
            test_name, gas_info = line.strip().split(" (")
            gas_cost = int(gas_info.split(": ")[1].rstrip(")"))

            gas_usage[test_name] = gas_cost
        except ValueError:
            # Skip the line if it doesn't match the expected format
            pass

    return gas_usage

def compare_gas_snapshots(base_file, pr_file):
    base_gas_usage = parse_gas_snapshot_file(base_file)
    pr_gas_usage = parse_gas_snapshot_file(pr_file)

    comparison = ""
    has_diff = False
    for test_name, base_gas in base_gas_usage.items():
        pr_gas = pr_gas_usage.get(test_name, None)
        if pr_gas is not None:
            diff = pr_gas - base_gas
            if diff != 0:
                has_diff = True
                break

    if has_diff:
        comparison += "Gas usage comparison:\n\n"
        comparison += "| Test Name | Gas Diff | Percentage Change |\n"
        comparison += "|-----------|----------|-------------------|\n"
        for test_name, base_gas in base_gas_usage.items():
            pr_gas = pr_gas_usage.get(test_name, None)
            if pr_gas is not None:
                diff = pr_gas - base_gas
                if diff == 0:
                    continue
                percentage_diff = round((diff / base_gas) * 100, 2)
                sign = "+" if diff > 0 else ""
                comparison += f"| {test_name} | {sign}{diff} | {sign}{percentage_diff}% |\n"

    return comparison

if __name__ == "__main__":
    base_file = sys.argv[1]
    pr_file = sys.argv[2]
    print(compare_gas_snapshots(base_file, pr_file))
