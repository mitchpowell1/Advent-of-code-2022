#! /opt/homebrew/bin/python3
import argparse
import urllib.request
import urllib.error
import os.path
from datetime import datetime
from os import getenv
from sys import version_info, path

SESSION_COOKIE_ENV_VAR = "AOC_SESSION_COOKIE"
CARGO_TOML_BINARY_SECTION_DELIMITER = "### END BINS ###\n"

def get_input_url(day: int, year:int):
    return f"https://adventofcode.com/{year}/day/{day}/input"

def fetch_input(day: int, year: int, session_cookie: str):
    url = get_input_url(day, year)
    headers = { "cookie": f"session={session_cookie}" }
    req = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(req) as res:
        day_input = res.read().decode('UTF-8')

    file_name = f"{path[0]}/inputs/day{day}_input.txt"
    with open(file_name, 'w') as f:
        f.write(day_input.rstrip('\n'))

def add_source_file(day: int):
    filepath = f"{path[0]}/src/day{day}.rs"
    if os.path.exists(filepath):
        print("File already exists, skipping source file creation")
        return

    challenge_file_template = f'''use std::fs;

const FILE_PATH: &str = "inputs/day{day}_input.txt";

fn main() {{
    let contents = fs::read_to_string(FILE_PATH).expect("Could not read input for day{day}");
    let parsed = contents.trim();
    println!("{{parsed}}");
}}
'''
    with open(filepath, 'w') as f:
        f.write(challenge_file_template)


def update_project_file(day: int):
    with open(f"{path[0]}/Cargo.toml", "r") as f:
        cargo_file_contents = f.readlines()

    if f'name = "day{day}"\n' in cargo_file_contents:
        print("Skipping project file modification")
        return
    delimeter_index = cargo_file_contents.index(CARGO_TOML_BINARY_SECTION_DELIMITER)
    before_delimeter = cargo_file_contents[0:delimeter_index]
    after_delimeter = cargo_file_contents[delimeter_index:]
    to_insert = [
        "[[bin]]\n",
        f"name = \"day{day}\"\n",
        f"path = \"src/day{day}.rs\"\n",
        "\n",
    ]
    with open(f"{path[0]}/Cargo.toml", "w") as f:
        f.writelines(before_delimeter)
        f.writelines(to_insert)
        f.writelines(after_delimeter)

def validate_py_version():
    if version_info.major < 3 or version_info.minor < 6:
        print("This script uses format strings, please use python version >= 3.6\n\n")
        print("...or modify the script or something, I'm not your boss")
        exit(1)

def main():
    validate_py_version()
    parser = argparse.ArgumentParser()
    parser.add_argument("--day", type=int)
    parser.add_argument("--year", type=int, default=2022)
    args = parser.parse_args()
    cookie_value = getenv(SESSION_COOKIE_ENV_VAR)
    date = datetime.now();
    if not cookie_value:
        print(f"In order to fetch your input you will need to populate the {SESSION_COOKIE_ENV_VAR} environment variable with your session cookie.")
        print("For details on how to get this value see the following: https://github.com/wimglenn/advent-of-code-wim/issues/1#issue-193321235")
        exit(1)
    day = args.day
    year = args.year
    if not day:
        day = date.day
    if not year:
        year = date.year

    fetch_input(day, year, cookie_value)
    add_source_file(day)
    update_project_file(day)

if __name__ == '__main__':
    main()
