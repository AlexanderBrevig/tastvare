use rp2040_hal::gpio::DynPin;
use tast::{layout::Layout, protocol::Event};

use crate::matrix::Matrix;

pub mod ergonomisk;

pub struct ThirtyFour {
    is_left: bool,
    matrix: Matrix<DynPin, DynPin, 6, 3>,
    prev_scan: [[bool; 6]; 3],
}

impl Layout for ThirtyFour {
    fn get_event(&mut self) -> Option<tast::protocol::TimedEvent> {
        let scan = self.matrix.get();
        if let Ok(scan) = scan {
            for (row_ix, row) in scan.iter().enumerate() {
                for (col_ix, key) in row.iter().enumerate() {
                    if self.prev_scan[row_ix][col_ix] != *key {
                        self.prev_scan[row_ix][col_ix] = *key;
                        let mut event = Event::from_bits((row_ix * 6 + col_ix) as u8).unwrap();
                        if *key {
                            event |= Event::PRESSED;
                        }
                        if row_ix != 1 || col_ix == 4 {
                            // Anything but home row and thumbs are immediate
                            event |= Event::IMMEDIATE;
                        }

                        if self.is_left {
                            event |= Event::ID5;
                        }

                        //TODO: decide if we actually need to send some time here
                        return Some((event, 10));
                    }
                }
            }
        }
        None
    }
}
