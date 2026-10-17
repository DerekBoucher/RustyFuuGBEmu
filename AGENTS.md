# AGENTS.md

## Contribution guidelines
- When starting work on a task, open a new git branch from `main`. The branch should respect the following naming conventions:
    * <TASK_TYPE>/<UNIX_TIMESTAMP>_<SHORT_DESCRIPTION>
    * `TASK_TYPE` is one of the following: BUGFIX, FEATURE, REFACTOR. Use your best judgement as to what the task type should be for a given requirement.
    * `UNIX_TIMESTAMP` is the unix timestamp at the moment of branch creation
    * `SHORT_DESCRIPTION` is an arbitrary, concise description of the requirement for the task, using underscores as word delimiters.
- When making a change, do not commit them. I want to be able to review the diffs afterwards.
