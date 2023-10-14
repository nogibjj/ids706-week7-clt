# Week 7: Package a Rust Script into a Command-Line Tool[![CI](https://github.com/nogibjj/ids706-week7-clt/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/ids706-week7-clt/actions/workflows/ci.yml)

## Project Description

Week 7:  (Students can do this in Rust as well)  
### Requirements 
- Package a Python script with setuptools or a similar tool
- Include a user guide on how to install and use the tool
- Include communication with an external or internal database (NoSQL, SQL, etc) [If you use Rust you can skip the DB part]
### Grading Criteria 
- Functionality of the tool (20 points)
- User guide clarity (20 points)    
### Deliverables     
- Python package
- User guide (PDF or markdown)  
You can delay turning this in until the following week so that Alfredo can guide through it

## `notekeeper` - Rust Command-Line Tool

`notekeeper` is a lightweight and straightforward command-line tool written in Rust, designed to assist you in adding, viewing, and deleting notes. While the current implementation stores notes in-memory (and hence they are not persistent), this tool serves as a foundation upon which additional features and integrations can be built.

## Getting Started

These instructions will help you get a copy of `notekeeper` up and running on your local machine.

### Installation

1. Clone the repository or download and extract the ZIP file.
   ```
   git clone https://github.com/nogibjj/ids706-week7-clt
   ```
   Or manually download and extract the ZIP file.

2. Navigate to the project directory:
   ```
   cd ids706-week7-clt
   ```

3. Build the project:
   ```
   cargo build
   ```

4. Now, the `notekeeper` tool is ready for use!

## Usage

To utilize `notekeeper`, use the `cargo run` command followed by the desired `notekeeper` command.

- **Add a Note**:
  ```
  cargo run -- add "Your note content here"
  ```

- **View All Notes**:
  ```
  cargo run -- view
  ```

- **Delete a Note by ID**:
  ```
  cargo run -- delete 1
  ```