# Ziom - an OpenAI-powered commiting assistant

A CLI tool for generating relevant commit messages for the staged changes.

# Installation

To be able to use `ziom` in any git repository, clone the repo, build it using 
```shell
cargo build --release
```
And then put the resulting binary in your `/usr/local/bin` folder.


# How to use?
![Kapture 2023-03-27 at 09 59 42](https://user-images.githubusercontent.com/46461547/227882718-973a7a09-754e-4e8c-aca6-081586099377.gif)

Before using a tool itself you need to set up your OpenAI API key. To do this use:

```shell
ziom config <YOUR:API:KEY>
```

To confirm that your key was set up correctly use:

```shell
ziom config
```

which should show you API key in a terminal.

To generate a commit message for your staged changes use

```shell
ziom generate
```

while being in a proper location (please make sure that you're using this tool inside of your project directory and you have git set up correctly).


# Future improvements

- [ ] Improve response parsing
- [ ] Assert that the current CLI's location is a valid git repository
- [ ] Add an option to add project's README file to prompt. This should give an API more context and generate more accurate results.
- [ ] Clean up the code.
- [ ] Come up with a proper error handling.
- [ ] Add the possibility to edit a message in CLI, copy to clipboard on edit end.
- [ ] Auto-commit
- [ ] simple UI and awaiting indicator
- [ ] Instead of running a proccess for a while when generating a message, keep a process running in the background - questionable

#

Already kinda deprecated because of the Copilot lmao. Feel free to open issues and pull requests though.
