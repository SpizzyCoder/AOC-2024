#[derive(PartialEq, Debug, Clone, Copy)]
enum BlockType {
    File,
    FreeSpace,
}

#[derive(Debug, Clone)]
struct Block {
    block_type: BlockType,
    id: i32,
}

pub fn solve(input: &str) {
    let mut memory: Vec<Block> = Vec::new();

    let mut char_iterator = input.chars();
    let mut id_counter: i32 = 0;
    loop {
        if let Some(files) = char_iterator.next() {
            let files_as_num: u32 = files.to_digit(10).unwrap();

            for _ in 0..files_as_num {
                memory.push(Block {
                    block_type: BlockType::File,
                    id: id_counter,
                });
            }
        } else {
            break;
        }

        if let Some(free_space) = char_iterator.next() {
            let free_space_as_num: u32 = free_space.to_digit(10).unwrap();

            for _ in 0..free_space_as_num {
                memory.push(Block {
                    block_type: BlockType::FreeSpace,
                    id: -1,
                });
            }
        } else {
            break;
        }

        id_counter += 1;
    }

    for index in 0..memory.len() {
        if memory[index].block_type == BlockType::FreeSpace {
            for last_index in (0..memory.len()).rev() {
                if index >= last_index {
                    break;
                }

                if memory[last_index].block_type == BlockType::File {
                    memory.swap(index, last_index);
                    break;
                }
            }
        }
    }

    let memory_without_free_space: &[Block] = memory
        .split_at(
            memory
                .iter()
                .position(|x| x.block_type == BlockType::FreeSpace)
                .unwrap(),
        )
        .0;

    let mut sum: i64 = 0;
    for (index, block) in memory_without_free_space.iter().enumerate() {
        sum += index as i64 * block.id as i64;
    }

    println!["Part 1: {}", sum];
}
