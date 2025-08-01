use crate::include_models;
include_models!(
    resize_1d_linear_scale,
    resize_1d_nearest_scale,
    resize_2d_bicubic_scale,
    resize_2d_bilinear_scale,
    resize_2d_nearest_scale,
    resize_with_sizes
);

#[cfg(test)]
mod tests {
    use super::*;
    use burn::tensor::{Tensor, TensorData, ops::FloatElem};
    use float_cmp::ApproxEq;

    use crate::backend::Backend;
    type FT = FloatElem<Backend>;

    #[test]
    fn resize_with_sizes() {
        // Initialize the model without weights (because the exported file does not contain them)
        let device = Default::default();
        let model: resize_with_sizes::Model<Backend> = resize_with_sizes::Model::new(&device);

        // Run the model
        let input = Tensor::<Backend, 4>::from_floats(
            [[[
                [0.0, 1.0, 2.0, 3.0],
                [4.0, 5.0, 6.0, 7.0],
                [8.0, 9.0, 10.0, 11.0],
                [12.0, 13.0, 14.0, 15.0],
            ]]],
            &device,
        );

        // The sizes are [1, 1, 2, 3]
        let output = model.forward(input);
        let expected = TensorData::from([[[[0.0f32, 1.5, 3.0], [12.0, 13.5, 15.0]]]]);

        output.to_data().assert_eq(&expected, true);
    }

    #[test]
    fn resize_with_scales_1d_linear() {
        // Initialize the model without weights (because the exported file does not contain them)
        let device = Default::default();
        let model: resize_1d_linear_scale::Model<Backend> =
            resize_1d_linear_scale::Model::new(&device);

        // Run the model
        let input = Tensor::<Backend, 3>::from_floats(
            [[[1.5410, -0.2934, -2.1788, 0.5684, -1.0845, -1.3986]]],
            &device,
        );

        // The scales are 1.5
        let output = model.forward(input);

        Tensor::<Backend, 3>::from([[[
            1.5410, 0.3945, -0.7648, -1.9431, -0.8052, 0.3618, -0.6713, -1.2023, -1.3986,
        ]]])
        .to_data()
        .assert_approx_eq::<FT>(&output.into_data(), burn::tensor::Tolerance::default());
    }

    #[test]
    fn resize_with_scales_2d_bilinear() {
        // Initialize the model without weights (because the exported file does not contain them)
        let device = Default::default();
        let model: resize_2d_bilinear_scale::Model<Backend> =
            resize_2d_bilinear_scale::Model::new(&device);

        // Run the model
        let input = Tensor::<Backend, 4>::from_floats(
            [[[
                [-1.1258, -1.1524, -0.2506, -0.4339, 0.8487, 0.6920],
                [-0.3160, -2.1152, 0.3223, -1.2633, 0.3500, 0.3081],
                [0.1198, 1.2377, 1.1168, -0.2473, -1.3527, -1.6959],
                [0.5667, 0.7935, 0.4397, 0.1124, 0.6408, 0.4412],
                [-0.2159, -0.7425, 0.5627, 0.2596, 0.5229, 2.3022],
                [-1.4689, -1.5867, 1.2032, 0.0845, -1.2001, -0.0048],
            ]]],
            &device,
        );

        // The scales are 1.5, 1.5
        let output = model.forward(input);

        let output_sum = output.sum().into_scalar();
        let expected_sum = -3.401_126_6; // from pytorch

        assert!(expected_sum.approx_eq(output_sum, (1.0e-4, 2)));
    }

    #[test]
    fn resize_with_scales_2d_nearest() {
        // Initialize the model without weights (because the exported file does not contain them)
        let device = Default::default();
        let model: resize_2d_nearest_scale::Model<Backend> =
            resize_2d_nearest_scale::Model::<Backend>::new(&device);

        // Run the model
        let input = Tensor::<Backend, 4>::from_floats(
            [[[
                [-1.1258, -1.1524, -0.2506, -0.4339, 0.8487, 0.6920],
                [-0.3160, -2.1152, 0.3223, -1.2633, 0.3500, 0.3081],
                [0.1198, 1.2377, 1.1168, -0.2473, -1.3527, -1.6959],
                [0.5667, 0.7935, 0.4397, 0.1124, 0.6408, 0.4412],
                [-0.2159, -0.7425, 0.5627, 0.2596, 0.5229, 2.3022],
                [-1.4689, -1.5867, 1.2032, 0.0845, -1.2001, -0.0048],
            ]]],
            &device,
        );

        // The scales are 1.5, 1.5
        let output = model.forward(input);

        assert_eq!(output.dims(), [1, 1, 9, 9]);

        let output_sum = output.sum().into_scalar();
        let expected_sum = -0.812_227_7; // from pytorch

        assert!(expected_sum.approx_eq(output_sum, (1.0e-4, 2)));
    }

    #[test]
    fn resize_with_scales_1d_nearest() {
        // Initialize the model without weights (because the exported file does not contain them)
        let device = Default::default();
        let model: resize_1d_nearest_scale::Model<Backend> =
            resize_1d_nearest_scale::Model::<Backend>::new(&device);

        // Run the model
        let input = Tensor::<Backend, 3>::from_floats(
            [[[1.5410, -0.2934, -2.1788, 0.5684, -1.0845, -1.3986]]],
            &device,
        );

        // The scales are 1.5, 1.5
        let output = model.forward(input);

        assert_eq!(output.dims(), [1, 1, 9]);

        let output_sum = output.sum().into_scalar();
        let expected_sum = -4.568_224; // from pytorch

        assert!(expected_sum.approx_eq(output_sum, (1.0e-4, 2)));
    }

    #[test]
    fn resize_with_scales_2d_bicubic() {
        // Initialize the model without weights (because the exported file does not contain them)
        let device = Default::default();
        let model: resize_2d_bicubic_scale::Model<Backend> =
            resize_2d_bicubic_scale::Model::<Backend>::new(&device);

        // Run the model
        let input = Tensor::<Backend, 4>::from_floats(
            [[[
                [-1.1258, -1.1524, -0.2506, -0.4339, 0.8487, 0.6920],
                [-0.3160, -2.1152, 0.3223, -1.2633, 0.3500, 0.3081],
                [0.1198, 1.2377, 1.1168, -0.2473, -1.3527, -1.6959],
                [0.5667, 0.7935, 0.4397, 0.1124, 0.6408, 0.4412],
                [-0.2159, -0.7425, 0.5627, 0.2596, 0.5229, 2.3022],
                [-1.4689, -1.5867, 1.2032, 0.0845, -1.2001, -0.0048],
            ]]],
            &device,
        );

        // The scales are 1.5, 1.5
        let output = model.forward(input);

        assert_eq!(output.dims(), [1, 1, 9, 9]);

        let output_sum = output.sum().into_scalar();

        let expected_sum = -3.515_921; // from pytorch

        assert!(expected_sum.approx_eq(output_sum, (1.0e-3, 2)));
    }
}
