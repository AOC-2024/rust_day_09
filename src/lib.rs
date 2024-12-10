use std::fs::read_to_string;

pub fn calculate_checksum(file_path: &str) -> usize {
    let data = read_file(file_path);

    let mut partitions = Vec::new();
    let mut is_data_segment = true;
    let mut segment_id = 0;

    for count in data {
        if is_data_segment {
            partitions.extend((0..count).map(|_| Some(segment_id)));
            segment_id += 1;
        } else {
            partitions.extend((0..count).map(|_| None));
        }
        is_data_segment = !is_data_segment;
    }

    let mut start = 0;
    let mut end = partitions.len() - 1;

    while start < end {
        while partitions[start].is_some() {
            start += 1;
        }
        while partitions[end].is_none() {
            end -= 1;
        }
        if start < end {
            partitions.swap(start, end);
        }
        start += 1;
        end -= 1;
    }

    partitions
        .into_iter()
        .enumerate()
        .filter_map(|(index, value)| value.map(|id| id * index))
        .sum()
}

pub fn calculate_all_fit_checksum(file_path: &str) -> usize {
    let data: Vec<usize> = read_file(file_path);

    let mut sections = Vec::new();
    let mut processing_files = true;
    let mut file_id = 0;

    for amount in data {
        if processing_files {
            sections.extend((0..amount).map(|_| Some(file_id)));
            file_id += 1;
        } else {
            sections.extend((0..amount).map(|_| None));
        }
        processing_files = !processing_files;
    }

    file_id -= 1;
    let mut upper_bound = (sections.len() - 1) as isize;

    while upper_bound > 0 {
        let mut current_index = upper_bound as usize;

        while sections[current_index].is_none() || sections[current_index] != Some(file_id) {
            current_index -= 1;
        }

        let mut segment_length = 0;
        let mut free_space_start = 0;
        let mut content_end = 0;

        while current_index >= content_end
            && sections[current_index - content_end].is_some()
            && sections[current_index - content_end] == Some(file_id)
        {
            content_end += 1;
        }

        if file_id == 0 {
            break;
        }

        file_id -= 1;

        while free_space_start < content_end && segment_length <= current_index {
            segment_length += free_space_start;
            free_space_start = 0;

            while sections[segment_length].is_some() {
                segment_length += 1;
            }

            while segment_length + free_space_start <= current_index
                && sections[segment_length + free_space_start].is_none()
            {
                free_space_start += 1;
            }
        }

        if free_space_start >= content_end {
            for offset in 0..content_end {
                sections.swap(segment_length + offset, current_index - offset);
            }
        }

        upper_bound = (current_index - content_end) as isize;
    }

    sections
        .into_iter()
        .enumerate()
        .filter_map(|(index, value)| value.map(|id| id * index))
        .sum()
}

fn read_file(file_path: &str) -> Vec<usize> {
    let data: Vec<usize> = read_to_string(file_path)
        .unwrap()
        .chars()
        .filter_map(|char| char.to_digit(10).map(|digit| digit as usize))
        .collect();
    data
}
