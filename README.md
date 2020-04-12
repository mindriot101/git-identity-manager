# Git identity manager

This configures git to use an identity per local repository.

The idea is that the gitconfig enforces the identity to be chosen on a
project-by-project basis. Enforce this by removing the `user.name` and
`user.email` settings in your gitconfig, and replacing them with:

```conf
[user]
    useConfigOnly = true
```

Then add an identity with e.g.

```
git identity add --id github.personal --name "My Name" --email "test@example.com" [--signing-key "4AJGF02"] [--ssh-key ~/.ssh/id_rsa]
```

Identities can be listed with

```
git identity list
```

## Selecting a git identity

For each project, git will not let you commit until you have chosen an identity.
To do this, ensure that an identity is set up in your global `.gitconfig`, for
example the line above adds the following to your `.gitconfig`:

```
[user "github.personal"]
    name = My Name
    email = test@example.com
    signingkey = 4AJGF02
    sshkey = ~/.ssh/id_rsa
```

Then the `github.personal` identity is available for selection.

Use `git identity set` in a project directory to interactively choose an
identity.

An identity can be removed using `git identity remove`.

## Generating shell completion

The command `git identity gen-completion -s <shell>` can be used to generate the
correct auto-completion for your shell. It prints the completion information to
stdout, so consult your shell's documentation for information on how to
integrate this.

## Tools used

* `rust`
* [`Skim`](https://crates.io/crates/skim) for fuzzy finding interface
* [`git2`](https://crates.io/crates/git2) for interacting with git repositories
