use crate::*;

fn reduction_re() -> RecExpr2<RiseENode> {
    let comp = 0;
    let add1 = 1;
    let y = 2;
    let f = 3;
    let g = 4;
    let x = 5;

    let comp_re = lam_re(f,
                    lam_re(g,
                        lam_re(x,
                            app_re(var_re(f),
                                app_re(
                                    var_re(g),
                                    var_re(x)
                                )
                            )
                        )
                    )
                );

    let add1_re = lam_re(y, add_re(var_re(y), num_re(1)));
    let mut it = var_re(add1);
    for _ in 0..6 {
        it = app_re(app_re(var_re(comp), var_re(add1)), it);
    }

    app_re(lam_re(comp,
            app_re(lam_re(add1, it),
                add1_re,
            )
        ),
        comp_re
    )
}

#[test]
fn test_reduction() {
    let mut eg = EGraph::new();
    let i = add_rec_expr2(&reduction_re(), &mut eg);
    for _ in 0..30 {
        rewrite_rise(&mut eg);
    }
    let out = extract::<_, AstSizeNoLet>(i.id, &eg);
    assert!(out.node_dag.len() == 16);
}

fn fission_re1() -> RecExpr2<RiseENode> {
    let map = symb_re("map");
    let x = 0;
    let mut it = var_re(x);
    for i in 1..=5 {
        let s = symb_re(&format!("f{}", i));
        it = app_re(s, it);
    }
    app_re(map, lam_re(x, it))
}