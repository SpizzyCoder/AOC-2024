#[derive(PartialEq, Debug, Clone, Copy)]
enum BlockType {
    File,
    FreeSpace,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Block {
    block_type: BlockType,
    id: i64,
}

#[derive(Debug, Clone, Copy)]
struct SmartPos {
    pos: usize,
    len: usize,
}

pub fn solve(input: &str) {
    let mut memory: Vec<Block> = Vec::new();

    let mut char_iterator = input.chars();
    let mut id_counter: i64 = 0;
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

    defrag(&mut memory);

    let mut sum: i64 = 0;
    for (index, block) in memory.iter().enumerate() {
        if block.block_type == BlockType::FreeSpace {
            continue;
        }

        sum += index as i64 * block.id as i64;
    }

    println!["Part 2: {}", sum];
}

fn count_blocks(block: Block, blocks: &[Block]) -> usize {
    let mut count: usize = 0;

    for i in blocks.iter() {
        if *i == block {
            count += 1;
        } else {
            break;
        }
    }

    return count;
}

fn defrag(memory: &mut [Block]) {
    let mut free_spaces: Vec<SmartPos> = Vec::new();
    let mut file_spaces: Vec<SmartPos> = Vec::new();

    let mut index: usize = 0;
    while memory.get(index).is_some() {
        let amount_of_blocks: usize = count_blocks(memory[index], &memory[index..]);
        let smart_pos: SmartPos = SmartPos {
            pos: index,
            len: amount_of_blocks,
        };

        if memory[index].block_type == BlockType::File {
            file_spaces.push(smart_pos);
        } else {
            free_spaces.push(smart_pos);
        }

        index += amount_of_blocks;
    }

    while file_spaces.len() != 0 {
        let last_used_space = file_spaces.len() - 1;
        let file_space = file_spaces[last_used_space];

        for cur_free_space_index in 0..free_spaces.len() {
            let cur_free_space = &mut free_spaces[cur_free_space_index];

            if cur_free_space.pos >= file_space.pos {
                break;
            }

            if cur_free_space.len >= file_space.len {
                for i in 0..file_space.len {
                    memory.swap(
                        cur_free_space.pos + i,
                        file_space.pos + i,
                    );
                }

                if cur_free_space.len == file_space.len {
                    free_spaces.remove(cur_free_space_index);
                } else {
                    cur_free_space.pos += file_space.len;
                    cur_free_space.len -= file_space.len;
                }

                break;
            }
        }

        file_spaces.remove(last_used_space);
    }
}