// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! SVG names supported by Color.
//!
//! See: <https://www.w3.org/TR/SVG11/types.html#ColorKeywords>

#![allow(non_upper_case_globals)]

use lazy_static::lazy_static;
use std::collections::BTreeMap;

use super::color::Color;

pub const aliceblue: Color = Color::from_rgb(0xf0, 0xf8, 0xff);
pub const antiquewhite: Color = Color::from_rgb(0xfa, 0xeb, 0xd7);
pub const aqua: Color = Color::from_rgb(0x00, 0xff, 0xff);
pub const aquamarine: Color = Color::from_rgb(0x7f, 0xff, 0xd4);
pub const azure: Color = Color::from_rgb(0xf0, 0xff, 0xff);
pub const beige: Color = Color::from_rgb(0xf5, 0xf5, 0xdc);
pub const bisque: Color = Color::from_rgb(0xff, 0xe4, 0xc4);
pub const black: Color = Color::from_rgb(0x00, 0x00, 0x00);
pub const blanchedalmond: Color = Color::from_rgb(0xff, 0xeb, 0xcd);
pub const blue: Color = Color::from_rgb(0x00, 0x00, 0xff);
pub const blueviolet: Color = Color::from_rgb(0x8a, 0x2b, 0xe2);
pub const brown: Color = Color::from_rgb(0xa5, 0x2a, 0x2a);
pub const burlywood: Color = Color::from_rgb(0xde, 0xb8, 0x87);
pub const cadetblue: Color = Color::from_rgb(0x5f, 0x9e, 0xa0);
pub const chartreuse: Color = Color::from_rgb(0x7f, 0xff, 0x00);
pub const chocolate: Color = Color::from_rgb(0xd2, 0x69, 0x1e);
pub const coral: Color = Color::from_rgb(0xff, 0x7f, 0x50);
pub const cornflowerblue: Color = Color::from_rgb(0x64, 0x95, 0xed);
pub const cornsilk: Color = Color::from_rgb(0xff, 0xf8, 0xdc);
pub const crimson: Color = Color::from_rgb(0xdc, 0x14, 0x3c);
pub const cyan: Color = Color::from_rgb(0x00, 0xff, 0xff);
pub const darkblue: Color = Color::from_rgb(0x00, 0x00, 0x8b);
pub const darkcyan: Color = Color::from_rgb(0x00, 0x8b, 0x8b);
pub const darkgoldenrod: Color = Color::from_rgb(0xb8, 0x86, 0x0b);
pub const darkgray: Color = Color::from_rgb(0xa9, 0xa9, 0xa9);
pub const darkgreen: Color = Color::from_rgb(0x00, 0x64, 0x00);
pub const darkgrey: Color = Color::from_rgb(0xa9, 0xa9, 0xa9);
pub const darkkhaki: Color = Color::from_rgb(0xbd, 0xb7, 0x6b);
pub const darkmagenta: Color = Color::from_rgb(0x8b, 0x00, 0x8b);
pub const darkolivegreen: Color = Color::from_rgb(0x55, 0x6b, 0x2f);
pub const darkorange: Color = Color::from_rgb(0xff, 0x8c, 0x00);
pub const darkorchid: Color = Color::from_rgb(0x99, 0x32, 0xcc);
pub const darkred: Color = Color::from_rgb(0x8b, 0x00, 0x00);
pub const darksalmon: Color = Color::from_rgb(0xe9, 0x96, 0x7a);
pub const darkseagreen: Color = Color::from_rgb(0x8f, 0xbc, 0x8f);
pub const darkslateblue: Color = Color::from_rgb(0x48, 0x3d, 0x8b);
pub const darkslategray: Color = Color::from_rgb(0x2f, 0x4f, 0x4f);
pub const darkslategrey: Color = Color::from_rgb(0x2f, 0x4f, 0x4f);
pub const darkturquoise: Color = Color::from_rgb(0x00, 0xce, 0xd1);
pub const darkviolet: Color = Color::from_rgb(0x94, 0x00, 0xd3);
pub const deeppink: Color = Color::from_rgb(0xff, 0x14, 0x93);
pub const deepskyblue: Color = Color::from_rgb(0x00, 0xbf, 0xff);
pub const dimgray: Color = Color::from_rgb(0x69, 0x69, 0x69);
pub const dimgrey: Color = Color::from_rgb(0x69, 0x69, 0x69);
pub const dodgerblue: Color = Color::from_rgb(0x1e, 0x90, 0xff);
pub const firebrick: Color = Color::from_rgb(0xb2, 0x22, 0x22);
pub const floralwhite: Color = Color::from_rgb(0xff, 0xfa, 0xf0);
pub const forestgreen: Color = Color::from_rgb(0x22, 0x8b, 0x22);
pub const fuchsia: Color = Color::from_rgb(0xff, 0x00, 0xff);
pub const gainsboro: Color = Color::from_rgb(0xdc, 0xdc, 0xdc);
pub const ghostwhite: Color = Color::from_rgb(0xf8, 0xf8, 0xff);
pub const gold: Color = Color::from_rgb(0xff, 0xd7, 0x00);
pub const goldenrod: Color = Color::from_rgb(0xda, 0xa5, 0x20);
pub const gray: Color = Color::from_rgb(0x80, 0x80, 0x80);
pub const green: Color = Color::from_rgb(0x00, 0x80, 0x00);
pub const greenyellow: Color = Color::from_rgb(0xad, 0xff, 0x2f);
pub const grey: Color = Color::from_rgb(0x80, 0x80, 0x80);
pub const honeydew: Color = Color::from_rgb(0xf0, 0xff, 0xf0);
pub const hotpink: Color = Color::from_rgb(0xff, 0x69, 0xb4);
pub const indianred: Color = Color::from_rgb(0xcd, 0x5c, 0x5c);
pub const indigo: Color = Color::from_rgb(0x4b, 0x00, 0x82);
pub const ivory: Color = Color::from_rgb(0xff, 0xff, 0xf0);
pub const khaki: Color = Color::from_rgb(0xf0, 0xe6, 0x8c);
pub const lavender: Color = Color::from_rgb(0xe6, 0xe6, 0xfa);
pub const lavenderblush: Color = Color::from_rgb(0xff, 0xf0, 0xf5);
pub const lawngreen: Color = Color::from_rgb(0x7c, 0xfc, 0x00);
pub const lemonchiffon: Color = Color::from_rgb(0xff, 0xfa, 0xcd);
pub const lightblue: Color = Color::from_rgb(0xad, 0xd8, 0xe6);
pub const lightcoral: Color = Color::from_rgb(0xf0, 0x80, 0x80);
pub const lightcyan: Color = Color::from_rgb(0xe0, 0xff, 0xff);
pub const lightgoldenrodyellow: Color = Color::from_rgb(0xfa, 0xfa, 0xd2);
pub const lightgray: Color = Color::from_rgb(0xd3, 0xd3, 0xd3);
pub const lightgreen: Color = Color::from_rgb(0x90, 0xee, 0x90);
pub const lightgrey: Color = Color::from_rgb(0xd3, 0xd3, 0xd3);
pub const lightpink: Color = Color::from_rgb(0xff, 0xb6, 0xc1);
pub const lightsalmon: Color = Color::from_rgb(0xff, 0xa0, 0x7a);
pub const lightseagreen: Color = Color::from_rgb(0x20, 0xb2, 0xaa);
pub const lightskyblue: Color = Color::from_rgb(0x87, 0xce, 0xfa);
pub const lightslategray: Color = Color::from_rgb(0x77, 0x88, 0x99);
pub const lightslategrey: Color = Color::from_rgb(0x77, 0x88, 0x99);
pub const lightsteelblue: Color = Color::from_rgb(0xb0, 0xc4, 0xde);
pub const lightyellow: Color = Color::from_rgb(0xff, 0xff, 0xe0);
pub const lime: Color = Color::from_rgb(0x00, 0xff, 0x00);
pub const limegreen: Color = Color::from_rgb(0x32, 0xcd, 0x32);
pub const linen: Color = Color::from_rgb(0xfa, 0xf0, 0xe6);
pub const magenta: Color = Color::from_rgb(0xff, 0x00, 0xff);
pub const maroon: Color = Color::from_rgb(0x80, 0x00, 0x00);
pub const mediumaquamarine: Color = Color::from_rgb(0x66, 0xcd, 0xaa);
pub const mediumblue: Color = Color::from_rgb(0x00, 0x00, 0xcd);
pub const mediumorchid: Color = Color::from_rgb(0xba, 0x55, 0xd3);
pub const mediumpurple: Color = Color::from_rgb(0x93, 0x70, 0xdb);
pub const mediumseagreen: Color = Color::from_rgb(0x3c, 0xb3, 0x71);
pub const mediumslateblue: Color = Color::from_rgb(0x7b, 0x68, 0xee);
pub const mediumspringgreen: Color = Color::from_rgb(0x00, 0xfa, 0x9a);
pub const mediumturquoise: Color = Color::from_rgb(0x48, 0xd1, 0xcc);
pub const mediumvioletred: Color = Color::from_rgb(0xc7, 0x15, 0x85);
pub const midnightblue: Color = Color::from_rgb(0x19, 0x19, 0x70);
pub const mintcream: Color = Color::from_rgb(0xf5, 0xff, 0xfa);
pub const mistyrose: Color = Color::from_rgb(0xff, 0xe4, 0xe1);
pub const moccasin: Color = Color::from_rgb(0xff, 0xe4, 0xb5);
pub const navajowhite: Color = Color::from_rgb(0xff, 0xde, 0xad);
pub const navy: Color = Color::from_rgb(0x00, 0x00, 0x80);
pub const oldlace: Color = Color::from_rgb(0xfd, 0xf5, 0xe6);
pub const olive: Color = Color::from_rgb(0x80, 0x80, 0x00);
pub const olivedrab: Color = Color::from_rgb(0x6b, 0x8e, 0x23);
pub const orange: Color = Color::from_rgb(0xff, 0xa5, 0x00);
pub const orangered: Color = Color::from_rgb(0xff, 0x45, 0x00);
pub const orchid: Color = Color::from_rgb(0xda, 0x70, 0xd6);
pub const palegoldenrod: Color = Color::from_rgb(0xee, 0xe8, 0xaa);
pub const palegreen: Color = Color::from_rgb(0x98, 0xfb, 0x98);
pub const paleturquoise: Color = Color::from_rgb(0xaf, 0xee, 0xee);
pub const palevioletred: Color = Color::from_rgb(0xdb, 0x70, 0x93);
pub const papayawhip: Color = Color::from_rgb(0xff, 0xef, 0xd5);
pub const peachpuff: Color = Color::from_rgb(0xff, 0xda, 0xb9);
pub const peru: Color = Color::from_rgb(0xcd, 0x85, 0x3f);
pub const pink: Color = Color::from_rgb(0xff, 0xc0, 0xcb);
pub const plum: Color = Color::from_rgb(0xdd, 0xa0, 0xdd);
pub const powderblue: Color = Color::from_rgb(0xb0, 0xe0, 0xe6);
pub const purple: Color = Color::from_rgb(0x80, 0x00, 0x80);
pub const red: Color = Color::from_rgb(0xff, 0x00, 0x00);
pub const rosybrown: Color = Color::from_rgb(0xbc, 0x8f, 0x8f);
pub const royalblue: Color = Color::from_rgb(0x41, 0x69, 0xe1);
pub const saddlebrown: Color = Color::from_rgb(0x8b, 0x45, 0x13);
pub const salmon: Color = Color::from_rgb(0xfa, 0x80, 0x72);
pub const sandybrown: Color = Color::from_rgb(0xf4, 0xa4, 0x60);
pub const seagreen: Color = Color::from_rgb(0x2e, 0x8b, 0x57);
pub const seashell: Color = Color::from_rgb(0xff, 0xf5, 0xee);
pub const sienna: Color = Color::from_rgb(0xa0, 0x52, 0x2d);
pub const silver: Color = Color::from_rgb(0xc0, 0xc0, 0xc0);
pub const skyblue: Color = Color::from_rgb(0x87, 0xce, 0xeb);
pub const slateblue: Color = Color::from_rgb(0x6a, 0x5a, 0xcd);
pub const slategray: Color = Color::from_rgb(0x70, 0x80, 0x90);
pub const slategrey: Color = Color::from_rgb(0x70, 0x80, 0x90);
pub const snow: Color = Color::from_rgb(0xff, 0xfa, 0xfa);
pub const springgreen: Color = Color::from_rgb(0x00, 0xff, 0x7f);
pub const steelblue: Color = Color::from_rgb(0x46, 0x82, 0xb4);
pub const tan: Color = Color::from_rgb(0xd2, 0xb4, 0x8c);
pub const teal: Color = Color::from_rgb(0x00, 0x80, 0x80);
pub const thistle: Color = Color::from_rgb(0xd8, 0xbf, 0xd8);
pub const tomato: Color = Color::from_rgb(0xff, 0x63, 0x47);
pub const turquoise: Color = Color::from_rgb(0x40, 0xe0, 0xd0);
pub const violet: Color = Color::from_rgb(0xee, 0x82, 0xee);
pub const wheat: Color = Color::from_rgb(0xf5, 0xde, 0xb3);
pub const white: Color = Color::from_rgb(0xff, 0xff, 0xff);
pub const whitesmoke: Color = Color::from_rgb(0xf5, 0xf5, 0xf5);
pub const yellow: Color = Color::from_rgb(0xff, 0xff, 0x00);
pub const yellowgreen: Color = Color::from_rgb(0x9a, 0xcd, 0x32);

lazy_static! {
    pub static ref COLOR_TABLE: BTreeMap<&'static str, &'static Color> = [
        ("aliceblue", &aliceblue),
        ("antiquewhite", &antiquewhite),
        ("aqua", &aqua),
        ("aquamarine", &aquamarine),
        ("azure", &azure),
        ("beige", &beige),
        ("bisque", &bisque),
        ("black", &black),
        ("blanchedalmond", &blanchedalmond),
        ("blue", &blue),
        ("blueviolet", &blueviolet),
        ("brown", &brown),
        ("burlywood", &burlywood),
        ("cadetblue", &cadetblue),
        ("chartreuse", &chartreuse),
        ("chocolate", &chocolate),
        ("coral", &coral),
        ("cornflowerblue", &cornflowerblue),
        ("cornsilk", &cornsilk),
        ("crimson", &crimson),
        ("cyan", &cyan),
        ("darkblue", &darkblue),
        ("darkcyan", &darkcyan),
        ("darkgoldenrod", &darkgoldenrod),
        ("darkgray", &darkgray),
        ("darkgreen", &darkgreen),
        ("darkgrey", &darkgrey),
        ("darkkhaki", &darkkhaki),
        ("darkmagenta", &darkmagenta),
        ("darkolivegreen", &darkolivegreen),
        ("darkorange", &darkorange),
        ("darkorchid", &darkorchid),
        ("darkred", &darkred),
        ("darksalmon", &darksalmon),
        ("darkseagreen", &darkseagreen),
        ("darkslateblue", &darkslateblue),
        ("darkslategray", &darkslategray),
        ("darkslategrey", &darkslategrey),
        ("darkturquoise", &darkturquoise),
        ("darkviolet", &darkviolet),
        ("deeppink", &deeppink),
        ("deepskyblue", &deepskyblue),
        ("dimgray", &dimgray),
        ("dimgrey", &dimgrey),
        ("dodgerblue", &dodgerblue),
        ("firebrick", &firebrick),
        ("floralwhite", &floralwhite),
        ("forestgreen", &forestgreen),
        ("fuchsia", &fuchsia),
        ("gainsboro", &gainsboro),
        ("ghostwhite", &ghostwhite),
        ("gold", &gold),
        ("goldenrod", &goldenrod),
        ("gray", &gray),
        ("green", &green),
        ("greenyellow", &greenyellow),
        ("grey", &grey),
        ("honeydew", &honeydew),
        ("hotpink", &hotpink),
        ("indianred", &indianred),
        ("indigo", &indigo),
        ("ivory", &ivory),
        ("khaki", &khaki),
        ("lavender", &lavender),
        ("lavenderblush", &lavenderblush),
        ("lawngreen", &lawngreen),
        ("lemonchiffon", &lemonchiffon),
        ("lightblue", &lightblue),
        ("lightcoral", &lightcoral),
        ("lightcyan", &lightcyan),
        ("lightgoldenrodyellow", &lightgoldenrodyellow),
        ("lightgray", &lightgray),
        ("lightgreen", &lightgreen),
        ("lightgrey", &lightgrey),
        ("lightpink", &lightpink),
        ("lightsalmon", &lightsalmon),
        ("lightseagreen", &lightseagreen),
        ("lightskyblue", &lightskyblue),
        ("lightslategray", &lightslategray),
        ("lightslategrey", &lightslategrey),
        ("lightsteelblue", &lightsteelblue),
        ("lightyellow", &lightyellow),
        ("lime", &lime),
        ("limegreen", &limegreen),
        ("linen", &linen),
        ("magenta", &magenta),
        ("maroon", &maroon),
        ("mediumaquamarine", &mediumaquamarine),
        ("mediumblue", &mediumblue),
        ("mediumorchid", &mediumorchid),
        ("mediumpurple", &mediumpurple),
        ("mediumseagreen", &mediumseagreen),
        ("mediumslateblue", &mediumslateblue),
        ("mediumspringgreen", &mediumspringgreen),
        ("mediumturquoise", &mediumturquoise),
        ("mediumvioletred", &mediumvioletred),
        ("midnightblue", &midnightblue),
        ("mintcream", &mintcream),
        ("mistyrose", &mistyrose),
        ("moccasin", &moccasin),
        ("navajowhite", &navajowhite),
        ("navy", &navy),
        ("oldlace", &oldlace),
        ("olive", &olive),
        ("olivedrab", &olivedrab),
        ("orange", &orange),
        ("orangered", &orangered),
        ("orchid", &orchid),
        ("palegoldenrod", &palegoldenrod),
        ("palegreen", &palegreen),
        ("paleturquoise", &paleturquoise),
        ("palevioletred", &palevioletred),
        ("papayawhip", &papayawhip),
        ("peachpuff", &peachpuff),
        ("peru", &peru),
        ("pink", &pink),
        ("plum", &plum),
        ("powderblue", &powderblue),
        ("purple", &purple),
        ("red", &red),
        ("rosybrown", &rosybrown),
        ("royalblue", &royalblue),
        ("saddlebrown", &saddlebrown),
        ("salmon", &salmon),
        ("sandybrown", &sandybrown),
        ("seagreen", &seagreen),
        ("seashell", &seashell),
        ("sienna", &sienna),
        ("silver", &silver),
        ("skyblue", &skyblue),
        ("slateblue", &slateblue),
        ("slategray", &slategray),
        ("slategrey", &slategrey),
        ("snow", &snow),
        ("springgreen", &springgreen),
        ("steelblue", &steelblue),
        ("tan", &tan),
        ("teal", &teal),
        ("thistle", &thistle),
        ("tomato", &tomato),
        ("turquoise", &turquoise),
        ("violet", &violet),
        ("wheat", &wheat),
        ("white", &white),
        ("whitesmoke", &whitesmoke),
        ("yellow", &yellow),
        ("yellowgreen", &yellowgreen),
    ]
    .iter()
    .copied()
    .collect();
}
