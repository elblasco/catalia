use super::super::enc::*;
use crate::common::{Cex as Model, *};
use crate::info::VarInfo;

pub struct LinearApprox {
    /// Existing approx
    approx: Approx,
    // approx template
    // n_args * num of its approx
    coef: VarMap<VarMap<VarIdx>>,
    cnst: VarIdx,
    min: Option<i64>,
    max: Option<i64>,
}

impl Approximation for LinearApprox {
    fn apply(&self, arg_terms: &[Term]) -> Vec<Term> {
        let mut terms = self.approx.apply(arg_terms);
        let mut res = vec![term::var(self.cnst, typ::int())];
        let coefs = self.coef.iter().flatten();
        for (arg, coef) in arg_terms.iter().zip(coefs) {
            let t = term::mul(vec![term::var(*coef, typ::int()), arg.clone()]);
            res.push(t);
        }
        terms.push(term::add(res));
        terms
    }
}

impl LinearApprox {
    pub(crate) fn constraint(&self) -> Option<Term> {
        let mut asserts = Vec::new();
        for c in self
            .coef
            .iter()
            .flatten()
            .chain(std::iter::once(&self.cnst))
        {
            if let Some(min) = self.min {
                let t = term::le(term::int(min), term::var(*c, typ::int()));
                asserts.push(t);
            }

            if let Some(max) = self.max {
                let t = term::le(term::var(*c, typ::int()), term::int(max));
                asserts.push(t);
            }
        }

        Some(term::and(asserts))
    }

    pub(crate) fn instantiate(&self, model: &Model) -> Approx {
        let mut approx = self.approx.clone();
        let cnst = &model[self.cnst];

        let mut terms = vec![term::val(cnst.clone())];
        //let mut constructor_args: Vec<usize> = Vec::with_capacity(approx.args.len());

        for (coef, arg) in self.coef.iter().flatten().zip(approx.args.iter()) {
            let val = &model[*coef];
            let val = term::val(val.clone());
            let var = term::var(arg.idx, arg.typ.clone());
            terms.push(term::mul(vec![val, var]));
        }
        let term = term::add(terms);
        approx.terms.push(term);

        approx
    }

    pub(crate) fn new(
        coef: VarMap<VarMap<VarIdx>>,
        fvs: &mut VarInfos,
        approx: Approx,
        min: Option<i64>,
        max: Option<i64>,
    ) -> Self {
        let idx = fvs.next_index();
        let info = VarInfo::new("const_value".to_string(), typ::int(), idx);
        fvs.push(info);
        Self {
            coef,
            cnst: idx,
            approx,
            min,
            max,
        }
    }
}
