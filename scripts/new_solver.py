#!/usr/bin/env python3
"""
Advent of Code Solution Generator

Generates new solution files for Advent of Code challenges, including:
- Creating the solution file (src/y{year}/day{day}.rs)
- Updating the year's mod.rs file
- Updating src/main.rs if it's a new year.

**This script was created with the help of Claude AI.**
"""

import argparse
import logging
import sys
from pathlib import Path
from typing import List, Tuple

MIN_YEAR = 2015
MAX_YEAR = 2025
MAX_DAYS = 25

# Easily modifiable template for new solution files
RUST_SOLUTION_TEMPLATE = """use advent_of_code_data as aoc;
use yuletide as yt;

use linkme::distributed_slice;

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: yt::SolverAutoRegister = yt::SolverAutoRegister {{
    modpath: std::module_path!(),
    part_one: yt::SolverPart {{
        func: day_{day}_1,
        examples: &[/*yt::Example {{
            input: "",
            expected: aoc::Answer::Int(0),
        }}*/],
    }},
    part_two: yt::SolverPart {{
        func: day_{day}_2,
        examples: &[/*yt::Example {{
            input: "",
            expected: aoc::Answer::Int(0),
        }}*/],
    }},
}};

pub fn day_{day}_1(args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {{ 
    Err(yt::SolverError::NotFinished)
}}

pub fn day_{day}_2(_args: &yt::SolverArgs) -> yt::Result<aoc::Answer> {{
    Err(yt::SolverError::NotFinished)
}}
"""


def setup_logging() -> None:
    """Configure logging with a clean format."""
    logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")


def parse_args() -> argparse.Namespace:
    """Parse command-line arguments."""
    parser = argparse.ArgumentParser(
        description="Generate new Advent of Code solution files"
    )

    parser.add_argument(
        "year", nargs="?", type=int, help="Year for the solution (e.g., 2024)"
    )
    parser.add_argument("day", nargs="?", type=int, help="Day for the solution (1-25)")
    parser.add_argument(
        "--year",
        dest="year_flag",
        type=int,
        help="Year for the solution (overrides positional)",
    )
    parser.add_argument(
        "--day",
        dest="day_flag",
        type=int,
        help="Day for the solution (overrides positional)",
    )

    args = parser.parse_args()

    # Use flag values if provided, otherwise use positional
    year = args.year_flag if args.year_flag is not None else args.year
    day = args.day_flag if args.day_flag is not None else args.day

    if year is None or day is None:
        parser.error("Both year and day are required")

    # Update namespace with final values
    args.year = year
    args.day = day

    return args


def validate_inputs(year: int, day: int) -> None:
    """
    Validate year and day inputs.

    Raises:
        ValueError: If inputs are out of valid range
    """
    if not (MIN_YEAR <= year <= MAX_YEAR):
        raise ValueError(f"Year must be between {MIN_YEAR} and {MAX_YEAR}, got {year}")

    if not (1 <= day <= MAX_DAYS):
        raise ValueError(f"Day must be between 1 and {MAX_DAYS}, got {day}")


def create_solution_file(year: int, day: int) -> bool:
    """
    Create the solution file for the given year and day.

    Args:
        year: The year for the solution
        day: The day for the solution

    Returns:
        True if this is a new year (directory was created), False otherwise

    Raises:
        FileExistsError: If the solution file already exists
    """
    year_dir = Path("src") / f"y{year}"
    solution_file = year_dir / f"day{day}.rs"

    # Check if solution file already exists
    if solution_file.exists():
        raise FileExistsError(f"Solution file already exists: {solution_file}")

    # Track if this is a new year
    is_new_year = not year_dir.exists()

    # Create year directory if needed
    year_dir.mkdir(parents=True, exist_ok=True)
    if is_new_year:
        logging.info(f"Created directory: {year_dir}")

    # Write solution file
    content = RUST_SOLUTION_TEMPLATE.format(year=year, day=day)
    solution_file.write_text(content)
    logging.info(f"Created solution file: {solution_file}")

    return is_new_year


def parse_mod_declarations(lines: List[str]) -> List[int]:
    """
    Parse mod declarations from mod.rs to extract day numbers.

    Args:
        lines: Lines from the mod.rs file

    Returns:
        List of day numbers found in mod declarations
    """
    days = []
    for line in lines:
        line = line.strip()
        if line.startswith("mod day") and line.endswith(";"):
            # Extract day number from "mod day{N};"
            day_str = line[7:-1]  # Remove "mod day" prefix and ";" suffix
            try:
                days.append(int(day_str))
            except ValueError:
                # Skip malformed lines
                continue
    return days


def update_mod_file(year: int, day: int) -> None:
    """
    Update (or create) the mod.rs file for the given year.

    Adds the mod declaration for the new day and sorts all declarations.

    Args:
        year: The year for the solution
        day: The day to add
    """
    year_dir = Path("src") / f"y{year}"
    mod_file = year_dir / "mod.rs"

    # Read existing mod declarations if file exists
    if mod_file.exists():
        lines = mod_file.read_text().splitlines()
        days = parse_mod_declarations(lines)
    else:
        days = []
        logging.info(f"Creating new mod file: {mod_file}")

    # Add new day if not present
    if day in days:
        logging.info(f"Day {day} already present in {mod_file}")
    else:
        days.append(day)
        logging.info(f"Added 'mod day{day};' to {mod_file}")

    # Sort days naturally (1, 2, ..., 10, not lexicographic)
    days.sort()

    # Write sorted mod declarations
    content = "\n".join(f"mod day{d};" for d in days) + "\n"
    mod_file.write_text(content)


def parse_year_mods(lines: List[str]) -> Tuple[List[int], int]:
    """
    Parse year mod declarations from main.rs.

    Args:
        lines: Lines from main.rs

    Returns:
        Tuple of (list of years, index of first blank line)
    """
    years = []
    blank_line_idx = 0

    for idx, line in enumerate(lines):
        stripped = line.strip()

        # Stop at first blank line
        if not stripped:
            blank_line_idx = idx
            break

        # Parse year mod declarations
        if stripped.startswith("mod y") and stripped.endswith(";"):
            # Extract year from "mod y{YEAR};"
            year_str = stripped[5:-1]  # Remove "mod y" prefix and ";" suffix
            try:
                years.append(int(year_str))
            except ValueError:
                # Skip malformed lines
                continue

    return years, blank_line_idx


def update_main_file(year: int) -> None:
    """
    Update src/main.rs to include the new year module.

    Adds the year mod declaration and sorts all year declarations.

    Args:
        year: The year to add
    """
    main_file = Path("src") / "main.rs"

    if not main_file.exists():
        logging.error(f"main.rs not found at {main_file}")
        raise FileNotFoundError(f"Expected main.rs at {main_file}")

    # Read main.rs
    lines = main_file.read_text().splitlines()

    # Parse year mods
    years, blank_line_idx = parse_year_mods(lines)

    # Add new year if not present
    if year in years:
        logging.info(f"Year {year} already present in {main_file}")
        return

    years.append(year)
    logging.info(f"Added 'mod y{year};' to {main_file}")

    # Sort years
    years.sort()

    # Reconstruct file: year mods + blank line + rest
    year_mod_lines = [f"mod y{y};" for y in years]
    rest_of_file = lines[blank_line_idx:]

    new_content = "\n".join(year_mod_lines) + "\n" + "\n".join(rest_of_file)
    main_file.write_text(new_content)


def main() -> int:
    """
    Main entry point for the script.

    Returns:
        Exit code (0 for success, 1 for error)
    """
    setup_logging()

    try:
        # Parse and validate arguments
        args = parse_args()
        year = args.year
        day = args.day

        logging.info(f"Creating solution for year {year}, day {day}")

        validate_inputs(year, day)

        # Create solution file and track if new year
        is_new_year = create_solution_file(year, day)

        # Update mod.rs
        update_mod_file(year, day)

        # Update main.rs if new year
        if is_new_year:
            update_main_file(year)

        logging.info("Successfully generated Advent of Code solution")
        return 0

    except (ValueError, FileExistsError, FileNotFoundError) as e:
        logging.error(str(e))
        return 1
    except Exception as e:
        logging.exception(f"Unexpected error: {e}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
