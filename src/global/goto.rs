use crate::config::APP_PAGE_SIZE;
use crate::{app::App, config::*};

impl App {
    /// The goto() function checks if the received offset is cached, otherwise
    /// it calls .read_chunk_from_file to fill the right cache block.
    pub fn goto(&mut self, offset: usize) {
        if offset >= self.file_info.size {
            return;
        }

        // If offset is not cached, read and cache the
        // block containing the offset
        // If offset is not cached, read and cache the
        // block containing the offset
        if offset > self.reader.cache_end || offset < self.reader.cache_start {
            let nblock = offset / APP_CACHE_SIZE;
            self.read_chunk_from_file(nblock).unwrap();
            self.reader.cache_start = nblock * APP_CACHE_SIZE;
            self.reader.cache_end = self.reader.cache_start + APP_CACHE_SIZE - 1;
        }

        let bytes_per_line = self.config.hex_mode_bytes_per_line;

        // Determine new page_start
        if offset == 0 {
            self.reader.page_start = 0;
            self.reader.page_end = APP_PAGE_SIZE - 1;
        } else {
            let mut start = self.reader.page_start;

            // If existing page_start is invalid (cache changed or far jump), reset it.
            if start < self.reader.buffer_start || start >= self.reader.buffer_end {
                start = (offset / APP_PAGE_SIZE) * APP_PAGE_SIZE;
            }

            // Ensure we are aligned to line start
            start = (start / bytes_per_line) * bytes_per_line;

            // Slide window if offset is out of bounds
            if offset < start {
                // Scrolled up: Make offset the top line
                start = (offset / bytes_per_line) * bytes_per_line;
            } else if offset >= start + APP_PAGE_SIZE {
                // Scrolled down: Make offset the bottom line (ensure it fits in view)
                let offset_line_start = (offset / bytes_per_line) * bytes_per_line;
                if offset_line_start + bytes_per_line >= APP_PAGE_SIZE {
                    start = offset_line_start + bytes_per_line - APP_PAGE_SIZE;
                } else {
                    start = 0;
                }
            }

            self.reader.page_start = start;
            self.reader.page_end = start + APP_PAGE_SIZE - 1;
        }

        // Update the cursor
        self.hex_view.cursor.y =
            (offset - self.reader.page_start) / self.config.hex_mode_bytes_per_line;
        self.hex_view.cursor.x =
            (offset - self.reader.page_start) % self.config.hex_mode_bytes_per_line;

        // Save current offset
        self.hex_view.last_visited_offset = self.hex_view.offset;
        // Update offset
        self.hex_view.offset = offset;

        // Update offset location in cache
        self.reader.offset_location_in_cache = self.reader.page_start - self.reader.buffer_start;

        self.reader.page_current = offset / APP_PAGE_SIZE;

        // Calculate page_current_size based on available data in file and buffer
        let remaining_file = self.file_info.size.saturating_sub(self.reader.page_start);
        let remaining_buffer = (self.reader.buffer_end + 1).saturating_sub(self.reader.page_start);

        let mut size = std::cmp::min(APP_PAGE_SIZE, remaining_file);
        size = std::cmp::min(size, remaining_buffer);

        self.reader.page_current_size = size;

        App::log(self, format!("goto: {:x}", offset));
    }
}
