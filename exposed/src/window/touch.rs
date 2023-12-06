/// Experimental
#[derive(Debug, Default, Clone, Copy)]
pub struct Touch {
    pub phase: TouchPhase,
    pub location: (f32, f32),
    pub pointer_index: usize,
    pub id: u64,
    pub os_data: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    #[default]
    Cancelled,
}

impl Touch {
    pub fn historical(&self) -> Option<(f32, f32)> {
        #[cfg(not(target_os = "android"))]
        return None;

        #[cfg(target_os = "android")]
        return unsafe {
            use ndk_sys::{AMotionEvent_getHistoricalX, AMotionEvent_getHistoricalY, AMotionEvent_getHistorySize};

            let h = AMotionEvent_getHistorySize(self.os_data as _);

            if h == 0 {
                return None;
            }

            let x = AMotionEvent_getHistoricalX(self.os_data as _, self.pointer_index, h - 1);
            let y = AMotionEvent_getHistoricalY(self.os_data as _, self.pointer_index, h - 1);

            Some((x, y))
        };
    }
}
