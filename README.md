# Cli Tool to run Otter Suite tests

## Usage

```bash
suite-cli --help
```

### Commands

```bash
suite-cli [command] 
```

- `verify`      Verifies an onchain program aganist given repo
- `create-task`  Creates a process to run given tasks on the repo
- `help`         Print this message or the help of the given subcommand(s)

#### Verify

```bash
suite-cli verify -b <BLOCKCHAIN> -r <GIT_REMOTE> -c <GIT_COMMIT> -p <PROGRAM_ID>
```

- `-b, --blockchain <BLOCKCHAIN>`  Blockchain to use (available: solana, ethereum, aptos, sui)
- `-r, --remote <GIT_REMOTE>`      Git remote to use
- `-c, --commit <GIT_COMMIT>`      Git commit to use
- `-p, --program <PROGRAM_ID>`     Program id/address of the program deployed on the blockchain.

#### Create Task

```bash
suite-cli create-task <BLOCKCHAIN> -r <GIT_REMOTE> -c <GIT_COMMIT> -t <TASKS>
```

- `<BLOCKCHAIN>`  Blockchain to use (available: solana, ethereum, aptos, sui)
- `-r, --remote <GIT_REMOTE>`      Git remote to use
- `-c, --commit <GIT_COMMIT>`      Git commit to use
- `-t, --tasks <TASKS>`            Tasks to run.
  - None to run all tasks
  - Comma separated list of tasks to run
  - `suite-cli create-task <BLOCKCHAIN> --tasks` or `suite-cli create-task <BLOCKCHAIN> -t` to see all available tasks for given blockchain
