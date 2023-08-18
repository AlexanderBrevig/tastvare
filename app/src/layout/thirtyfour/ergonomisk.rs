use rp2040_hal::gpio::DynPin;

use crate::matrix::Matrix;

use super::ThirtyFour;

#[allow(unused)]
pub(crate) fn new(matrix: Matrix<DynPin, DynPin, 6, 3>) -> ThirtyFour {
    ThirtyFour {
        matrix,
        is_left: true,
        prev_scan: [[false; 6]; 3],
    }
}
