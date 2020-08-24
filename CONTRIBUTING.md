# Contributing to Homework 2-3
Welcome to the CSC 510 fall 2020 repository for team z09. Please review the following guidelines for contributing to this project.

## Code of Conduct
This project and everyone participating in it is governed by the project [code of conduct](CODE-OF-CONDUCT.md). By participating in this project, you are expected to uphold this code. Please report unacceptable behavior to the instructor at timm@ieee.org.

## Git Remote Workflow
When adding a new feature to the project, work should be completed in a branch from the main repository, with a name that roughly indicates what its purpose is. To create a new branch on your local machine, run:  

`git checkout -b my_new_feature`

You can then make changes in your branch. When you are ready to add this branch to the remote, run:  

`git push -u origin my_new_feature`

You will only need to run this command the first time you want to commit your local changes to the remote branch. After the first commit, a simple `git push` from your repository root will push committed changes to the remote repository.

### Pull Requests
When a branch is ready to be merged into the main branch, the developer must submit a pull request (PR) using the GitHub web interface. The PR message should contain an overview of the changes made. This repository has been configured to require at least 1 approving review from a team member. Once the PR is submitted, please request a review by submitting the link to the PR in the team Slack channel.

## Development Style
### Git commit messages
- Use present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Keep commit message as short but as descriptive as possible
### Python
- Use the pep8 coding style (linter and formatters TBD)
- Every function that has an external scope should be documented with a [Google style docstring](https://google.github.io/styleguide/pyguide.html)

## Testing
TBD
