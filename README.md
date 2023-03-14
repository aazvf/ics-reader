This entire project was generated using chat-gpt, I just pieced it all together.

# Rust ICS Reader

The Rust ICS Reader is a simple Rust program that extracts UK bank holidays from an iCalendar file and outputs them in JSON format.

## Getting Started

## Prerequisites

To use this program, you'll need to have Rust installed on your system. If you don't already have Rust installed, you can download and install it from the official Rust website at rust-lang.org.

## Installing
To install the Rust ICS Reader, you can simply clone this repository:

```bash
git clone https://github.com/aazvf/rust-ics-reader.git
```

Once you've cloned the repository, you can build and run the program using Cargo:

```bash
cd rust-ics-reader
cargo run --release https://www.gov.uk/bank-holidays/england-and-wales.ics
```

## Usage
The Rust ICS Reader requires one command line argument: the URL of the iCalendar file containing UK bank holidays.

```
cargo run --release <URL>
```

## Output

The Rust ICS Reader outputs UK bank holidays in JSON format. Each holiday is represented by a JSON object with the following fields:

- date_start: The start date of a multi-day holiday in YYYY-MM-DD format.
- date_end: The end date of a multi-day holiday in YYYY-MM-DD format.
- summary: A short summary of the holiday.

Here's an example of what the JSON output might look like:

```json
[
  {
    "date_start": "2023-04-14",
    "date_end": "2023-04-17",
    "summary": "Good Friday"
  },
  {
    "date": "2023-04-17",
    "date_start": "2023-04-14",
    "date_end": "2023-04-17",
    "summary": "Easter Monday"
  },
]
```

## Contributing
If you'd like to contribute to the Rust ICS Reader, please feel free to fork the repository and submit a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.
