# Cli Tool to run Otter Suite tests

## Installation

```bash
cargo install --git https://github.com/otter-dev/otter-cli
```

## To update

```bash
cargo install --git https://github.com/otter-dev/otter-cli --force
```

## Usage

To use Otter CLI we need to authenticate ourself: (You need to be whitelisted contact Osec team if you need any help here)

1. Open the command prompt or terminal on your computer.
2. Run `otr -h` or `otr --help` to see all the available commands.

## Authentication

- When prompted open the given link and Enter your code and give permission for application.
- Once the authentication is completed we can now submit jobs to Otter API via CLI.

## Submitting a Job

1. Use `ote solana-verify` and pass the required arguments to know more use `otr solana-verify --help`.
2. Input the required arguments for the Job `-r` or `--repository` for git repository, `-b` for git branch and `-p` for program path.
3. After submitting this you will get a `jobId` we need this to retrieve the status of the job.

## Checking the status of a Job

1. Use `otr check` and pass the required arguments to know more use `otr check --help`.
2. Input the required arguments for the Job `otr check <ID>`.
