use colored::CustomColor;

pub const PAST_DUE_COLOR: CustomColor = CustomColor {
    r: 122,
    g: 136,
    b: 164,
};
pub const DUE_THREE_DAYS: CustomColor = CustomColor { r: 255, g: 0, b: 0 };
pub const DUE_ONE_WEEK: CustomColor = CustomColor {
    r: 255,
    g: 60,
    b: 60,
};
pub const DUE_TWO_WEEKS: CustomColor = CustomColor {
    r: 245,
    g: 165,
    b: 0,
};
pub const DUE_FOUR_WEEKS: CustomColor = CustomColor {
    r: 254,
    g: 225,
    b: 65,
};
// pub const DUE_ONE_MONTH: CustomColor = CustomColor { r: 0, g: 0, b: 255 };
// greater than one month is normal
// pub const DUE_OTHER: CustomColor = CustomColor { r: 0, g: 0, b: 0 };

pub const DONE_COLOR: CustomColor = CustomColor {
    r: 100,
    g: 230,
    b: 100,
};
pub const IN_PROGRESS_COLOR: CustomColor = CustomColor {
    r: 24,
    g: 165,
    b: 230,
};
// not started is normal
// const NOT_STARTED_COLOR: CustomColor = CustomColor { r: 0, g: 0, b: 0 };
