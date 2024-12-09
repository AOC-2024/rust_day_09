use std::{collections::VecDeque, fs::read_to_string};

pub fn optimize_checksum(input_path: &str) -> u32 {
    let files = extract_original_file(input_path);

    let ordered_files = re_order_files(files);
    println!("{:?}", flatten_checksum(&ordered_files));
    calculate_checksum(ordered_files)
}

fn calculate_checksum(files: Vec<File>) -> u32 {
    let mut checksum = 0;
    let mut position = 0;

    for file in files {
        for _ in 0..file.capacity {
            checksum += position * file.id;
            position += 1;
        }
    }

    checksum as u32
}

fn flatten_checksum(files: &Vec<File>) -> String {
    let mut flattened_results = Vec::new();

    for file in files {
        for _ in 0..file.capacity {
            flattened_results.push(format!("{}",file.id));
        }
    }

    flattened_results.join("")
}


fn re_order_files(files: Vec<File>) -> Vec<File> {
    if files.len() == 0 {
        return files;
    }
    let mut ordered_files: Vec<File> = vec![files.get(0).unwrap().clone()];
    let mut queue: VecDeque<File> = VecDeque::from(files.clone().into_iter().skip(1).filter(|file| !file.free_space).collect::<Vec<File>>());


    for file_index in 1..files.len() {
        if queue.is_empty() {
            break;
        }
        let file = files.get(file_index).unwrap().clone();
        if file.free_space {
            let mut capacity_to_insert = file.capacity;

            loop {
                let last_pending_in_queue = queue.pop_back().unwrap();

                if capacity_to_insert == 0 || queue.is_empty() {
                    break;
                }

                // If there is still capacity to allocate after moving last file pending => pop and push back with the remaining capacity
                if file.capacity >= last_pending_in_queue.capacity {
                    ordered_files.push(File {
                        id: last_pending_in_queue.id,
                        capacity: last_pending_in_queue.capacity,
                        free_space: false
                    });
                    capacity_to_insert = file.capacity - last_pending_in_queue.capacity;
                } else {
                    ordered_files.push(File {
                        id: last_pending_in_queue.id,
                        capacity: file.capacity,
                        free_space: false
                    });
                    queue.push_back(File {
                        id: last_pending_in_queue.id,
                        capacity: last_pending_in_queue.capacity - file.capacity,
                        free_space: false
                    });
                    break;
                }
            }
            
        } else {
            ordered_files.push(file);
        }
    }

    ordered_files
} 

fn extract_original_file(input_path: &str) -> Vec<File> {
    read_to_string(input_path)
    .unwrap()
    .lines()
    .last()
    .unwrap()
    .chars()
    .enumerate()
    .fold(Vec::new(), |mut acc, (index, number)| {
        let mut id = index / 2;
        if index % 2 != 0 {
            id = 0;
        }
        acc.push(File  {
            id,
            capacity: number.to_digit(10).unwrap(),
            free_space: index % 2 != 0
        });
        acc
    })
}

#[derive(PartialEq, Debug, Clone)]
struct File {
    id: usize,
    capacity: u32,
    free_space: bool
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn should_add_last_content_in_free_space_with_remaining() {
        let files = vec![
            File {
                id: 0,
                capacity: 1,
                free_space: false
            },
            File {
                id: 0,
                capacity: 2,
                free_space: true
            },
            File {
                id: 1,
                capacity: 3,
                free_space: false
            },
            
            File {
                id: 0,
                capacity: 4,
                free_space: true
            },
            File {
                id: 2,
                capacity: 5,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files), vec![
            File {
                id: 0,
                capacity: 1,
                free_space: false
            },
            File {
                id: 2,
                capacity: 2,
                free_space: false
            },
            File {
                id: 1,
                capacity: 3,
                free_space: false
            },
            File {
                id: 2,
                capacity: 3,
                free_space: false
            }
        ]);
    }

    #[test]
    fn should_add_last_content_in_free_space_without_remaining() {
        let files = vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 1,
                capacity: 2,
                free_space: true
            },
            File {
                id: 2,
                capacity: 1,
                free_space: false
            },

            File {
                id: 3,
                capacity: 2,
                free_space: true
            },
            File {
                id: 4,
                capacity: 1,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files), vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 4,
                capacity: 1,
                free_space: false
            },
            File {
                id: 2,
                capacity: 1,
                free_space: false
            }
        ]);
    }

    #[test]
    fn should_put_last_file_into_empty_file_when_same_capacity() {
        let files = vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 0,
                capacity: 2,
                free_space: true
            },
            File {
                id: 1,
                capacity: 2,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files), vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 1,
                capacity: 2,
                free_space: false
            }
        ]);
    }

    #[test]
    fn should_not_order_when_there_is_no_free_space() {
        let files = vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            },
            File {
                id: 1,
                capacity: 2,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files.clone()), files);
    }

    #[test]
    fn should_ordered_file_return_empty_when_files_empty() {
        assert_eq!(re_order_files(Vec::new()), Vec::new());
    }

    #[test]
    fn should_extract_original_file() {
        assert_eq!(extract_original_file("tests/resources/light_puzzle.txt"), vec![
            File {
                id: 0,
                capacity: 1,
                free_space: false
            },
            File {
                id: 0,
                capacity: 2,
                free_space: true
            },
            File {
                id: 1,
                capacity: 3,
                free_space: false
            },
            File {
                id: 0,
                capacity: 4,
                free_space: true
            },
            File {
                id: 2,
                capacity: 5,
                free_space: false
            },
        ]);
    }
    

}