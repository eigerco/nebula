# Working by Forking

Just head over to the GitHub page and click the "Fork" button. It's just that simple. Once you've done that, you can use your favorite git client to clone your repo or just head straight to the command line:

## Clone your fork to your local machine

```
git clone git@github.com:eigerco/nebula.git
```

Keeping Your Fork Up to Date
While this isn't an absolutely necessary step, if you plan on doing anything more than just a tiny quick fix, you'll want to make sure you keep your fork up to date by tracking the original "upstream" repo that you forked. To do this, you'll need to add a remote:

## Add 'upstream' repo to list of remotes

```
git remote add upstream https://github.com/eigerco/nebula.git
```

## Verify the new remote named 'upstream'

```
git remote -v
```

Whenever you want to update your fork with the latest upstream changes, you'll need to first fetch the upstream repo's branches and latest commits to bring them into your repository:

## Fetch from upstream remote

```
git fetch upstream
```

## View all branches, including those from upstream

```
git branch -va
```

Now, checkout your own main branch and merge the upstream repo's main branch:

## Checkout your main branch and merge upstream

```
git checkout main
git merge upstream/main
```

If there are no unique commits on the local main branch, git will simply perform a fast-forward. However, if you have been making changes on main (in the vast majority of cases you probably shouldn't be - see the next section, you may have to deal with conflicts. When doing so, be careful to respect the changes made upstream.

Now, your local main branch is up-to-date with everything modified upstream.

**Create a Branch** (doing your work)
Whenever you begin work on a new feature or bugfix, it's important that you create a new branch. Not only is it proper git workflow, but it also keeps your changes organized and separated from the main branch so that you can easily submit and manage multiple pull requests for every task you complete.

To create a new branch and start working on it, peform the following flow.

## Checkout the main branch - you want your new branch to come from main

```
git checkout main
```

## Create a new branch (give your branch its own simple informative name)

For enhancements use `feature/your_username/issue#` or `feature/your_username/name_of_feature`

For bugs use `bug/your_username/issue#` or `bug/your_username/name_of_bug`

If you belong to the @eigerco organisation, you can omit the username part eg `feature/name_of_feature`

Here are some common categories:

| Category Word | Meaning                                                               |
| ------------- | --------------------------------------------------------------------- |
| hotfix        | for quickly fixing critical issues, usually with a temporary solution |
| bugfix        | for fixing a bug                                                      |
| feature       | for adding, removing or modifying a feature                           |
| test          | for experimenting something which is not an issue                     |
| wip           | for a work in progress                                                |

```
git branch feature/jdoe/567
```

## Switch to your new branch

```
git checkout feature/jdoe/567
```

Now, go to town hacking away and making whatever changes you want to.

### Commit Messages

Use the following headers when committing

```
add: (message) - used for adding files/features/etc.
remove:        - used for removing features or (temporarily) files
update:        - for things like package.json, cargo.toml dependency updates
create:        - for new files
delete:        - for permanent deletion
lint:
style:         - cosmetic changes
refactor:      - re-writing code
fix:           - fixing errors
```

For the message, please use _present_ tense with no 3rd-person -s or continuous -ing suffixes. Meaning:

`add: grunt task` is OK.
`adds: grunt task` is NOT OK.
`added: grunt task` is NOT OK.
`adding: grunt task` is NOT OK.

## Submitting your changes (a Pull Request)

Prior to submitting your pull request, you might want to do a few things to clean up your branch and make it as simple as possible for the original repo's maintainer to test, accept, and merge your work.

In the time that you've been working on your changes, if any commits have been made to the upstream main branch, you will need to rebase your development branch so that merging it will be a simple fast-forward that won't require any conflict resolution work.

## Fetch upstream main and merge with your repo's main branch

```
git fetch upstream
git checkout main
git merge upstream/main
```

## If there were any new commits, rebase your development branch

```
git checkout feature/jdoe/567
git rebase main
```

Now, it may be desirable to squash some of your smaller commits down into a small number of larger more cohesive commits. You can do this with an interactive rebase:

## Rebase all commits on your development branch

```
git checkout
git rebase -i main
```

This will open up a text editor where you can specify which commits to squash.

## Submitting

Once you've committed and pushed all of your changes to GitHub, go to the page for your fork on GitHub, select your development branch, and click the pull request button. If you need to make any adjustments to your pull request, just push the updates to GitHub. Your pull request will automatically track the changes on your development branch and update.
