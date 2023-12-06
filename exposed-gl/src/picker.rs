use crate::{platform::GlPixelFormat, tokens, GlConfigPicker};

#[derive(Debug, Default)]
pub struct GlDefaultPicker {
    values: [i32; 3],
}

impl GlConfigPicker for GlDefaultPicker {
    fn pick(&mut self, pixel_format: GlPixelFormat) -> Option<usize> {
        #[rustfmt::skip]
        let attributes = [
            tokens::COLOR_BITS_ARB,
            tokens::SAMPLE_BUFFERS_ARB,
            tokens::SAMPLES_ARB,
        ];

        let mut values = unsafe { std::mem::zeroed() };

        pixel_format.get(&attributes, &mut values).ok()?;

        if values[2] > self.values[2] {
            self.values = values;
            return Some(pixel_format.format);
        }

        if values[0] > self.values[0] {
            self.values = values;
            return Some(pixel_format.format);
        }

        None
    }
}
