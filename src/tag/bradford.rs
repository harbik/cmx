use nalgebra::Matrix3;

use crate::S15Fixed16;

pub struct Bradford {
    m_adapt: Matrix3<f64>,
}

impl Bradford {
    const BRADFORD_ADAPT: Matrix3<f64> = Matrix3::new(
        0.8951, 0.2664, -0.1614, -0.7502, 1.7135, 0.0367, 0.0389, -0.0685, 1.0296,
    );

    const BRADFORD_ADAPT_INV: Matrix3<f64> = Matrix3::new(
        0.9869929, -0.1470543, 0.1599627, 0.4323053, 0.5183603, 0.0492912, -0.0085287, 0.0400428,
        0.9684867,
    );

    pub fn new(source_white: [f64; 3], target_white: [f64; 3]) -> Self {
        let source_cone_response =
            Self::BRADFORD_ADAPT * nalgebra::Vector3::from_column_slice(&source_white);
        let target_cone_response =
            Self::BRADFORD_ADAPT * nalgebra::Vector3::from_column_slice(&target_white);

        let scale = Matrix3::new(
            target_cone_response[0] / source_cone_response[0],
            0.0,
            0.0,
            0.0,
            target_cone_response[1] / source_cone_response[1],
            0.0,
            0.0,
            0.0,
            target_cone_response[2] / source_cone_response[2],
        );

        let m_adapt = Self::BRADFORD_ADAPT_INV * scale * Self::BRADFORD_ADAPT;

        Bradford { m_adapt }
    }

    pub fn adapt(&self, color: [f64; 3]) -> [f64; 3] {
        let adapted = self.m_adapt * nalgebra::Vector3::from_column_slice(&color);
        [adapted[0], adapted[1], adapted[2]]
    }

    pub fn as_matrix(&self) -> Matrix3<f64> {
        self.m_adapt
    }

    pub fn as_tag_data(&self) -> [S15Fixed16; 9] {
        let mut data = [S15Fixed16::from(0.0); 9];
        for i in 0..3 {
            for j in 0..3 {
                data[i * 3 + j] = S15Fixed16::from(self.m_adapt[(i, j)]);
            }
        }
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bradford_adaptation() {
        let source_white = [0.95047, 1.00000, 1.08883]; // D65
        let target_white = [0.96422, 1.00000, 0.82521]; // D50

        let bradford = Bradford::new(source_white, target_white);

        let color = source_white;
        let adapted_color = bradford.adapt(color);

        // Expected values can be calculated using a reliable color science library or tool
        let expected_color = target_white;
        approx::assert_abs_diff_eq!(
            adapted_color.as_slice(),
            expected_color.as_slice(),
            epsilon = 0.0001
        );
    }

    #[test]
    fn test_bradford_tag_data() {
        let d65 = [0.95047, 1.00000, 1.08883]; // D65
        let d50 = [0.96422, 1.00000, 0.82521]; // D50

        // values from Apple's DisplayP3 ICC profile
        // matrix = [[1.047882, 0.022919, -0.050201], [0.029587, 0.990479, -0.017059], [-0.009232, 0.015076, 0.751678]]

        let want = Matrix3::new(
            1.0478112, 0.0228866, -0.0501270, 0.0295424, 0.9904844, -0.0170491, -0.0092345,
            0.0150436, 0.7521316,
        );

        let bradford = Bradford::new(d65, d50);
        let got = bradford.as_matrix();
        approx::assert_abs_diff_eq!(got, want, epsilon = 0.0001);
    }
}
