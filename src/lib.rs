use std::fs::read_to_string;

pub fn optimize_checksum(input_path: &str) -> u32 {
    return 0;
}

fn re_order_files(mut files: Vec<File>) -> Vec<File> {
    let mut ordered_files = Vec::new();
    for existing_file_index in 0..files.len() {
        let existing_file = files.get_mut(existing_file_index).unwrap();
        let mut item_places_count = 0;

        if !existing_file.free_space {
            ordered_files.push(File {
                id: existing_file.id,
                capacity: existing_file.capacity - item_places_count,
                free_space: existing_file.free_space
            });
        } else {
            let free_space_available = existing_file.capacity;
            for _i in 0..free_space_available  {
                if let Some(last_file_with_content) = files.iter_mut().filter(|file| !file.free_space).last() {
                    if last_file_with_content.capacity == 1 {
                        last_file_with_content.capacity -= 1;   
                        if last_file_with_content.capacity == 0 {
                            last_file_with_content.free_space = false;
                        }
                    }
                    ordered_files.push(File {
                        id: last_file_with_content.id,
                        capacity: 1,
                        free_space: false
                    });
                    item_places_count += 1;
                    
                } else {
                    return ordered_files;
                }
                
            }
        }
    }
    ordered_files.into_iter().filter(|file| !file.free_space).collect()
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
        acc.push(File  {
            id: index,
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
    fn should_re_order_when_there_is_free_space() {
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
                capacity: 4,
                free_space: false
            },
            File {
                id: 3,
                capacity: 2,
                free_space: true
            },
            File {
                id: 4,
                capacity: 2,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files.clone()), vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            }, 
            File {
                id: 2,
                capacity: 1,
                free_space: false
            },
            File {
                id: 2,
                capacity: 1,
                free_space: false
            },
            File {
                id: 2,
                capacity: 1,
                free_space: false
            },
            File {
                id: 2,
                capacity: 1,
                free_space: false
            },
            File {
                id: 4,
                capacity: 1,
                free_space: false
            },
            File {
                id: 4,
                capacity: 1,
                free_space: false
            }
        ]);
    }

    #[test]
    fn should_re_order_when_there_is_free_space_to_contain_all_the_last_one() {
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
                capacity: 3,
                free_space: false
            }
        ];
        assert_eq!(re_order_files(files.clone()), vec![
            File {
                id: 0,
                capacity: 2,
                free_space: false
            }, 
            File {
                id: 2,
                capacity: 1,
                free_space: false
            },
            File {
                id: 2,
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
                capacity: 3,
                free_space: false
            },
            File {
                id: 1,
                capacity: 4,
                free_space: true
            },
            File {
                id: 2,
                capacity: 6,
                free_space: false
            },
            File {
                id: 3,
                capacity: 3,
                free_space: true
            },
        ]);
    }
    

}