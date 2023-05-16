# Cli Tool to run Otter Suite tests

## Installation

```bash
cargo install --git https://github.com/otter-dev/otter-cli
```

## Usage

To use Otter CLI we need to Auth ourself: (You need to be whitelisted contact Osec team if any help needed here)

1. Open the command prompt or terminal on your computer.
2. Run `otr` and select `authenticate`
3. Open the given link and Enter your code and give permission for application.
4. Once the authentication is completed we can now submit jobs to Otter API via CLI.

## Creating and Retrieving Jobs

1. Now open `otr` and select `CreateTask`
2. Select blockchain as Solana.
3. Input the required arguments for the Job and select which tasks to run on this particular job.
4. After submitting this you will get a `jobId` we need this to retrieve the status of the job.
5. Run `otr` and select `GetTask` and enter the Job Id we got to get the results of the Job.
