use elysian_core::{
    ast::{combine::IntoCombine, expr::IntoLiteral},
    ir::{
        ast::{DISTANCE, GRADIENT_2D, UV},
        module::DynAsIR,
    },
};
use elysian_shapes::{
    combine::{SmoothUnion, Union},
    field::{Line, LineMode},
    modify::{IntoMirror, IntoTranslate, MirrorMode},
};

pub fn combinator() -> impl IntoIterator<Item = DynAsIR> {
    [
        Box::new(Union::default()) as DynAsIR,
        Box::new(SmoothUnion {
            prop: DISTANCE.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothUnion {
            prop: GRADIENT_2D.into(),
            k: 0.4.literal(),
        }),
        Box::new(SmoothUnion {
            prop: UV.into(),
            k: 0.4.literal(),
        }),
    ]
}

pub fn a() -> DynAsIR {
    let shape_a = Line {
        dir: [0.5, 0.0].literal(),
        mode: LineMode::Segment,
    }
    .translate([0.0, -0.5].literal());

    let shape_b = Line {
        dir: [1.0, -2.25].literal(),
        mode: LineMode::Segment,
    }
    .translate([0.0, 1.0].literal());

    Box::new(
        [Box::new(shape_a) as DynAsIR, Box::new(shape_b)]
            .combine(combinator())
            .mirror(MirrorMode::Basis([1.0, 0.0].literal())),
    )
}

pub fn b() -> DynAsIR {
    todo!()
}

pub fn c() -> DynAsIR {
    todo!()
}

pub fn d() -> DynAsIR {
    todo!()
}

pub fn e() -> DynAsIR {
    let shape_a = Line {
        dir: [0.0, 0.8].literal(),
        mode: LineMode::Segment,
    }
    .translate([-1.0, 0.0].literal());

    let shape_b = Line {
        dir: [0.8, 0.0].literal(),
        mode: LineMode::Segment,
    }
    .translate([-0.8, 0.0].literal());

    let shape_c = Line {
        dir: [0.8, 0.0].literal(),
        mode: LineMode::Centered,
    }
    .translate([0.0, 1.0].literal());

    Box::new(
        [
            Box::new(shape_a) as DynAsIR,
            Box::new(shape_b),
            Box::new(shape_c),
        ]
        .combine(combinator())
        .mirror(MirrorMode::Basis([0.0, 1.0].literal())),
    )
}

pub fn f() -> DynAsIR {
    let shape_a = Line {
        dir: [0.0, 1.0].literal(),
        mode: LineMode::Centered,
    }
    .translate([-1.0, -0.2].literal());

    let shape_b = Line {
        dir: [0.8, 0.0].literal(),
        mode: LineMode::Segment,
    }
    .translate([-0.8, 0.0].literal());

    let shape_c = Line {
        dir: [0.8, 0.0].literal(),
        mode: LineMode::Centered,
    }
    .translate([0.0, 1.0].literal());

    Box::new(
        [
            Box::new(shape_a) as DynAsIR,
            Box::new(shape_b),
            Box::new(shape_c),
        ]
        .combine(combinator()),
    )
}

pub fn g() -> DynAsIR {
    todo!()
}

pub fn h() -> DynAsIR {
    let shape_a = Line {
        dir: [0.0, 1.0].literal(),
        mode: LineMode::Centered,
    }
    .translate([1.0, 0.0].literal());

    let shape_b = Line {
        dir: [0.8, 0.0].literal(),
        mode: LineMode::Segment,
    };

    Box::new(
        [Box::new(shape_a) as DynAsIR, Box::new(shape_b)]
            .combine(combinator())
            .mirror(MirrorMode::Basis([1.0, 0.0].literal())),
    )
}

pub fn i() -> DynAsIR {
    Box::new(Line {
        dir: [0.0, 1.0].literal(),
        mode: LineMode::Centered,
    })
}

pub fn j() -> DynAsIR {
    todo!()
}

pub fn k() -> DynAsIR {
    let shape_a = Line {
        dir: [0.0, 1.0].literal(),
        mode: LineMode::Segment,
    }
    .translate([-1.0, 0.0].literal());

    let shape_b = Line {
        dir: [1.6, 1.0].literal(),
        mode: LineMode::Segment,
    }
    .translate([-0.8, 0.0].literal());

    Box::new(
        [Box::new(shape_a) as DynAsIR, Box::new(shape_b)]
            .combine(combinator())
            .mirror(MirrorMode::Basis([0.0, 1.0].literal())),
    )
}

pub fn l() -> DynAsIR {
    let shape_a = Line {
        dir: [0.0, 1.0].literal(),
        mode: LineMode::Centered,
    }
    .translate([-1.0, 0.0].literal());

    let shape_b = Line {
        dir: [0.8, 0.0].literal(),
        mode: LineMode::Centered,
    }
    .translate([0.0, -1.2].literal());

    Box::new([Box::new(shape_a) as DynAsIR, Box::new(shape_b)].combine(combinator()))
}

pub fn m() -> DynAsIR {
    let shape_a = Line {
        dir: [0.0, 2.0].literal(),
        mode: LineMode::Segment,
    }
    .translate([1.0, -1.0].literal());

    let shape_b = Line {
        dir: [1.0, 1.0].literal(),
        mode: LineMode::Segment,
    };

    Box::new(
        [Box::new(shape_a) as DynAsIR, Box::new(shape_b)]
            .combine(combinator())
            .mirror(MirrorMode::Basis([1.0, 0.0].literal())),
    )
}

pub fn n() -> DynAsIR {
    let shape_a = Line {
        dir: [0.0, 1.0].literal(),
        mode: LineMode::Segment,
    }
    .translate([1.0, 0.0].literal())
    .mirror(MirrorMode::Basis([1.0, 1.0].literal()));

    let shape_b = Line {
        dir: [-0.8, 1.0].literal(),
        mode: LineMode::Centered,
    };

    Box::new([Box::new(shape_a) as DynAsIR, Box::new(shape_b)].combine(combinator()))
}

pub fn o() -> DynAsIR {
    todo!()
}

pub fn p() -> DynAsIR {
    todo!()
}

pub fn q() -> DynAsIR {
    todo!()
}

pub fn r() -> DynAsIR {
    todo!()
}

pub fn s() -> DynAsIR {
    todo!()
}

pub fn t() -> DynAsIR {
    let shape_a = Line {
        dir: [1.0, 0.0].literal(),
        mode: LineMode::Centered,
    };

    let shape_b = Line {
        dir: [0.0, 1.0].literal(),
        mode: LineMode::Centered,
    }
    .translate([0.0, -1.2].literal());

    Box::new(
        [Box::new(shape_a) as DynAsIR, Box::new(shape_b)]
            .combine(combinator())
            .translate([0.0, 1.0].literal()),
    )
}

pub fn u() -> DynAsIR {
    todo!()
}

pub fn v() -> DynAsIR {
    Box::new(
        Line {
            dir: [1.0, 2.0].literal(),
            mode: LineMode::Segment,
        }
        .translate([0.0, -1.0].literal())
        .mirror(MirrorMode::Basis([1.0, 0.0].literal())),
    )
}

pub fn w() -> DynAsIR {
    let shape_a = Line {
        dir: [0.5, 2.0].literal(),
        mode: LineMode::Segment,
    }
    .translate([0.5, -1.0].literal());

    let shape_b = Line {
        dir: [0.5, -0.5].literal(),
        mode: LineMode::Segment,
    };

    Box::new(
        [Box::new(shape_a) as DynAsIR, Box::new(shape_b)]
            .combine(combinator())
            .mirror(MirrorMode::Basis([1.0, 0.0].literal())),
    )
}

pub fn x() -> DynAsIR {
    Box::new(
        Line {
            dir: [0.6, 1.0].literal(),
            mode: LineMode::Segment,
        }
        .mirror(MirrorMode::Basis([1.0, 1.0].literal())),
    )
}

pub fn y() -> DynAsIR {
    let shape_a = Line {
        dir: [1.0, 1.0].literal(),
        mode: LineMode::Segment,
    };

    let shape_b = Line {
        dir: [0.0, -1.2].literal(),
        mode: LineMode::Segment,
    };

    Box::new(
        [Box::new(shape_a) as DynAsIR, Box::new(shape_b)]
            .combine(combinator())
            .mirror(MirrorMode::Basis([1.0, 0.0].literal())),
    )
}

pub fn z() -> DynAsIR {
    let shape_a = Line {
        dir: [1.0, 0.0].literal(),
        mode: LineMode::Segment,
    }
    .translate([0.0, 1.2].literal())
    .mirror(MirrorMode::Basis([1.0, 1.0].literal()));

    let shape_b = Line {
        dir: [1.0, 1.0].literal(),
        mode: LineMode::Centered,
    };

    Box::new([Box::new(shape_a) as DynAsIR, Box::new(shape_b)].combine(combinator()))
}
