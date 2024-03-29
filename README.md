# Gas Diff Action

Gas Diff Action is a GitHub Action that compares gas usage in Solidity projects using Foundry and comments the changes on the PR.

## Preview

![](./docs/imgs/screenshot.png)

## Usage

To integrate Gas Diff Action into your project, follow these steps:

1. Create a `.github/workflows` directory in your repository if it doesn't already exist.
2. Inside the `workflows` directory, create a new file called `gas_comparison.yml`.
3. Add the following content to the `gas_comparison.yml` file:

```yaml
name: Gas Comparison

on:
  pull_request:
    types:
      - opened
      - synchronize

jobs:
  gas_comparison:
    runs-on: ubuntu-latest

    steps:
      - name: Run Gas Comparison
        uses: antoncoding/gas-diff-action@v2.0.1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          base_ref: ${{ github.base_ref }}
          head_ref: ${{ github.head_ref }}
```

Commit and push your changes to the repository.
Now, Gas Diff Action will automatically compare gas usage for Solidity contracts in your repository and comment the changes on the PR whenever a pull request is opened or updated.

## Contributing

Contributions to Gas Diff Action are welcome! Please submit issues for bug reports or feature requests and create pull requests for any improvements or bug fixes.

## License

Gas Diff Action is released under the MIT License.