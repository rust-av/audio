pub trait SampleType: Copy + Default {}

impl SampleType for u8 {}
impl SampleType for i16 {}
impl SampleType for f32 {}


