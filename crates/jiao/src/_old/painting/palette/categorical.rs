// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by LGPL-3.0 License that can be found
// in the LICENSE file.

use crate::painting::Color;

pub const ACCENT: &[Color] = &[
    Color::from_rgb(0x7f, 0xc9, 0x7f),
    Color::from_rgb(0xbe, 0xae, 0xd4),
    Color::from_rgb(0xfd, 0xc0, 0x86),
    Color::from_rgb(0xff, 0xff, 0x99),
    Color::from_rgb(0x38, 0x6c, 0xb0),
    Color::from_rgb(0xf0, 0x02, 0x7f),
    Color::from_rgb(0xbf, 0x5b, 0x17),
    Color::from_rgb(0x66, 0x66, 0x66),
];

pub const CATEGORY10: &[Color] = &[
    Color::from_rgb(0x1f, 0x77, 0xb4),
    Color::from_rgb(0xff, 0x7f, 0x0e),
    Color::from_rgb(0x2c, 0xa0, 0x2c),
    Color::from_rgb(0xd6, 0x27, 0x28),
    Color::from_rgb(0x94, 0x67, 0xbd),
    Color::from_rgb(0x8c, 0x56, 0x4b),
    Color::from_rgb(0xe3, 0x77, 0xc2),
    Color::from_rgb(0x7f, 0x7f, 0x7f),
    Color::from_rgb(0xbc, 0xbd, 0x22),
    Color::from_rgb(0x17, 0xbe, 0xcf),
];

pub const DARK2: &[Color] = &[
    Color::from_rgb(0x1b, 0x9e, 0x77),
    Color::from_rgb(0xd9, 0x5f, 0x02),
    Color::from_rgb(0x75, 0x70, 0xb3),
    Color::from_rgb(0xe7, 0x29, 0x8a),
    Color::from_rgb(0x66, 0xa6, 0x1e),
    Color::from_rgb(0xe6, 0xab, 0x02),
    Color::from_rgb(0xa6, 0x76, 0x1d),
    Color::from_rgb(0x66, 0x66, 0x66),
];

pub const PAIRED: &[Color] = &[
    Color::from_rgb(0xa6, 0xce, 0xe3),
    Color::from_rgb(0x1f, 0x78, 0xb4),
    Color::from_rgb(0xb2, 0xdf, 0x8a),
    Color::from_rgb(0x33, 0xa0, 0x2c),
    Color::from_rgb(0xfb, 0x9a, 0x99),
    Color::from_rgb(0xe3, 0x1a, 0x1c),
    Color::from_rgb(0xfd, 0xbf, 0x6f),
    Color::from_rgb(0xff, 0x7f, 0x00),
    Color::from_rgb(0xca, 0xb2, 0xd6),
    Color::from_rgb(0x6a, 0x3d, 0x9a),
    Color::from_rgb(0xff, 0xff, 0x99),
    Color::from_rgb(0xb1, 0x59, 0x28),
];

pub const PASTEL1: &[Color] = &[
    Color::from_rgb(0xfb, 0xb4, 0xae),
    Color::from_rgb(0xb3, 0xcd, 0xe3),
    Color::from_rgb(0xcc, 0xeb, 0xc5),
    Color::from_rgb(0xde, 0xcb, 0xe4),
    Color::from_rgb(0xfe, 0xd9, 0xa6),
    Color::from_rgb(0xff, 0xff, 0xcc),
    Color::from_rgb(0xe5, 0xd8, 0xbd),
    Color::from_rgb(0xfd, 0xda, 0xec),
    Color::from_rgb(0xf2, 0xf2, 0xf2),
];

pub const PASTEL2: &[Color] = &[
    Color::from_rgb(0xb3, 0xe2, 0xcd),
    Color::from_rgb(0xfd, 0xcd, 0xac),
    Color::from_rgb(0xcb, 0xd5, 0xe8),
    Color::from_rgb(0xf4, 0xca, 0xe4),
    Color::from_rgb(0xe6, 0xf5, 0xc9),
    Color::from_rgb(0xff, 0xf2, 0xae),
    Color::from_rgb(0xf1, 0xe2, 0xcc),
    Color::from_rgb(0xcc, 0xcc, 0xcc),
];

pub const SET1: &[Color] = &[
    Color::from_rgb(0xe4, 0x1a, 0x1c),
    Color::from_rgb(0x37, 0x7e, 0xb8),
    Color::from_rgb(0x4d, 0xaf, 0x4a),
    Color::from_rgb(0x98, 0x4e, 0xa3),
    Color::from_rgb(0xff, 0x7f, 0x00),
    Color::from_rgb(0xff, 0xff, 0x33),
    Color::from_rgb(0xa6, 0x56, 0x28),
    Color::from_rgb(0xf7, 0x81, 0xbf),
    Color::from_rgb(0x99, 0x99, 0x99),
];

pub const SET2: &[Color] = &[
    Color::from_rgb(0x66, 0xc2, 0xa5),
    Color::from_rgb(0xfc, 0x8d, 0x62),
    Color::from_rgb(0x8d, 0xa0, 0xcb),
    Color::from_rgb(0xe7, 0x8a, 0xc3),
    Color::from_rgb(0xa6, 0xd8, 0x54),
    Color::from_rgb(0xff, 0xd9, 0x2f),
    Color::from_rgb(0xe5, 0xc4, 0x94),
    Color::from_rgb(0xb3, 0xb3, 0xb3),
];

pub const SET3: &[Color] = &[
    Color::from_rgb(0x8d, 0xd3, 0xc7),
    Color::from_rgb(0xff, 0xff, 0xb3),
    Color::from_rgb(0xbe, 0xba, 0xda),
    Color::from_rgb(0xfb, 0x80, 0x72),
    Color::from_rgb(0x80, 0xb1, 0xd3),
    Color::from_rgb(0xfd, 0xb4, 0x62),
    Color::from_rgb(0xb3, 0xde, 0x69),
    Color::from_rgb(0xfc, 0xcd, 0xe5),
    Color::from_rgb(0xd9, 0xd9, 0xd9),
    Color::from_rgb(0xbc, 0x80, 0xbd),
    Color::from_rgb(0xcc, 0xeb, 0xc5),
    Color::from_rgb(0xff, 0xed, 0x6f),
];

pub const TABLEAU10: &[Color] = &[
    Color::from_rgb(0x4e, 0x79, 0xa7),
    Color::from_rgb(0xf2, 0x8e, 0x2c),
    Color::from_rgb(0xe1, 0x57, 0x59),
    Color::from_rgb(0x76, 0xb7, 0xb2),
    Color::from_rgb(0x59, 0xa1, 0x4f),
    Color::from_rgb(0xed, 0xc9, 0x49),
    Color::from_rgb(0xaf, 0x7a, 0xa1),
    Color::from_rgb(0xff, 0x9d, 0xa7),
    Color::from_rgb(0x9c, 0x75, 0x5f),
    Color::from_rgb(0xba, 0xb0, 0xab),
];
