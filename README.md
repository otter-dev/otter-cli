# Cli Tool to run Otter Suite tests

## Installation

```bash
cargo install --git https://github.com/otter-dev/otter-cli
```

## Usage

To use Otter CLI we need to Auth ourself: (You need to be whitelisted contact Osec team if any help needed here)

1. Open the command prompt or terminal on your computer.
2. Run `otr -i` for interactive mode or `otr -h` to see all the available commands.

## Authentication

- When prompted open the given link and Enter your code and give permission for application.
- Once the authentication is completed we can now submit jobs to Otter API via CLI.

## Creating and Retrieving Jobs via Interactive Mode

1. Use `otr -i` and select `CreateTask`
2. Select blockchain as Solana.
3. Input the required arguments for the Job and select which tasks to run on this particular job.
4. After submitting this you will get a `jobId` we need this to retrieve the status of the job.
5. Run `otr -i` and select `GetTask` and enter the Job Id we got to get the results of the Job.

## Creating and Retrieving Jobs via CLI

1. Use `otr create` and pass the required arguments.
2. To use blockchain as Solana we need to `otr create solana`.
3. Input the required arguments for the Job `-r` or `--remote` for git repository, `-c` for git commit and `-t` for comma seperated solana tasks.
    - Possible values for tasks: `cargo-build, cargo-clippy, overflow-check, build-bpf, verify-github-build-is-onchain, build-and-return-hash, formal-verification`
4. After submitting this you will get a `jobId` we need this to retrieve the status of the job.
5. Run `otr get -i` or `otr get --id` and pass `JobId` we got to get the results of the Job.

## Submit for Formal Verification

1. Use `ote verify` and pass the required arguments.
2. Input the required arguments for the Job `-r` or `--remote` for git repository, `-c` for git commit and `-p` for program path.
3. After submitting this you will get a `jobId` we need this to retrieve the status of the job.
