use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
"; // TODO: Add the test input

#[derive(Debug, Clone, Copy)]
enum Block {
    File { number: u64, size: u64 },
    Space { size: u64 },
}

#[derive(Debug, Clone, Copy)]
struct Item {
    number: u64,
    size: u64,
    position: u64,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let input = read_input(reader);

        let mut disk = Vec::new();

        for index in 0..input.len() {
            let size = input.get(index).unwrap().clone();
            if index % 2 == 0 {
                disk.push(Block::File {
                    number: (index / 2) as u64,
                    size,
                });
            } else if size > 0 {
                disk.push(Block::Space {
                    size: *input.get(index).unwrap(),
                })
            }
        }

        let mut defragmented = Vec::new();

        let mut left = 0;
        let mut right = disk.len() - 1;
        let mut space_size = 0;
        let mut file_size = 0;
        let mut file_number = 0;
        loop {
            if left > right {
                if file_size != 0 {
                    defragmented.push(Block::File {
                        number: file_number,
                        size: file_size,
                    });
                }
                break;
            }

            if file_size == 0 {
                match disk.get(right) {
                    None => {
                        panic!("should not be empty");
                    }
                    Some(Block::Space { .. }) => {
                        right -= 1;
                    }
                    Some(Block::File { number, size }) => {
                        file_number = *number;
                        file_size = *size;
                        right -= 1;
                    }
                }
                continue;
            }

            if space_size == 0 {
                match disk.get(left) {
                    None => {
                        panic!("should not be empty");
                    }
                    Some(b @ Block::File { .. }) => {
                        defragmented.push(*b);
                        left += 1;
                    }
                    Some(Block::Space { size }) => {
                        space_size = *size;
                        left += 1;
                    }
                }
                continue;
            }

            if space_size >= file_size {
                defragmented.push(Block::File {
                    number: file_number,
                    size: file_size,
                });
                space_size -= file_size;
                file_size = 0;
            } else {
                defragmented.push(Block::File {
                    number: file_number,
                    size: space_size,
                });

                file_size -= space_size;
                space_size = 0;
            }
        }

        let mut sum = 0;
        let mut total_size = 0;
        for b in defragmented.iter() {
            if let Block::File { number, size } = b {
                for i in total_size..total_size + size {
                    sum += i * number;
                }

                total_size += size;
            }
        }

        Ok(sum)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u64> {
        let input = read_input(reader);

        let mut files = Vec::new();
        let mut spaces = Vec::new();

        let mut position = 0;
        let mut number = 0;
        for index in 0..input.len() {
            let size = input.get(index).unwrap().clone();
            if index % 2 == 0 {
                files.push(Item {
                    position,
                    number,
                    size,
                });
                number += 1;
            } else if size > 0 {
                spaces.push(Item {
                    position,
                    number: 0,
                    size,
                })
            }
            position += size;
        }

        let mut defragmented = Vec::new();

        'outer: loop {
            if files.is_empty() {
                break;
            }

            if spaces.is_empty() {
                break;
            }

            if files.first().unwrap().position < spaces.first().unwrap().position {
                defragmented.push(files.remove(0));
                continue;
            }

            let file = files.remove(files.len() - 1);

            for si in 0..spaces.len() {
                if spaces[si].position > file.position {
                    break;
                }
                
                if spaces[si].size < file.size {
                    continue;
                }

                let space = spaces.remove(si);

                defragmented.push(Item {
                    number: file.number,
                    size: file.size,
                    position: space.position,
                });

                // insert space leftover created by moving a file to a bigger space
                if space.size > file.size {
                    spaces.insert(
                        si,
                        Item {
                            number: 0,
                            size: space.size - file.size,
                            position: space.position + file.size,
                        },
                    );
                }

                // merge space crated by moving a file
                merge_space(
                    &mut spaces,
                    Item {
                        number: 0,
                        size: file.size,
                        position: file.position,
                    },
                );

                continue 'outer;
            }
            
            // could find space
            defragmented.push(file);
        }
        
        defragmented.sort_by(|a, b| a.position.cmp(&b.position));
        for d in defragmented.iter() {
            println!("{:?}", d)
        }

        let mut sum = 0;
        for b in defragmented.iter() {
            for p in b.position..b.position + b.size {
                sum += p * b.number;
            }
        }

        Ok(sum)
    }
    fn merge_space(spaces: &mut Vec<Item>, space: Item) {
        for i in 0..spaces.len() {
            if spaces[i].position + spaces[i].size == space.position {
                let prev = spaces.remove(i);
                let mut item = Item {
                    number: 0,
                    size: prev.size + space.size,
                    position: prev.position,
                };
                
                if i + 1 < spaces.len() {
                    if space.position + space.size == spaces[i+1].position {
                        let next = spaces.remove(i + 1);
                        item.size += prev.size;
                    } 
                }
                
                spaces.insert(
                    i,
                    item
                );
            }
        }
    }

    fn read_input<R: BufRead>(reader: R) -> Vec<u64> {
        reader
            .lines()
            .flatten()
            .flat_map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
