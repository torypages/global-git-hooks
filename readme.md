# global-git-hooks

This project allows for the creation of git hooks that will apply to all repos
on your system.

The installation will alter the git global hooks path to point to this project
so that when hooks run this project will be run first. After running any code
from this project, the hooks local to project that the commit being executed on
will be called.

Commit for YourProject -> global-git-hooks hook is executed -> global-git-hooks
calls the hook of YourProject.

## Installation From the global-git-hooks directory run the following command:

```bash ./install.sh ```

## Uninstall Inspect the contents of uninstall.sh and make sure that you are
okay with it, then run it.

```bash ./uninstall.sh ```

## Notes
- I'm not a Rust developer, this is probably terrible code.
- There is only customization for commit-msg.
- The install could be modified to use `--local` when configuring the hooks
  path to only apply to a particular repository. At some point I might add
  project filters.
- It would be cool if this were more extensible.
- The project is only intended to scratch my itch of wanting:
  - Prepend commit messages with a jira ticket number found in the branch name.
  - Convert git commit messages of "-" to some generic message.
