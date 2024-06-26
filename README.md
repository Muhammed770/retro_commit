# Random Git Commit Generator

This Rust program automates the creation of random Git commits over a specified number of days.

## Features

- Initializes a new Git repository if not already initialized.
- Prompts for GitHub username, email, number of commits, and number of days.
- Generates random commit dates within the specified range.
- Creates commits with the generated dates.
- Configures remote Git repository if not already set.

## Prerequisites

- Rust and Cargo installed
- Git installed

## Usage

1. Clone the repository:

```sh
   git clone https://github.com/Muhammed770/retro_commit.git
   cd retro_commit
```
2. Build and run the program:

```sh
cargo build
cargo run 
```
Follow the prompts to enter your GitHub username, email, number of commits, and number of days.

Example
```sh
user.name: default_username
user.email: default_mail@example.com
Do you want to change DEFAULT user.name and user.email? (Y/N)
Y
Enter username: muhammed770
Please enter email id linked to your GitHub: muhammed770@example.com
Please enter number of commits: 10
Please enter number of days: 30
```
The program will generate 10 random commits over the past 30 days.

## License
This project is licensed under the MIT License.
