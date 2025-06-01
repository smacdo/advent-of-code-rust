use advent_of_code_data::{Answer, Day, Year};
use linkme::distributed_slice;
use yuletide::{Example, Result, Solver, SolverPart};

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(9),
    year: Year(2024),
    part_one: SolverPart {
        func: day_9_1,
        examples: &[Example {
            input: "2333133121414131402",
            expected: Answer::Int(1928),
        }],
    },
    part_two: SolverPart {
        func: day_9_2,
        examples: &[Example {
            input: "2333133121414131402",
            expected: Answer::Int(2858),
        }],
    },
};

#[allow(dead_code)]
fn visualize(disk: &[Option<usize>]) {
    disk.iter().for_each(|block| match block {
        Some(file_id) => print!("{}", *file_id),
        None => print!("."),
    });

    println!();
}

#[allow(dead_code)]
fn visualize2(disk: &[Chunk]) {
    let mut block_index: usize = 0;

    for chunk in disk {
        for _ in block_index..(block_index + chunk.length) {
            match chunk.file_id {
                Some(id) => print!("{}", id),
                None => print!("."),
            }
        }

        block_index += chunk.length;
    }

    println!();
}
pub fn day_9_1(input: &str) -> Result<Answer> {
    // Build the initial disk using the disk map input.
    let mut disk: Vec<Option<usize>> = Vec::new();
    let mut next_id = 0;

    for (i, c) in input.chars().filter(|c| c.is_ascii_digit()).enumerate() {
        let length = c.to_digit(10).unwrap() as usize;
        let file_id: Option<usize> = if i % 2 == 0 {
            // file
            next_id += 1;
            Some(next_id - 1)
        } else {
            // empty space
            None
        };

        // insert block
        disk.extend(std::iter::repeat_n(file_id, length));
    }

    //visualize(&disk);

    // Compact the disk.
    assert!(disk.len() > 1);

    let mut left: usize = 0;
    let mut right = disk.len() - 1;

    while left < right {
        // Skip file blocks until the next empty block is located.
        while left < right && disk[left].is_some() {
            left += 1;
        }

        // Skip empty blocks on the right.
        while left < right && disk[right].is_none() {
            assert_ne!(right, 0); // TODO: probably return?
            right -= 1;
        }

        // Compact.
        if left < right {
            assert!(disk[left].is_none());
            assert!(disk[right].is_some());

            disk.swap(left, right);
        }
    }

    //visualize(&disk);

    // Calculate the checksum.
    let checksum: usize = disk
        .into_iter()
        .enumerate()
        .filter_map(|(index, id)| id.map(|id| id * index))
        .sum();

    Ok(checksum.into())
}

struct Chunk {
    file_id: Option<usize>,
    length: usize,
}

pub fn day_9_2(input: &str) -> Result<Answer> {
    // Build the initial disk using the disk map input.
    let mut disk: Vec<Chunk> = Vec::new();
    //let mut disk: Vec<Option<usize>> = Vec::new();
    let mut next_id = 0;

    for (i, c) in input.chars().filter(|c| c.is_ascii_digit()).enumerate() {
        let length = c.to_digit(10).unwrap() as usize;
        let file_id: Option<usize> = if i % 2 == 0 {
            // file
            next_id += 1;
            Some(next_id - 1)
        } else {
            // empty space
            None
        };

        // insert block
        disk.push(Chunk { file_id, length });
        //disk.extend(std::iter::repeat(file_id).take(length));
    }

    //visualize2(&disk);

    // Compact the disk.
    assert!(disk.len() > 1);

    for i in (0..disk.len()).rev() {
        // skip blank space.
        if disk[i].file_id.is_none() {
            continue;
        }

        // find the first space large enough to hold this file.
        let file_length = disk[i].length;
        let empty_index = disk
            .iter()
            .take(i) // only search chunks prior to this file
            .position(|chunk| chunk.file_id.is_none() && chunk.length >= file_length);

        if let Some(empty_index) = empty_index {
            // Split the free space into used / unused.
            let unused_length = disk[empty_index].length - file_length;

            // Swap blocks.
            //disk.swap(i, empty_index);
            disk[empty_index].file_id = disk[i].file_id;
            disk[empty_index].length = disk[i].length;

            disk[i].file_id = None;

            // Generate a new empty block
            if unused_length > 0 {
                disk.insert(
                    empty_index + 1,
                    Chunk {
                        file_id: None,
                        length: unused_length,
                    },
                );
            }
        }
    }

    //visualize2(&disk);

    // Calculate the checksum.
    let mut checksum: usize = 0;
    let mut block_index: usize = 0;

    for chunk in disk {
        for i in block_index..(block_index + chunk.length) {
            checksum += chunk.file_id.unwrap_or(0) * i
        }

        block_index += chunk.length;
    }

    Ok(checksum.into())
}
